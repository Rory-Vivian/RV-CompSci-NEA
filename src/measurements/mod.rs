use macroquad::time::get_frame_time;
use macroquad::math::Vec2;

pub(crate) fn dt() -> f32 {
    get_frame_time()
}


pub fn meter(meters: f32) -> f32 {
    meters * 100.0
}

pub fn vec2_meter(input: Vec2) -> Vec2 {
    Vec2::new(input.x/meter(1.), input.y/meter(1.))
}