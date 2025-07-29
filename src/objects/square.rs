use macroquad::prelude::*;
use macroquad::color::Color;
use crate::objects::*;

pub struct Square {
    pos: Vec2,
    size: f32,
    colour: Color
}

pub struct Rectangle {
    pos: Vec2,
    width: f32,
    length: f32,
    colour: Color
}

impl Square {
    pub(crate) fn new(pos: Vec2, size: f32, color: Color) -> Square {
        Square {
            pos,
            size,
            colour: color,
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

impl Render for Square {
    fn render(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, self.colour);
    }
}
impl Render for Rectangle {
    fn render(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.width, self.length, self.colour);
    }
}