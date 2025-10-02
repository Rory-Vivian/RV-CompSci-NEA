#![allow(unused, dead_code)]

use macroquad::input::mouse_position;
use macroquad::prelude::camera::mouse;
use macroquad::prelude::scene::camera_pos;
use macroquad::prelude::*;

use crate::objects::physics::{Material, PhysicsType};
use crate::objects::shapes::Square;
use crate::objects::{self, Object};
use crate::{MouseMode, Render};

fn create_square_render(pos1: Vec2, pos2: Vec2, colour: Color) -> Square {
    let dx: f32 = (pos2.x - pos1.x).abs();
    let dy: f32 = (pos2.y - pos1.y).abs();
    let length = dx.max(dy);
    
    let pos = Vec2::new(pos1.x.min(pos2.x), pos1.y.min(pos2.y));
    Square::new(pos, length, colour)
}

fn create_square(pos_1: Vec2, pos_2: Vec2) -> Object<Square> {
    let square = create_square_render(pos_1, pos_2, WHITE);
    let material = Material::new(square.get_area() * 0.98, square.get_area());
    Object::new(square, material, PhysicsType::Static)
}

pub fn draw_process(mouse_mode: MouseMode, first_mouse_pos: &mut Option<Vec2>, camera: &Camera2D) {
    match mouse_mode {
        MouseMode::DrawSquare => {
            if is_mouse_button_down(MouseButton::Left) {
                println!("draw process");
                let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
                if let Some(pos1) = *first_mouse_pos {
                    let mut square = create_square_render(pos1, pos2, PURPLE);
                    println!("pos1: {}, pos2: {}, mouse: x:{},y:{}", pos1, pos2, mouse_position().0, mouse_position().1);
                    square.render();
                }else {
                    *first_mouse_pos = Some(pos2);
                }
            } else {
                *first_mouse_pos = None;
            }
        }
        _ => {}
    }
}
