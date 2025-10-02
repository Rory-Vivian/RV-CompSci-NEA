#![allow(unused, dead_code)]

use macroquad::input::mouse_position;
use macroquad::prelude::camera::mouse;
use macroquad::prelude::scene::camera_pos;
use macroquad::prelude::*;

use crate::measurements::vec2_meter;
use crate::objects::physics::{Material, PhysicsType};
use crate::objects::shapes::{Rectangle, Square};
use crate::objects::{self, Object, Render};
use crate::{MouseMode};

fn create_square_render(pos1: Vec2, pos2: Vec2, colour: Color) -> Rectangle {
    let pos1 = vec2_meter(pos1);
    let pos2 = vec2_meter(pos2);

    let dx: f32 = (pos2.x - pos1.x);
    let dy: f32 = (pos2.y - pos1.y);

    let length = dx.abs().max(dy.abs());
    let x: f32;
    let y: f32;
    if pos2.x >= pos1.x {
        x = length.abs();
    } else {
        x = -length.abs();
    }
    if pos2.y >= pos1.y {
        y = length.abs();
    } else {
        y = -length.abs();
    }

    Rectangle::new(pos1, x, y, colour)
}

fn create_square(pos_1: Vec2, pos_2: Vec2) -> Object<Rectangle> {
    let square = create_square_render(pos_1, pos_2, WHITE);
    let material = Material::new(square.get_area() * 0.98, square.get_area());
    Object::new(square, material, PhysicsType::Static)
}

pub fn draw_process<T: Render>(mouse_mode: MouseMode, first_mouse_pos: &mut Option<Vec2>, camera: &Camera2D) -> Option<Object<T>> {
    match mouse_mode {
        MouseMode::DrawSquare => {
            if is_mouse_button_down(MouseButton::Left) {
                let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
                if let Some(pos1) = *first_mouse_pos {
                    let mut square = create_square_render(pos1, pos2, PURPLE);
                    square.render();
                } else {
                    *first_mouse_pos = Some(pos2);
                }
            } else {
                if let Some(pos1) = *first_mouse_pos {
                    let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
                    let square = create_square(pos1, pos2);
                }
                *first_mouse_pos = None;
            }
        }
        _ => {}
    }
    None
}
