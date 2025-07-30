use macroquad;
use macroquad::math::Vec2;

pub(crate) mod shapes;
pub(crate) mod physics;
use physics::{Material};

#[allow(dead_code)]
pub(crate) trait Render {
    fn render(&self);
    fn get_area(&self) -> f32;
    fn get_pos(&mut self) -> &mut Vec2;
    fn clone(&mut self) -> Box<dyn Render>;
}

#[allow(dead_code)]
pub (crate) struct Object<T> where T: Render {
    pub(crate) shape: T,
    material: Material,
    pub(crate) dx: f32,
    dy: f32,
}

#[allow(dead_code)]
impl<T: Render> Object<T>{
    pub(crate) fn new(shape: T, material: Material) -> Object<T> {
        Object {
            shape,
            material,
            dx: 0.0,
            dy: 0.0,
        }
    }
    pub(crate) fn create(shape: T, mass: f32) -> Object<T> {
        Object {
            material: Material::new(mass, shape.get_area()),
            shape,
            dx: 0.0,
            dy: 0.0,
        }
    }

    pub fn physics_process(&mut self) {
        self.shape.get_pos().x += self.dx;
        self.shape.get_pos().y += self.dy;
    }
}

pub fn render_objects(objects: &Vec<Box<dyn Render>>) {
    for object in objects {
        object.render();
    }
}