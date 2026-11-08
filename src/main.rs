mod body;
mod physics;
mod units;

use macroquad::prelude::*;

use body::Body;
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
        },
        // Body {
        //     name: "Alpha Centauri B".to_string(),
        //     mass: 2.146e30, // 1.079 solar masses
        //     radius: 8.5112e8,
        //     color: GREEN,
        //     pos: pixel_2_m(dvec2(200.0, 500.0)),
        //     vel: DVec2::ZERO,
        //     force: DVec2::ZERO,
        //     acc: DVec2::ZERO,
        // }
    ];

    // Sim and viz update loop
    loop {
        clear_background(BLACK);
        
        // todo: Step Physics
        calculate_net_gravity(&mut bodies);
        euler_forward_integration(&mut bodies);
        // Visual test: draw bodies using PX2M and RAD2PX
        for (i, body) in bodies.iter().enumerate() {
            let p = m_2_pixel(body.pos);
            let r_px = rad_2_pixel(body.radius);
            draw_circle(p.x as f32, p.y as f32, r_px as f32, body.color);
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
