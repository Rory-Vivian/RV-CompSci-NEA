#![allow(unused)]
use macroquad::color::{RED, YELLOW};
use macroquad::time::get_frame_time;
use macroquad::math::Vec2;
use macroquad::prelude::{rand, WHITE};
use macroquad::rand::gen_range;
use macroquad::shapes::{draw_circle, draw_rectangle_lines};

//Rect used to store the area for a Q-Tree, or
#[derive(Clone, Copy)]
pub struct Rect {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) w: f32,
    pub(crate) h: f32,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub index: usize,
}

pub struct QuadTree {
    boundary: Rect,
    capacity: usize,
    points: Vec<Point>,
    subdivided: bool,
    ne: Option<Box<QuadTree>>,
    nw: Option<Box<QuadTree>>,
    se: Option<Box<QuadTree>>,
    sw: Option<Box<QuadTree>>,
}

impl Rect {
    pub(crate) fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= self.x - self.w && point.x <= self.x + self.w
            && point.y >= self.y - self.h && point.y <= self.y + self.h
    }

    pub fn intersects(&self, range: &Rect) -> bool {
        !(range.x - range.w > self.x + self.w ||
            range.x + range.w < self.x - self.w ||
            range.y - range.h > self.y + self.h ||
            range.y + range.h < self.y - self.h)
    }
}

impl Point {
    pub fn new(x: f32, y: f32, index: usize) -> Point { Point { x, y, index } }
}

impl QuadTree {
    pub(crate) fn new(boundary: Rect, capacity: usize) -> QuadTree {
        let vec = vec![];
        QuadTree {
            boundary,
            capacity,
            points: vec,
            subdivided: false,
            ne: None,
            nw: None,
            se: None,
            sw: None
        }
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        self.ne = Some(Box::new(QuadTree::new(Rect::new(x+w/2., y-h/2., w/2., h/2.), self.capacity)));
        self.nw = Some(Box::new(QuadTree::new(Rect::new(x-w/2., y-h/2., w/2., h/2.), self.capacity)));
        self.se = Some(Box::new(QuadTree::new(Rect::new(x+w/2., y+h/2., w/2., h/2.), self.capacity)));
        self.sw = Some(Box::new(QuadTree::new(Rect::new(x-w/2., y+h/2., w/2., h/2.), self.capacity)));

        for i in &mut self.points {
            self.ne.as_mut().unwrap().insert(i.clone());
            self.nw.as_mut().unwrap().insert(i.clone());
            self.se.as_mut().unwrap().insert(i.clone());
            self.sw.as_mut().unwrap().insert(i.clone());
        }
        self.points.clear();
    }

    pub fn insert(&mut self, point: Point) -> bool {

        if !self.boundary.contains(point) {
            return false;
        }

        if self.points.len() < self.capacity  && !self.subdivided {
            self.points.push(point);
            return true;
        } else {
            if !self.subdivided {
                self.subdivided = true;
                self.subdivide();
            }
            if self.ne.is_some() {
                if self.ne.as_mut().unwrap().insert(point) { return true; }
            }
            if self.nw.is_some() {
                if self.nw.as_mut().unwrap().insert(point) { return true; }
            }
            if self.se.is_some() {
                if self.se.as_mut().unwrap().insert(point) { return true; }
            }
            if self.sw.is_some() {
                if self.sw.as_mut().unwrap().insert(point) { return true; }
            }
        }
        false
    }

    pub(crate) fn query(&mut self, range: &Rect) -> Vec<Point> {
        let mut found: Vec<Point> = vec![];
        if !self.boundary.intersects(range) {
            return found;
        }else {
            for p in &self.points {
                if range.contains(p.clone()) {
                    found.push(p.clone());
                }
            }

            if self.subdivided {
                found.append(&mut self.ne.as_mut().unwrap().query(range));
                found.append(&mut self.nw.as_mut().unwrap().query(range));
                found.append(&mut self.se.as_mut().unwrap().query(range));
                found.append(&mut self.sw.as_mut().unwrap().query(range));
            }
        }
        found
    }

    //Optional test feature, allows the user to see the QuadTree
    pub(crate) fn show(&mut self) {
        draw_rectangle_lines(self.boundary.x - self.boundary.w, self.boundary.y - self.boundary.h,
                             self.boundary.w *2., self.boundary.h *2., 1., WHITE);

        for i in &self.points {
            draw_circle(i.x, i.y, 0.5, WHITE);
        }

        if self.subdivided {
            self.ne.as_mut().unwrap().show();
            self.nw.as_mut().unwrap().show();
            self.se.as_mut().unwrap().show();
            self.sw.as_mut().unwrap().show();
        }
    }
}

//PLEASE DELETE NON PERMINANT TEST STRUCT!

//NON PERMINANT TEST STRUCT: PARTICLE
#[derive(Clone, Copy)]
pub struct Particle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) r: f32,
    pub highlight: bool,
}

impl Particle {
    pub fn new(x: f32, y: f32, r: f32) -> Particle {
        Particle {x, y, r, highlight: false}
    }

    pub fn move_process(&mut self) {
        self.x += rand::gen_range(gen_range(-1.0, -0.1), 0.4);
        self.y += rand::gen_range(gen_range(-1.0, -0.1), 0.4);
    }

    pub fn render(&self) {
        if self.highlight {
            draw_circle(self.x, self.y, self.r+0.1, RED);
        }else {
            draw_circle(self.x, self.y, self.r, YELLOW);
        }
    }
    pub fn intersects(&self, other: &Particle) -> bool {
        let dist = distance_sqr(Vec2::new(self.x, self.y), Vec2::new(other.x, other.y));
        dist <= (self.r + other.r).powf(2.0)
    }

}


//Get the delta time
pub(crate) fn dt() -> f32 {
    get_frame_time()
}

//Convert meters into pixels
pub fn meter(meters: f32) -> f32 {
    meters * 100.0
}

//Convert a vec2 of pixels into meters
pub fn vec2_meter(input: Vec2) -> Vec2 {
    Vec2::new(input.x/meter(1.), input.y/meter(1.))
}

pub fn distance_sqr(p1: Vec2, p2: Vec2) -> f32 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    dx.powf(2.0) + dy.powf(2.0)
}