use macroquad::miniquad::window::{request_quit};
//External libraries
use macroquad::prelude::*;

//Internal Modules
mod objects;
mod measurements;
mod uis;

use objects::*;
use measurements::*;
use crate::objects::shapes::{Rectangle, Circle};
use crate::objects::physics::{PhysicsObeject, PhysicsType};
use uis::{build_hot_bar};
use crate::uis::build_ui;

fn conf() -> Conf {
    Conf {
        window_title: "Rory Vivian Computer Science NEA".parse().unwrap(),
        fullscreen: true,
        ..Default::default()
    }
}

//Main function called by macroquad as to allow the program to render.
#[macroquad::main(conf)]
async fn main() {
    let mut active = true;
    //Infinite loop
    let circ = Circle::new(Vec2::new(1.0, 1.0), 0.1, ORANGE);
    let sqr = Rectangle::new(Vec2::new(0.0, 0.0), 1.0, 1.0, WHITE);

    let mut ball = Object::create(circ, 3.85, PhysicsType::Dynamic);
    let mut rect = Object::create(sqr, 3.85, PhysicsType::Static);

    let mut camera = Camera2D{
        zoom: Vec2::new(0.05,0.05),
        ..Default::default()
    };
    set_camera(&camera);
    let mut zoom: f32 = 100.;
    camera.target = Vec2::new(0.,0.);
    let mut world_mouse_before: Option<Vec2> = None;

    loop {
        clear_background(Color::from_rgba(30,30,30,255));

        if is_key_down(KeyCode::Up) {ball.dy -= 15.0*dt()}
        if is_key_down(KeyCode::Right) {ball.dx += 3.0*dt()}
        if is_key_down(KeyCode::Left) {ball.dx -= 3.0*dt()}

        ball.physics_process();

        let mut render: Vec<Box<dyn Render>> = Vec::new();
        
        render.push(rect.shape.clone());
        render.push(ball.shape.clone());

        render_objects(&render);

        build_ui(&mut zoom);
        if build_hot_bar() {
            active = false;
        }
        // change the level of the cameras zoom
        camera.zoom = Vec2::new(zoom/(10.0 * screen_width()), zoom/(10.0 * screen_height()));
        let scroll = mouse_wheel();
        if scroll.1 != 0. {
            let mouse_before = camera.screen_to_world(Vec2::from(mouse_position()));
            zoom += scroll.1/100.;
            camera.target = Vec2::from(mouse_position());
            camera.zoom = Vec2::new(zoom/(10.0 * screen_width()), zoom/(10.0 * screen_height()));
            let mouse_after = camera.screen_to_world(Vec2::from(mouse_position()));
            let offset = mouse_after - mouse_before;
            camera.target -= offset;
        }

        if is_mouse_button_down(MouseButton::Left) && is_key_down(KeyCode::Space) {
            let world_mouse_after = camera.screen_to_world(Vec2::from(mouse_position()));
            if let Some(last_pos) = world_mouse_before {
                let offset = world_mouse_after - last_pos;
                camera.target -= offset;
            }
            world_mouse_before = Some(world_mouse_after);
        }else {
            world_mouse_before = None;
        }


        if !active {
            request_quit();
        }

        // set camera and produce the next frame
        set_camera(&camera);
        next_frame().await;
    };
}