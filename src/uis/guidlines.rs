use macroquad::prelude::*;
use macroquad::{
    color::PURPLE,
    shapes::draw_line,
    ui::root_ui,
    window::{screen_height, screen_width},
};

fn draw_x_guidlines(camera: &Camera2D) {
    draw_line(0., 0., screen_width() + camera.target.x, 0.0, 100. / (camera.zoom.x * (screen_width() * 10.)) , PURPLE);
    draw_line(0., 0., -screen_width() + camera.target.x, 0.0, 100. / (camera.zoom.x * (screen_width() * 10.)), PURPLE);
}

fn draw_y_guidlines(camera: &Camera2D) {
    draw_line(0., 0., 0., screen_height() + camera.target.y, 100. / (camera.zoom.y * (screen_height() * 10.)), PURPLE);
    draw_line(0., 0., 0., -screen_height() + camera.target.y, 100. / (camera.zoom.y * (screen_height() * 10.)), PURPLE);
}

pub fn draw_guidlines(camera: &Camera2D) {
    draw_x_guidlines(camera);
    draw_y_guidlines(camera);
}
