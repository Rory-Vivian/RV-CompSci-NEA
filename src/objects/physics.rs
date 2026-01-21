use macroquad::camera::Camera2D;
use macroquad::color::GREEN;
use crate::measurements::{dt, Point, QuadTree, Rect};
use crate::objects::{Object, Render};
use macroquad::math::Vec2;
use crate::objects::shapes::Square;

//Create the PhysicsType enum
#[derive(Clone, Copy)]
pub(crate) enum PhysicsType {
    Static,
    Dynamic,
    Kinematic,
}

//Create the material struct
#[derive(Clone, Copy)]
pub struct Material {
    pub(crate) mass: f32,
    pub(crate) area: f32,
    pub(crate) density: f32,
}

//Create the trait PhysicObject to be given to all physics objects (Objects)
#[allow(unused)]
pub(crate) trait PhysicsObject {
    fn physics_process(&mut self, camera: &Camera2D);
    fn get_drag(&self) -> Vec2;
    fn get_terminal_velocity(&self) -> f32;
    fn get_physics_type(&mut self) -> &mut PhysicsType;
    fn set_physics_type(&mut self, new_type: PhysicsType);
    fn get_render_shape(&mut self) -> Box<dyn Render>;
    fn get_render_shape_reference(&mut self) -> Box<&mut dyn Render>;
    fn get_material(&mut self) -> &mut Material;
    fn update_material(&mut self);
    fn get_to_be_deleted(&mut self) -> &mut bool;
    fn get_gravity(&mut self) -> &mut f32;
    fn get_velocity(&self) -> Vec2;
    fn set_velocity(&mut self, velocity: Vec2);
    fn set_do_air_resistance(&mut self) -> &mut bool;
    fn detect_near_object(&mut self, qtree: &mut QuadTree, objects: Vec<&mut Box<dyn PhysicsObject>>);
    fn check_collisions (&mut self, object: &mut Box<dyn PhysicsObject>);
}

//Give default functions to material
impl Material {
    pub(crate) fn new(mass: f32, area: f32) -> Material {
        Material {
            mass,
            area,
            density: mass / area,
        }
    }
}

