use macroquad::prelude::*;
use macroquad::{
    shapes::draw_line,
    window::{screen_height, screen_width},
};

const THEME:Color = Color::from_rgba(61, 61, 61, 255);

//Construct the guidelines in the x-axis
fn draw_x_guidelines(camera: &Camera2D) {
    draw_line(0., 0., screen_width() + camera.target.x, 0.0, 100. / (camera.zoom.x * (screen_width() * 10.)) , THEME);
    draw_line(0., 0., -screen_width() + camera.target.x, 0.0, 100. / (camera.zoom.x * (screen_width() * 10.)), THEME);
}

//Construct the guidelines in the y-axis
fn draw_y_guidelines(camera: &Camera2D) {
    draw_line(0., 0., 0., screen_height() + camera.target.y, 100. / (camera.zoom.y * (screen_height() * 10.)), THEME);
    draw_line(0., 0., 0., -screen_height() + camera.target.y, 100. / (camera.zoom.y * (screen_height() * 10.)), THEME);
}

//Construct the guidelines in both the x and y -axis
pub fn draw_guidelines(camera: &Camera2D) {
    draw_x_guidelines(camera);
    draw_y_guidelines(camera);
}
