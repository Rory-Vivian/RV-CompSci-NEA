use macroquad::math::Vec2;
use crate::measurements::{dt, get_gravity};
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
    fn get_physics_type(&self) -> &PhysicsType;
    fn get_render_shape(&mut self) -> Box<dyn Render>;
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
        match self.get_physics_type() {
            PhysicsType::Static => {
            }
            PhysicsType::Dynamic => {
                self.dy += get_gravity();
                if self.dx > 0.0 { self.dx -= self.get_drag().x * dt()} 
                else if self.dx < 0.0 { self.dx += self.get_drag().x * dt()}
                if self.dy > 0.0 { self.dy -= self.get_drag().y * dt()}
                else if self.dy < 0.0 { self.dy += self.get_drag().x * dt()}
                self.movement_process();
            }
            PhysicsType::Kinematic => {
                self.movement_process();
            }
        };
    }

    fn get_drag(&self) -> Vec2 {
        let drag_x = 0.5 * 1.29 * (self.dx * self.dx) * self.shape.get_area() * 1.05;
        let drag_y = 0.5 * 1.29 * (self.dy * self.dy) * self.shape.get_area() * 1.05;

        let dc_x = drag_x / self.material.mass;
        let dc_y = drag_y / self.material.mass;
        Vec2::new(dc_x,dc_y)
    }
    fn get_terminal_velocity(&self) -> f32 {
        f32::sqrt((2.0 * self.material.mass * 1.0) * (1.29 * self.shape.get_area() * 1.05))
    }

    fn get_physics_type(&self) -> &PhysicsType {
        &self.phys_type
    }
    
    fn get_render_shape(&mut self) -> Box<dyn Render> {
        self.shape.clone()
    }
}



