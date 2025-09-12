use std::sync::atomic::{AtomicU32, Ordering};
//External libraries
use crate::uis::build_ui;
use macroquad::prelude::*;

//Internal Modules
mod objects;
mod measurements;
mod uis;

use objects::*;
use measurements::*;
use crate::objects::shapes::{Rectangle, Square, Circle};
use crate::objects::physics::{PhysicsObeject, PhysicsType};

static ZOOM: AtomicU32 = AtomicU32::new(100);

//Main function called by macroquad as to allow the program to render.
#[macroquad::main("Rory Vivian Computer Science NEA")]
async fn main() {
    //Infinite loop
    let circ = Circle::new(Vec2::new(1.0, 1.0), 0.1, ORANGE);
    let sqr = Rectangle::new(Vec2::new(1.0, 1.0), 1.0, 1.0, WHITE);

    let mut ball = Object::create(circ, 3.85, PhysicsType::Dynamic);
    let mut rect = Object::create(sqr, 3.85, PhysicsType::Static);

    //ball.dy = -10.0;
    //ball.dx = 1.0;

    loop {
        //Clear the background and make the screen black
        clear_background(Color::from_rgba(30,30,30,255));

        //creat a list of rendered object

        if is_key_down(KeyCode::Up) {ball.dy -= 15.0*dt()}
        if is_key_down(KeyCode::Right) {ball.dx += 3.0*dt()}
        if is_key_down(KeyCode::Left) {ball.dx -= 3.0*dt()}
        
        let scrolling = mouse_wheel();
        if scrolling.1 != 0.0 {
            if scrolling.1 > 0.0 {
                ZOOM.fetch_add(1, Ordering::Relaxed);
            }else {
                ZOOM.fetch_sub(1, Ordering::Relaxed);
            }
        }

        ball.physics_process();

        let mut render: Vec<Box<dyn Render>> = Vec::new();
        
        render.push(rect.shape.clone());
        render.push(ball.shape.clone());

        render_objects(&render);

        let mut zoom: f32 = ZOOM.load(Ordering::SeqCst) as f32;
        build_ui(&mut zoom);
        ZOOM.store(zoom as u32, Ordering::SeqCst);

        next_frame().await;
    };
}