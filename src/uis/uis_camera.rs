use macroquad::camera::{set_camera, Camera2D};

pub

fn start_camera() {
    set_camera(&Camera2D {
        rotation: 0.0,
        zoom: Default::default(),
        target: Default::default(),
        offset: Default::default(),
        render_target: None,
        viewport: None,
    });
}