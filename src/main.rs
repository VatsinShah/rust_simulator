mod body;
mod physics;
mod units;

use std::collections::VecDeque;

use macroquad::prelude::*;

use body::{Body, TAIL_LEN, TAIL_FALLOFF};
use physics::{calculate_net_gravity, euler_forward_integration};
use units::{m_2_pixel, pixel_2_m, rad_2_pixel};

#[macroquad::main("Gravity Simulator")]
async fn main() {
    let mut bodies = vec![
        Body {
            name: "Sun".to_string(),
            mass: 1.989e30,
            radius: 6.957e8,
            color: YELLOW,
            pos: pixel_2_m(dvec2(100.0, 100.0)),
            vel: dvec2(3000.0, -1000.0),
            force: DVec2::ZERO,
            acc: DVec2::ZERO,
            tail_buffer: VecDeque::with_capacity(TAIL_LEN),
        },
        Body {
            name: "Alpha Centauri A".to_string(),
            mass: 2.146e30, // 1.079 solar masses
            radius: 8.5112e8,
            color: BLUE,
            pos: pixel_2_m(dvec2(400.0, 400.0)),
            vel: dvec2(-1000.0, 2000.0),
            force: DVec2::ZERO,
            acc: DVec2::ZERO,
            tail_buffer: VecDeque::with_capacity(TAIL_LEN),
        },
        Body {
            name: "Alpha Centauri B".to_string(),
            mass: 2.146e30, // 1.079 solar masses
            radius: 8.5112e8,
            color: GREEN,
            pos: pixel_2_m(dvec2(650.0, 100.0)),
            vel: dvec2(-4000.0, 1000.0),
            force: DVec2::ZERO,
            acc: DVec2::ZERO,
            tail_buffer: VecDeque::with_capacity(TAIL_LEN),
        }
    ];

    // Sim and viz update loop
    loop {
        // Prepare for drawing
        clear_background(BLACK);
        
        // Update Physics
        calculate_net_gravity(&mut bodies);
        euler_forward_integration(&mut bodies);
        
        // Draw bodies and tails
        for (i, body) in bodies.iter().enumerate() {
            let p = m_2_pixel(body.pos);
            let r_px = rad_2_pixel(body.radius);
            
            // Draw Tail First
            for k in 0..body.tail_buffer.len() {
                // Total number of points in tail
                let n = TAIL_LEN;
                
                // Define the position along the tail
                let t = k as f32 / (n - 1) as f32;
                // Define the end offset
                let end = (-TAIL_FALLOFF).exp();
                // Calculate the opacity gradient based on exponential falloff and limit to [0,1]
                let opacity = ((-TAIL_FALLOFF * t).exp() - end) / (1.0 - end);
                draw_circle(body.tail_buffer[k][0] as f32, body.tail_buffer[k][1] as f32, r_px as f32, body.color.with_alpha(opacity));
            }

            // Draw Body
            draw_circle(p.x as f32, p.y as f32, r_px as f32, body.color);
            
            // Draw Text
            draw_text(
                &format!("{}: r={r_px:.2}px", body.name),
                10.0,
                screen_height() - 20.0 - 20.0 * i as f32,
                20.0,
                body.color,
            );
            
        }
        next_frame().await;
    }
}
