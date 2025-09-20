use macroquad::time::get_frame_time;
pub(crate) fn dt() -> f32 {
    get_frame_time()
}

pub fn get_gravity() -> f32 {
    9.81*dt()
}


pub fn meter(meters: f32) -> f32 {
    meters * 100.0
}