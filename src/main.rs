//External libraries
use macroquad::prelude::*;

//Internal Modules
mod objects;
mod measurements;
use objects::*;
use measurements::*;
use crate::objects::shapes::{Rectangle, Square, Circle};
use crate::objects::physics::{PhysicsObeject, PhysicsType};

//Main function called by macroquad as to allow the program to render.
#[macroquad::main("Rory Vivian Computer Science NEA")]
async fn main() {
    //Infinite loop
    let square = Square::new(Vec2::new(0.0,0.0), meter(5.0), BLUE);
    let rect = Rectangle::new(Vec2::new(meter(20.0), meter(20.0)), meter(1.0), meter(2.5), PURPLE);
    let circ = Circle::new(Vec2::new(meter(10.0), meter(10.0)), meter(1.0), ORANGE);

    let mut ball = Object::create(circ, 8.5, PhysicsType::Dynamic);
    let mut metal_square = Object::create(square, 8.5, PhysicsType::Static);
    let mut wooden_square = Object::create(rect, 8.5, PhysicsType::Static);

    ball.dy = -meter(10.0);
    ball.dx = meter(1.0);
    loop {
        //Clear the background and make the screen black
        clear_background(BLACK);

        //creat a list of rendered object

        if is_key_down(KeyCode::Up) {ball.dy -= mps(20.0)}
        if is_key_down(KeyCode::Right) {ball.dx += mps(20.0)}
        if is_key_down(KeyCode::Left) {ball.dx -= mps(20.0)}

        ball.physics_process();

        let mut render: Vec<Box<dyn Render>> = Vec::new();

        render.push(metal_square.shape.clone());
        render.push(wooden_square.shape.clone());
        render.push(ball.shape.clone());

        render_objects(&render);

        next_frame().await;
    };
}