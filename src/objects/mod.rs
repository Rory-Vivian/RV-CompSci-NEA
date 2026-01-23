use macroquad::{self};
use macroquad::math::Vec2;
use macroquad::color::Color;

pub(crate) mod shapes;
pub(crate) mod physics;
pub(crate) mod create_objects;
use physics::{Material, PhysicsType};
use crate::measurements::{dt, Point, QuadTree};

//Give all the functions for shapes that can be rendered (this will be used for everything related to shapes)
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
    fn get_id(&self) -> &str;
    fn get_measurements(&self) -> (f32, f32);
    fn set_measurements(&mut self, measurements: (f32, f32));
    fn get_outline_colour(&self) -> &Color;
    fn set_outline_colour(&mut self, colour: Color);
    fn detect_near_object(&mut self, qtree: &mut QuadTree) -> Vec<Point>;
}

//Create the object struct
#[derive(Clone, Copy)]
pub (crate) struct Object<T> where T: Render {
    pub(crate) shape: T,
    material: Material,
    gravity: f32,
    pub(crate) dx: f32,
    pub(crate) dy: f32,
    do_air_resistance: bool,
    phys_type: PhysicsType,
    to_be_deleted: bool,
}

//Implement functions for the object trait
impl<T: Render> Object<T>{
    //New function for the Object type. dx, dy and gravity do not have to be constant, however to_be_deleted should always start false
    pub(crate) fn new(shape: T, material: Material, phys_type: PhysicsType) -> Object<T> {
        Object {
            shape,
            material,
            dx: 0.0,
            dy: 0.0,
            gravity: 9.81,
            do_air_resistance: true,
            phys_type,
            to_be_deleted: false,
        }
    }
    #[allow(dead_code)]
    //The same as the previous new function, however creates the material for the shape and mass, saving time.
    pub(crate) fn create(shape: T, mass: f32, phys_type: PhysicsType) -> Object<T> {
        Object {
            material: Material::new(mass, shape.get_area()),
            shape,
            dx: 0.0,
            dy: 0.0,
            gravity: 9.81,
            do_air_resistance: true,
            phys_type,
            to_be_deleted: false,
        }
    }
    //Process the movement of any objects
    fn movement_process(&mut self) {
        self.shape.get_pos().y += self.dy * dt();
        self.shape.get_pos().x += self.dx * dt();
    }
}


//Loop through all objects needed to be rendered, and render them
pub fn render_objects(objects: &Vec<Box<dyn Render>>) {
    for object in objects {
        object.render();
    }
}