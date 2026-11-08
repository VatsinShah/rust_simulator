//! Physical constants and conversions between simulation units (SI) and screen pixels.

use macroquad::prelude::DVec2;

pub const AU: f64 = 1.495_978_707e11; // meters in 1 au
pub const PX2M: f64 = 0.1 * AU; // pixel_val * px2m = meters -> 1au = 10px
pub const RAD2PX: f64 = 10.0 / 6.957e8; // radius_in_m * rad2px = radius_in_px -> 10 px = 695,700km (radius of sun)

pub fn pixel_2_m(px_pos: DVec2) -> DVec2 {
    px_pos * PX2M
}

pub fn m_2_pixel(m_pos: DVec2) -> DVec2 {
    m_pos / PX2M
}

pub fn rad_2_pixel(rad_m: f64) -> f64 {
    rad_m * RAD2PX
}

pub fn pixel_2_rad(rad_px: f64) -> f64 {
    rad_px / RAD2PX
}
