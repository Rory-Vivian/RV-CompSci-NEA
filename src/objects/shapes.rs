use crate::measurements::{meter};
use crate::objects::*;
use macroquad::color::Color;
use macroquad::prelude::*;
use std::f32::consts::*;

//Struct for a Square
#[derive(Clone)]
pub struct Square {
    pos: Vec2,
    size: f32,
    colour: Color,
}

//Struct for a rectangle
#[derive(Clone)]
pub struct Rectangle {
    pos: Vec2,
    width: f32,
    length: f32,
    colour: Color,
}

//Struct for a circle
#[derive(Clone)]
pub struct Circle {
    pos: Vec2,
    radius: f32,
    colour: Color,
}

//New function for a square
#[allow(dead_code)]
impl Square {
    pub(crate) fn new(pos: Vec2, size: f32, colour: Color) -> Square {
        Square { pos, size, colour }
    }
}
//New function for a Rectangle
impl Rectangle {
    pub(crate) fn new(pos: Vec2, width: f32, length: f32, colour: Color) -> Rectangle {
        Rectangle {
            pos,
            width,
            length,
            colour,
        }
    }
}
//New function for a Circle
impl Circle {
    pub(crate) fn new(pos: Vec2, radius: f32, colour: Color) -> Circle {
        Circle {
            pos,
            radius,
            colour,
        }
    }
}

//Implement render for all shapes
//Implementing render for a square
impl Render for Square {
    //Draw the shape and the outline on=top of said shape
    fn render(&self) {
        let mut outline = BLACK;
        outline.a = self.colour.a;
        draw_rectangle(
            meter(self.pos.x),
            meter(self.pos.y),
            meter(self.size),
            meter(self.size),
            self.colour,
        );
        draw_rectangle_lines(
            meter(self.pos.x),
            meter(self.pos.y),
            meter(self.size),
            meter(self.size),
            1.,
            outline,
        );
    }
    //Getters for area, position, a cloned self, drag and colour
    fn get_area(&self) -> f32 {
        self.size * self.size
    }
    fn get_pos(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
    fn clone_box(&mut self) -> Box<dyn Render> {
        Box::new(Square::new(self.pos.clone(), self.size, self.colour))
    }
    fn get_drag_coefficient(&self) -> f32 {
        1.05
    }
    fn get_colour(&self) -> Color {
        self.colour.clone()
    }
    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }
    //Calculate if the mouse is in the area of the shape
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool {
        meter(self.pos.x) <= mouse_pos.x && (meter(self.pos.x) + meter(self.size)) >= mouse_pos.x && 
            meter(self.pos.y) <= mouse_pos.y && (meter(self.pos.y + self.size)) >= mouse_pos.y
    }
    //Getter functions for the string of the shape, measurements and setters for the measurements
    fn get_id(&self) -> &str { "Square" }
    fn get_measurements(&self) -> (f32, f32) { (self.size, -1.)}
    fn set_measurements(&mut self, measurements: (f32, f32)) { self.size = measurements.0; }
}

//Implementing Render for Rectangle
impl Render for Rectangle {
    //Render the Rectangle and the Outline of said rectangle
    fn render(&self) {
        let mut outline = BLACK;
        outline.a = self.colour.a;
        draw_rectangle(
            meter(self.pos.x),
            meter(self.pos.y),
            meter(self.width),
            meter(self.length),
            self.colour,
        );
        draw_rectangle_lines(
            meter(self.pos.x),
            meter(self.pos.y),
            meter(self.width),
            meter(self.length),
            1.,
            outline,
        );
    }
    //Getter for the area, position, a clone of self, the drag co-efficient, and the colour
    fn get_area(&self) -> f32 {
        self.width * self.length
    }
    fn get_pos(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
    fn clone_box(&mut self) -> Box<dyn Render> {
        Box::new(Rectangle::new(
            self.pos.clone(),
            self.width,
            self.length,
            self.colour,
        ))
    }
    fn get_drag_coefficient(&self) -> f32 {
        1.05
    }
    fn get_colour(&self) -> Color {
        self.colour.clone()
    }
    //Setter for the colour
    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }
    //Calculate if the mouse is in the area of the shape
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool {
         meter(self.pos.x) <= mouse_pos.x && meter(self.pos.x + self.width) >= mouse_pos.x 
             && meter(self.pos.y) <= mouse_pos.y && (meter(self.pos.y) + meter(self.length)) >= mouse_pos.y
    }
    //Getter functions for the string of the shape, the measurements of the shape, and a setter for the measurements of the shape
    fn get_id(&self) -> &str { "Rectangle" }
    fn get_measurements(&self) -> (f32, f32) { (self.width, self.length) }
    fn set_measurements(&mut self, measurements: (f32, f32)) {
        self.width = measurements.0;
        self.length = measurements.1;
    }
}
//Implement Render for circle
impl Render for Circle {
    //Render the shape and then render the outline of the shape on-top of it
    fn render(&self) {
        let mut outline = BLACK;
        outline.a = self.colour.a;
        draw_poly(
            meter(self.pos.x),
            meter(self.pos.y),
            30,
            meter(self.radius),
            0.,
            self.colour,
        );
        draw_circle_lines(
            meter(self.pos.x),
            meter(self.pos.y),
            meter(self.radius),
            1.,
            outline,
        );
    }
    //Getter functions for the area, position, clone of self, drag_coefficient and colour
    fn get_area(&self) -> f32 {
        PI * ((self.radius) * (self.radius)) * (self.radius) * (self.radius)
    }
    fn get_pos(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
    fn clone_box(&mut self) -> Box<dyn Render> {
        Box::new(Circle::new(self.pos.clone(), self.radius, self.colour))
    }
    fn get_drag_coefficient(&self) -> f32 {
        0.47
    }
    fn get_colour(&self) -> Color {
        self.colour.clone()
    }
    //Setter function for the colour
    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }
    //Calculate if the mouse in the area of the shape
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool {
        let distance = (meter(self.pos.x) - mouse_pos.x) * (meter(self.pos.x) - mouse_pos.x)
            + (meter(self.pos.y) - mouse_pos.y) * (meter(self.pos.y) - mouse_pos.y);
        distance <= (meter(self.radius) * meter(self.radius))
    }
    //Getter functions for the string ID of the shape, the measurements for the shape, and a setter function for the measurements of the shape
    fn get_id(&self) -> &str { "Circle" }
    fn get_measurements(&self) -> (f32, f32) { (self.radius, -1.) }
    fn set_measurements(&mut self, measurements: (f32, f32)) { self.radius = measurements.0; }
}
