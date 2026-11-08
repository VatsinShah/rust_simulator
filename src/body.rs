//! The `Body` type: a single star in the simulation.

use macroquad::prelude::{Color, DVec2};

// Rust has no per-field mutability: `let mut body` makes every field mutable.
// By convention we only modify `pos`, `vel`, `force` and `acc` after creation.
pub struct Body {
    pub name: String,
    pub mass: f64,   // kg
    pub radius: f64, // m
    pub color: Color,
    pub pos: DVec2, // position in universe coordinate frame (m)
    pub vel: DVec2, // m/s
    pub force: DVec2, // net gravitational force on this body (N)
    pub acc: DVec2,   // m/s^2
}
