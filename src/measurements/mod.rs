use std::ptr::dangling;
use std::sync::atomic::Ordering;
use macroquad::time::get_frame_time;
use crate::ZOOM;

pub(crate) fn dt() -> f32 {
    get_frame_time()
}

pub fn get_gravity() -> f32 {
    9.81*dt()
}


pub fn meter(meters: f32) -> f32 {
    let zoom = ZOOM.load(Ordering::SeqCst) as f32;
    meters * 100.0 * ((zoom / 100.) + (50./100.))
}