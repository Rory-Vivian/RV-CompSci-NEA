use std::ops::Add;
use macroquad::camera::Camera2D;
use macroquad::color::{GREEN, RED};
use crate::measurements::{dt, meter, QuadTree};
use crate::objects::{Object, Render};
use macroquad::math::{Vec2, Rect};
use macroquad::shapes::draw_circle_lines;

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
                resolve_overlap_circles(self, object);
            }
            return;
        }

        if object.get_render_shape_reference().get_id() == "Rectangle" && self.get_render_shape().get_id() == object.get_render_shape().get_id() {
            let mut overlap = false;

            let l1 = self.get_render_shape().get_pos().clone();
            let l2 = object.get_render_shape().get_pos().clone();

            let r1 = Rect::new(l1.x, l1.y, self.get_render_shape().get_measurements().0, self.get_render_shape().get_measurements().1);
            let r2 = Rect::new(l2.x, l2.y, object.get_render_shape().get_measurements().0, object.get_render_shape().get_measurements().1);

            if r1.intersect(r2).is_some() || r2.intersect(r1).is_some() {
                overlap = true;
            }

            if overlap {
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);

                resolve_overlap_rect(self, object);
            }
        }


        if self.get_render_shape_reference().get_id() == "Circle" && object.get_render_shape_reference().get_id() == "Rectangle" {
            let closest_point = get_closest_point(self.get_render_shape().get_pos().clone(),
                                                  object.get_render_shape().get_pos().clone(),
                                                  Vec2::new(object.get_render_shape().get_pos().x + object.get_render_shape().get_measurements().0,
                                                            object.get_render_shape().get_pos().y + object.get_render_shape().get_measurements().1));

            let target = self.get_render_shape().get_pos().clone();
            let overlap = target.distance(closest_point) < self.get_render_shape_reference().get_measurements().0;

            if overlap {
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);

                let mut circle_type = self.get_physics_type().clone();
                let mut rect_type = object.get_physics_type().clone();

                resolve_overlap_cirect(self.get_render_shape_reference(), object.get_render_shape_reference(), &mut circle_type, &mut rect_type);
            }
        }

        if self.get_render_shape_reference().get_id() == "Rectangle" && object.get_render_shape_reference().get_id() == "Circle" {

            //Check Vertices
            let closest_point = get_closest_point(object.get_render_shape().get_pos().clone(),
                                                  self.get_render_shape().get_pos().clone(),
                                                  Vec2::new(self.get_render_shape().get_pos().x + self.get_render_shape().get_measurements().0,
                                                            self.get_render_shape().get_pos().y + self.get_render_shape().get_measurements().1));

            let target = object.get_render_shape().get_pos().clone();
            let overlap = target.distance(closest_point) < object.get_render_shape_reference().get_measurements().0;
            if overlap {
                self.get_render_shape_reference().set_colour(GREEN);
                object.get_render_shape_reference().set_colour(GREEN);

                let mut circle_type = object.get_physics_type().clone();
                let mut rect_type = self.get_physics_type().clone();

                resolve_overlap_cirect(object.get_render_shape_reference(), self.get_render_shape_reference(), &mut circle_type, &mut rect_type);
            }
        }
    }
}

fn get_closest_point(target: Vec2, corner1: Vec2, corner2: Vec2) -> Vec2 {
    let mut closest_point = corner1;

    let min_x = corner1.x.min(corner2.x);
    let max_x = corner1.x.max(corner2.x);
    let min_y = corner1.y.min(corner2.y);
    let max_y = corner1.y.max(corner2.y);

    let true_x = target.x.clamp(min_x, max_x);
    let true_y = target.y.clamp(min_y, max_y);

    draw_circle_lines(meter(true_x), meter(true_y), 1.,1., RED);

    Vec2::new(true_x, true_y)
}

fn resolve_overlap_circles(object_1: &mut dyn PhysicsObject, object_2: &mut Box<dyn PhysicsObject>) {
    let distance = object_1.get_render_shape().get_pos().distance(object_2.get_render_shape().get_pos().clone());
    let dx = object_2.get_render_shape().get_pos().x - object_1.get_render_shape().get_pos().x;
    let dy = object_2.get_render_shape().get_pos().y - object_1.get_render_shape().get_pos().y;

    let overlap = (object_1.get_render_shape().get_measurements().0 + object_2.get_render_shape().get_measurements().0) - distance;

    let nx = dx / distance;
    let ny = dy / distance;

    let mut move_it_1 = Vec2::new(nx * (overlap/2.), ny * (overlap/2.));
    let mut move_it_2 = move_it_1.clone();

    if matches!(object_1.get_physics_type(), PhysicsType::Static) {
        move_it_1.x = 0.;
        move_it_1.y = 0.;

        move_it_2.x = move_it_2.x * 2.;
        move_it_2.y = move_it_2.y * 2.;
    }
    if matches!(object_2.get_physics_type(), PhysicsType::Static) {
        move_it_2.x = 0.;
        move_it_2.y = 0.;

        move_it_1.x = move_it_1.x * 2.;
        move_it_1.y = move_it_1.y * 2.;
    }
    object_1.get_render_shape_reference().get_pos().x -= move_it_1.x;
    object_1.get_render_shape_reference().get_pos().y -= move_it_1.y;
    object_2.get_render_shape_reference().get_pos().x += move_it_2.x;
    object_2.get_render_shape_reference().get_pos().y += move_it_2.y;
}

