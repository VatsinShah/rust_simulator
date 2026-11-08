//! Gravity and (soon) integration.

use macroquad::prelude::DVec2;

use crate::body::Body;
use crate::body::TAIL_LEN;
use crate::units::m_2_pixel;

const G: f64 = 6.6743e-11;
const DT: f64 = 604800.0; // 100hz
// Function to calculate net gravitational forces
pub fn calculate_net_gravity(bodies: &mut [Body]) {
    for i in 0..bodies.len() {
        let mut net_force = DVec2::ZERO;
        for j in 0..bodies.len() {
            if i == j {continue};

            let dist_vec = bodies[j].pos - bodies[i].pos;
            let dist = dist_vec.length();
            let dir = dist_vec.normalize();

            // todo: Gravity vector calculation now that distance and direction are verified
            let magnitude = G * bodies[i].mass * bodies[j].mass / (dist * dist);
            net_force+=magnitude*dir;
        }
        bodies[i].force = net_force;
        bodies[i].acc = bodies[i].force/bodies[i].mass;
    }
}

pub fn euler_forward_integration(bodies: &mut [Body]) {
    for i in 0..bodies.len() {
        // Update Tail Buffer
        bodies[i].tail_buffer.push_front(m_2_pixel(bodies[i].pos));
        if bodies[i].tail_buffer.len() > TAIL_LEN {
            bodies[i].tail_buffer.pop_back();
        }
        
        // Euler integration
        bodies[i].vel+=bodies[i].acc*DT;
        bodies[i].pos+=bodies[i].vel*DT;
    }
}