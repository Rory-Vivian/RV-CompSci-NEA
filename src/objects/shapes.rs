use macroquad::prelude::*;
use macroquad::color::Color;
use crate::objects::*;
use std::f32::consts::*;
use crate::measurements::{meter};

#[derive(Clone)]
pub struct Square {
    pos: Vec2,
    size: f32,
    colour: Color
}

#[derive(Clone)]
pub struct Rectangle {
    pos: Vec2,
    width: f32,
    length: f32,
    colour: Color
}

#[derive(Clone)]
pub struct Circle {
    pos: Vec2,
    radius: f32,
    colour: Color
}

impl Square {
    pub(crate) fn new(pos: Vec2, size: f32, colour: Color) -> Square {
        Square {
            pos,
            size,
            colour,
        }
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
        draw_rectangle(meter(self.pos.x), meter(self.pos.y), meter(self.size), meter(self.size), self.colour);
        draw_rectangle_lines(meter(self.pos.x), meter(self.pos.y), meter(self.size), meter(self.size), 1., BLACK);
    }
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
}
impl Render for Rectangle {
    fn render(&self) {
        draw_rectangle(meter(self.pos.x), meter(self.pos.y), meter(self.width), meter(self.length), self.colour);
        draw_rectangle_lines(meter(self.pos.x), meter(self.pos.y), meter(self.width), meter(self.length), 1., BLACK);
    }
    fn get_area(&self) -> f32 {
        self.width * self.length
    }
    fn get_pos(&mut self) -> &mut Vec2 {
        &mut self.pos
    }

    fn clone_box(&mut self) -> Box<dyn Render> {
        Box::new(Rectangle::new(self.pos.clone(), self.width, self.length, self.colour))
    }
    fn get_drag_coefficient(&self) -> f32 {
        1.05
    }
}
impl Render for Circle {
    fn render(&self) {
        draw_poly(meter(self.pos.x), meter(self.pos.y), 30, meter(self.radius), 0. ,self.colour);
        draw_circle_lines(meter(self.pos.x), meter(self.pos.y), meter(self.radius), 1., BLACK);
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
}