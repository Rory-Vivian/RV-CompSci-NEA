use macroquad::math::Vec2;
use macroquad::ui::Drag;
use crate::objects::{Object, Render};

pub(crate) enum PhysicsType {
    Static,
    Dynamic,
    Kinematic,
}

pub struct Material {
    mass: f32,
    area: f32,
    density: f32,
}

pub(crate) trait PhysicsObeject {
    fn physics_process(&mut self);
    fn get_drag(&self) -> Vec2;
    fn get_terminal_velocity(&self) -> f32;
}

impl Material {
    pub(crate) fn new(mass: f32, area: f32) -> Material {
        Material{
            mass,
            area,
            density: mass/area,
        }
    }
}

impl<T: Render> PhysicsObeject for Object<T> {
    fn physics_process(&mut self) {

    }

    fn get_drag(&self) -> Vec2 {
        todo!()
    }

    fn get_terminal_velocity(&self) -> f32 {
        todo!()
    }
}



