use std::clone;

use macroquad::{self, input::is_mouse_button_down};
use macroquad::math::Vec2;
use macroquad::color::Color;

pub(crate) mod shapes;
pub(crate) mod physics;
pub(crate) mod create_objects;
use physics::{Material, PhysicsType};
use crate::measurements::{dt};

#[allow(dead_code)]
pub(crate) trait Render {
    fn render(&self);
    fn get_area(&self) -> f32;
    fn get_pos(&mut self) -> &mut Vec2;
    fn clone_box(&mut self) -> Box<dyn Render>;
    fn get_drag_coefficient(&self) -> f32;
    fn get_colour(&self) -> Color;
    fn set_colour(&mut self, colour: Color);
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool;
}

#[derive(Clone, Copy)]
pub (crate) struct Object<T> where T: Render {
    pub(crate) shape: T,
    material: Material,
    pub(crate) dx: f32,
    pub(crate) dy: f32,
    phys_type: PhysicsType
}

#[allow(dead_code)]
impl<T: Render> Object<T>{
    pub(crate) fn new(shape: T, material: Material, phys_type: PhysicsType) -> Object<T> {
        Object {
            shape,
            material,
            dx: 0.0,
            dy: 0.0,
            phys_type,
        }
    }
    pub(crate) fn create(shape: T, mass: f32, phys_type: PhysicsType) -> Object<T> {
        Object {
            material: Material::new(mass, shape.get_area()),
            shape,
            dx: 0.0,
            dy: 0.0,
            phys_type,
        }
    }
    fn movement_process(&mut self) {
        self.shape.get_pos().y += self.dy * dt();
        self.shape.get_pos().x += self.dx * dt();
    }
}


pub fn render_objects(objects: &Vec<Box<dyn Render>>) {
    for object in objects {
        object.render();
    }
}