fn resolve_overlap_rect(object_1: &mut dyn PhysicsObject, object_2: &mut Box<dyn PhysicsObject>) {
    // 1. Fix: Get pos2 from object_2, not object_1!
    let pos1 = object_1.get_render_shape().get_pos().clone();
    let pos2 = object_2.get_render_shape().get_pos().clone(); // Fixed this line

    let m1 = object_1.get_render_shape().get_measurements();
    let m2 = object_2.get_render_shape().get_measurements();

    // Calculate actual overlap
    let overlap_x = (pos1.x + m1.0).min(pos2.x + m2.0) - pos1.x.max(pos2.x);
    let overlap_y = (pos1.y + m1.1).min(pos2.y + m2.1) - pos1.y.max(pos2.y);

    let mut object1_overlap = Vec2::new(overlap_x/2., overlap_y/2.);
    let mut object2_overlap = Vec2::new(overlap_x/2., overlap_y/2.);

    if matches!(object_1.get_physics_type(), PhysicsType::Static) {
        object1_overlap = Vec2::new(0., 0.);
        object2_overlap = Vec2::new(object2_overlap.x *2. , object2_overlap.y *2.);
    }
    if matches!(object_2.get_physics_type(), PhysicsType::Static) {
        object2_overlap = Vec2::new(0., 0.);
        object1_overlap = Vec2::new(object1_overlap.x *2. , object1_overlap.y *2.);
    }

    // 2. Fix: Only resolve if there is a real overlap (both > 0)
    if overlap_x > 0.0 && overlap_y > 0.0 {

        // 3. Fix: Resolve the SHORTEST distance (Minimum Separation Vector)
        if overlap_x < overlap_y {
            // Resolve on X Axis
            if pos1.x < pos2.x {
                object_1.get_render_shape_reference().get_pos().x -= object1_overlap.x;
                object_2.get_render_shape_reference().get_pos().x += object2_overlap.x;
            } else {
                object_1.get_render_shape_reference().get_pos().x += object1_overlap.x;
                object_2.get_render_shape_reference().get_pos().x -= object2_overlap.x;
            }
        } else {
            // Resolve on Y Axis
            if pos1.y < pos2.y {
                object_1.get_render_shape_reference().get_pos().y -= object1_overlap.y;
                object_2.get_render_shape_reference().get_pos().y += object2_overlap.y;
            } else {
                object_1.get_render_shape_reference().get_pos().y += object1_overlap.y;
                object_2.get_render_shape_reference().get_pos().y -= object2_overlap.y;
            }
        }
    }
}

fn resolve_overlap_cirect(mut cirlce: Box<&mut dyn Render>, mut rect: Box<&mut dyn Render>, circle_type: &mut PhysicsType, rect_type: &mut PhysicsType) {
    let target = cirlce.get_pos().clone();
    let l1 = rect.get_pos().clone();
    let l2 = Vec2::new(rect.get_pos().x + rect.get_measurements().0, rect.get_pos().y + rect.get_measurements().1);
    let closest_point = get_closest_point(target, l1, l2);

    let dist_x = cirlce.get_pos().x - closest_point.x;
    let dist_y = cirlce.get_pos().y - closest_point.y;

    let radius = cirlce.get_measurements().0.clone();
    let dist = (dist_x*dist_x + dist_y*dist_y).sqrt();
    let overlap = radius - dist;

    if dist == 0.0 { return; }
    if dist.powf(2.) >= radius * radius { return; }

    let nx = dist_x/dist.sqrt();
    let ny = dist_y/dist;

    let mut move_circle = Vec2::new(nx * overlap/2., ny * overlap/2.);
    let mut move_rect = Vec2::new(nx * overlap/2., ny * overlap/2.);

    if matches!(circle_type, PhysicsType::Static) {
        move_circle = Vec2::new(0., 0.);
        move_rect = Vec2::new(move_rect.x * 2., move_rect.y * 2.);
    }
    if matches!(rect_type, PhysicsType::Static) {
        move_rect = Vec2::new(0., 0.);
        move_circle = Vec2::new(move_circle.x * 2., move_circle.y * 2.);
    }

    cirlce.get_pos().x += move_circle.x; cirlce.get_pos().y += move_circle.y;
    rect.get_pos().x -= move_rect.x; rect.get_pos().y -= move_rect.y;
}