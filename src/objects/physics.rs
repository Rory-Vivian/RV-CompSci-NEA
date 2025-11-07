use macroquad::camera::Camera2D;
use crate::measurements::{dt, get_gravity};
use crate::objects::{Object, Render};
use macroquad::math::Vec2;

#[allow(unused)]
#[derive(Clone, Copy)]
pub(crate) enum PhysicsType {
    Static,
    Dynamic,
    Kinematic,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Material {
    pub(crate) mass: f32,
    pub(crate) area: f32,
    pub(crate) density: f32,
}

#[allow(unused)]
pub(crate) trait PhysicsObeject {
    fn physics_process(&mut self, camera: &Camera2D);
    fn get_drag(&self) -> Vec2;
    fn get_terminal_velocity(&self) -> f32;
    fn get_physics_type(&mut self) -> &mut PhysicsType;
    fn set_physics_type(&mut self, new_type: PhysicsType);
    fn get_render_shape(&mut self) -> Box<dyn Render>;
    fn get_render_shape_referance(&mut self) -> Box<&mut dyn Render>;
    fn get_material(&mut self) -> &mut Material;
    fn update_material(&mut self);
    fn get_to_be_deleted(&mut self) -> &mut bool;
}

impl Material {
    pub(crate) fn new(mass: f32, area: f32) -> Material {
        Material {
            mass,
            area,
            density: mass / area,
        }
    }
}

impl<T: Render + Clone + 'static> PhysicsObeject for Object<T> {
    fn physics_process(&mut self, _camera: &Camera2D) {
        match self.get_physics_type() {
            PhysicsType::Static => {}
            PhysicsType::Dynamic => {
                self.dy += get_gravity();
                if self.dx > 0.0 {
                    self.dx -= self.get_drag().x * dt()
                } else if self.dx < 0.0 {
                    self.dx += self.get_drag().x * dt()
                }
                if self.dy > 0.0 {
                    self.dy -= self.get_drag().y * dt()
                } else if self.dy < 0.0 {
                    self.dy += self.get_drag().x * dt()
                }
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
        Vec2::new(dc_x, dc_y)
    }
    fn get_terminal_velocity(&self) -> f32 {
        f32::sqrt((2.0 * self.material.mass * 1.0) * (1.29 * self.shape.get_area() * 1.05))
    }

    fn get_physics_type(&mut self) -> &mut PhysicsType {
        &mut self.phys_type
    }
    fn set_physics_type(&mut self, new_type: PhysicsType) {
        self.phys_type = new_type;
    }

    fn get_render_shape(&mut self) -> Box<dyn Render> {
        self.shape.clone_box()
    }
    fn get_render_shape_referance(&mut self) -> Box<&mut dyn Render> {
        Box::new(&mut self.shape)
    }
    fn get_material(&mut self) -> &mut Material { &mut self.material }
    fn update_material(&mut self) {
        self.material.area = self.shape.get_area();
        self.material.density = self.material.mass / self.material.area;
        println!("Updated the material!")
    }
    fn get_to_be_deleted(&mut self) -> &mut bool { &mut self.to_be_deleted }
}
