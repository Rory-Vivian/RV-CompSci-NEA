use macroquad::prelude::*;
use macroquad::{
    shapes::draw_line,
    window::{screen_height, screen_width},
};

const THEME:Color = Color::from_rgba(61, 61, 61, 255);

fn draw_x_guidlines(camera: &Camera2D) {
    draw_line(0., 0., screen_width() + camera.target.x, 0.0, 100. / (camera.zoom.x * (screen_width() * 10.)) , THEME);
    draw_line(0., 0., -screen_width() + camera.target.x, 0.0, 100. / (camera.zoom.x * (screen_width() * 10.)), THEME);
}

fn draw_y_guidlines(camera: &Camera2D) {
    draw_line(0., 0., 0., screen_height() + camera.target.y, 100. / (camera.zoom.y * (screen_height() * 10.)), THEME);
    draw_line(0., 0., 0., -screen_height() + camera.target.y, 100. / (camera.zoom.y * (screen_height() * 10.)), THEME);
}

pub fn draw_guidlines(camera: &Camera2D) {
    draw_x_guidlines(camera);
    draw_y_guidlines(camera);
}
