use macroquad::prelude::*;
mod objects;
use objects::*;
use crate::objects::square::{Rectangle, Square};

#[macroquad::main("MyGame")]
async fn main() {
    //Infinite loop
    loop {
        clear_background(BLACK);

        let mut render: Vec<Box<dyn Render>> = Vec::new();
        let square = Square::new(Vec2::new(0.0,0.0), 100.0, BLUE);
        let rect = Rectangle::new(Vec2::new(800.0, 800.0), 20.0, 50.0, BLUE);
        render.push(Box::new(square));
        render.push(Box::new(rect));

        render_objects(render);

        next_frame().await;
    };

}