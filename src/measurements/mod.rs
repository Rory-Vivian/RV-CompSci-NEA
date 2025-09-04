use std::ptr::dangling;
use macroquad::time::get_frame_time;

pub static METER: f32 = 20.0;
pub(crate) fn dt() -> f32 {
    get_frame_time()
}

pub fn get_gravity() -> f32 {
    mps(9.81)
}

pub fn mps(meters: f32) -> f32 {
    meter(meters) * dt()
}


pub fn meter(meters: f32) -> f32 {
    meters * METER
}