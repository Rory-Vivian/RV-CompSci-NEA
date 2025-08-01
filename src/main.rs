//External libraries
use macroquad::prelude::*;

//Internal Modules
mod objects;
use objects::*;
use crate::objects::shapes::{Rectangle, Square, Circle};

//Main function called by macroquad as to allow the program to render.
#[macroquad::main("MyGame")]
async fn main() {
    //Infinite loop
    let square = Square::new(Vec2::new(0.0,0.0), 100.0, BLUE);
    let rect = Rectangle::new(Vec2::new(400.0, 400.0), 20.0, 50.0, PURPLE);
    let circ = Circle::new(Vec2::new(200.0, 200.0), 300.0, ORANGE);

    let mut ball = Object::create(circ, 8.5);
    let mut metal_square = Object::create(square, 8.5);
    let mut wooden_square = Object::create(rect, 8.5);


    loop {
        //Clear the background and make the screen black
        clear_background(BLACK);

        //creat a list of rendered object
        ball.dx += 0.05;

        ball.physics_process();

        let mut render: Vec<Box<dyn Render>> = Vec::new();

        render.push(metal_square.shape.clone());
        render.push(wooden_square.shape.clone());
        render.push(ball.shape.clone());

        render_objects(&render);

        next_frame().await;
    };
}