//Implement PhysicsObject to the object struct
impl<T: Render + Clone + 'static> PhysicsObject for Object<T> {
    //Perform the physics process for different objects
    fn physics_process(&mut self, _camera: &Camera2D) {
        match self.get_physics_type() {
            PhysicsType::Static => {}
            PhysicsType::Dynamic => {
                //Process the drag and gravity for all directions of movement
                self.dy += self.gravity * dt();
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
                //Move the object in the correct direction
                self.movement_process();
            }
            PhysicsType::Kinematic => {
                //Move the object in the correct direction
                self.movement_process();
            }
        };
    }

    //Calculate the drag in the x and y direction for any object
    fn get_drag(&self) -> Vec2 {
        //Solve for drag using F = 1/2 p v^2 A C_d
        let drag_x = 0.5 * 1.29 * (self.dx * self.dx) * self.shape.get_area() * 1.05;
        let drag_y = 0.5 * 1.29 * (self.dy * self.dy) * self.shape.get_area() * 1.05;

        //Resolve for the force applied
        let dc_x = drag_x / self.material.mass;
        let dc_y = drag_y / self.material.mass;
        //Return the value of the drag
        Vec2::new(dc_x, dc_y)
    }
    //Find the terminal velocity of the object (TO BE USED)
    #[allow(dead_code)]
    fn get_terminal_velocity(&self) -> f32 {
        f32::sqrt((2.0 * self.material.mass * 1.0) * (1.29 * self.shape.get_area() * 1.05))
    }
    //Setters and getters for the physics type
    fn get_physics_type(&mut self) -> &mut PhysicsType {
        &mut self.phys_type
    }
    fn set_physics_type(&mut self, new_type: PhysicsType) {
        self.phys_type = new_type;
    }
    //Getters for the render shape and material
    fn get_render_shape(&mut self) -> Box<dyn Render> {
        self.shape.clone_box()
    }
    fn get_render_shape_reference(&mut self) -> Box<&mut dyn Render> {
        Box::new(&mut self.shape)
    }
    fn get_material(&mut self) -> &mut Material { &mut self.material }
    //Update the material of the object
    fn update_material(&mut self) {
        self.material.area = self.shape.get_area();
        self.material.density = self.material.mass / self.material.area;
    }
    //Getters for the gravity, to_be_deleted and velocity. Also, setters for the velocity
    fn get_to_be_deleted(&mut self) -> &mut bool { &mut self.to_be_deleted }
    fn get_gravity(&mut self) -> &mut f32 { &mut self.gravity }
    fn get_velocity(&self) -> Vec2 { Vec2::new(self.dx, self.dy) }
    fn set_velocity(&mut self, velocity: Vec2) {
        self.dx = velocity.x;
        self.dy = velocity.y;
    }
    fn set_do_air_resistance(&mut self) -> &mut bool { &mut self.do_air_resistance }

    fn detect_near_object(&mut self, mut qtree: &mut QuadTree, mut objects: Vec<&mut Box<dyn PhysicsObject>>) {
        let points = self.get_render_shape_reference().detect_near_object(&mut qtree);
        for i in points {
            if objects.get_mut(i.index).is_some() {
                self.check_collisions(objects.get_mut(i.index).unwrap());
            };
        }
    }

    fn check_collisions(&mut self, object: &mut Box<dyn PhysicsObject>) {
        if object.get_render_shape_reference().get_id() == "Circle" && object.get_render_shape_reference().get_id() == self.get_render_shape_reference().get_id() {
            if self.get_render_shape_reference().get_pos().distance(*object.get_render_shape_reference().get_pos()) <=
                self.get_render_shape_reference().get_measurements().0 + object.get_render_shape_reference().get_measurements().0 {

                //Objects are colliding
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);
            }
            return;
        }

        if object.get_render_shape_reference().get_id() == "Rectangle" && self.get_render_shape().get_id() == object.get_render_shape().get_id() {
            let mut overlap = true;
            let pos = self.get_render_shape_reference().get_pos().clone();
            let pos_other = object.get_render_shape().get_pos().clone();
            if pos.x > pos_other.x + object.get_render_shape_reference().get_measurements().0 || pos_other.x > pos.x + self.get_render_shape_reference().get_measurements().0 {
                overlap = false;
            }
            if pos.y > pos_other.y + object.get_render_shape().get_measurements().1 || pos_other.y > pos.y + self.get_render_shape_reference().get_measurements().1 {
                overlap = false;
            }

            if overlap {
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);
            }
            return;
        }

        let mut overlap = false;
        if self.get_render_shape_reference().get_id() == "Circle" && object.get_render_shape_reference().get_id() == "Rectangle" {
            let mut closest = object.get_render_shape_reference().get_pos().clone();
            let v2 = Vec2::new(object.get_render_shape_reference().get_pos().x + object.get_render_shape_reference().get_measurements().0, object.get_render_shape_reference().get_pos().y);
            let v3 = Vec2::new(object.get_render_shape_reference().get_pos().x, object.get_render_shape_reference().get_pos().y + object.get_render_shape_reference().get_measurements().1);
            let v4 = Vec2::new(object.get_render_shape_reference().get_pos().x + object.get_render_shape_reference().get_measurements().0, object.get_render_shape_reference().get_pos().y + object.get_render_shape_reference().get_measurements().1);

            if self.get_render_shape_reference().get_pos().distance(closest) >
                self.get_render_shape_reference().get_pos().distance(v2) {
                closest = v2;
            }
            if self.get_render_shape_reference().get_pos().distance(closest) >
                self.get_render_shape_reference().get_pos().distance(v3) {
                closest = v3;
            }
            if self.get_render_shape_reference().get_pos().distance(closest) >
                self.get_render_shape_reference().get_pos().distance(v4) {
                closest = v4;
            }

            let overlap = self.get_render_shape_reference().get_pos().distance(closest) < self.get_render_shape_reference().get_measurements().0;
            if overlap {
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);
            }

        } else if self.get_render_shape_reference().get_id() == "Rectangle" && object.get_render_shape_reference().get_id() == "Circle" {
            let mut closest = self.get_render_shape_reference().get_pos().clone();
            let v2 = Vec2::new(self.get_render_shape_reference().get_pos().x + self.get_render_shape_reference().get_measurements().0, self.get_render_shape_reference().get_pos().y);
            let v3 = Vec2::new(self.get_render_shape_reference().get_pos().x, self.get_render_shape_reference().get_pos().y + self.get_render_shape_reference().get_measurements().1);
            let v4 = Vec2::new(self.get_render_shape_reference().get_pos().x + self.get_render_shape_reference().get_measurements().0, self.get_render_shape_reference().get_pos().y + self.get_render_shape_reference().get_measurements().1);

            if object.get_render_shape_reference().get_pos().distance(closest) >
                object.get_render_shape_reference().get_pos().distance(v2) {
                closest = v2;
            }
            if object.get_render_shape_reference().get_pos().distance(closest) >
                object.get_render_shape_reference().get_pos().distance(v3) {
                closest = v3;
            }
            if object.get_render_shape_reference().get_pos().distance(closest) >
                object.get_render_shape_reference().get_pos().distance(v4) {
                closest = v4;
            }

            let overlap = object.get_render_shape_reference().get_pos().distance(closest) < object.get_render_shape_reference().get_measurements().0;
            if overlap {
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);
            }
        }
    }
}
