use crate::measurements::{meter};
use crate::objects::*;
use macroquad::color::Color;
use macroquad::prelude::*;
use std::f32::consts::*;

#[derive(Clone)]
pub struct Square {
    pos: Vec2,
    size: f32,
    colour: Color,
}

#[derive(Clone)]
pub struct Rectangle {
    pos: Vec2,
    width: f32,
    length: f32,
    colour: Color,
}

#[derive(Clone)]
pub struct Circle {
    pos: Vec2,
    radius: f32,
    colour: Color,
}

impl Square {
    pub(crate) fn new(pos: Vec2, size: f32, colour: Color) -> Square {
        Square { pos, size, colour }
    }
}
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
impl Render for Square {
    fn render(&self) {
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
            BLACK,
        );
    }
    fn get_area(&self) -> f32 {
        self.size * self.size
    }
    fn get_pos(&mut self) -> &mut Vec2 {
        &mut self.pos
    }
    fn get_colour(&self) -> Color {
        self.colour.clone()
    }
    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }

    fn clone_box(&mut self) -> Box<dyn Render> {
        Box::new(Square::new(self.pos.clone(), self.size, self.colour))
    }
    fn get_drag_coefficient(&self) -> f32 {
        1.05
    }
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool {
        if meter(self.pos.x) <= mouse_pos.x && (meter(self.pos.x) + meter(self.size)) >= mouse_pos.x {
            if meter(self.pos.y) <= mouse_pos.y && (meter(self.pos.y + self.size)) >= mouse_pos.y {
                return true
            }
        }
        false
    }
    fn get_id(&self) -> &str { "Square" }
    fn get_measurements(&self) -> (f32, f32) { (self.size, -1.)}
    fn set_measurements(&mut self, measurements: (f32, f32)) { self.size = measurements.0; }
}

impl Render for Rectangle {
    fn render(&self) {
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
            BLACK,
        );
    }
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
    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool {
        if meter(self.pos.x) <= mouse_pos.x && meter(self.pos.x + self.width) >= mouse_pos.x {
            if meter(self.pos.y) <= mouse_pos.y && (meter(self.pos.y) + meter(self.length)) >= mouse_pos.y {
                return true
            }
        }
        false
    }
    fn get_id(&self) -> &str { "Rectangle" }
    fn get_measurements(&self) -> (f32, f32) { (self.width, self.length) }
    fn set_measurements(&mut self, measurements: (f32, f32)) {
        self.width = measurements.0;
        self.length = measurements.1;
    }
}
impl Render for Circle {
    fn render(&self) {
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
            BLACK,
        );
    }
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
    fn set_colour(&mut self, colour: Color) {
        self.colour = colour;
    }
    fn mouse_in_area(&self, mouse_pos: Vec2) -> bool {
        let distance = (meter(self.pos.x) - mouse_pos.x) * (meter(self.pos.x) - mouse_pos.x)
            + (meter(self.pos.y) - mouse_pos.y) * (meter(self.pos.y) - mouse_pos.y);
        distance <= (meter(self.radius) * meter(self.radius))
    }
    fn get_id(&self) -> &str { "Circle" }
    fn get_measurements(&self) -> (f32, f32) { (self.radius, -1.) }
    fn set_measurements(&mut self, measurements: (f32, f32)) { self.radius = measurements.0; }
}
