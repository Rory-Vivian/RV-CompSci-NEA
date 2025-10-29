use macroquad::miniquad::window::request_quit;
//External libraries
use macroquad::prelude::*;

//Internal Modules
mod measurements;
mod objects;
mod uis;

use crate::objects::create_objects::{
    draw_porcess_ball, draw_porcess_rectangele, draw_process_square,
};
use crate::objects::physics::{PhysicsObeject, PhysicsType};
use crate::objects::shapes::{Circle, Rectangle};
use crate::uis::build_ui;
//use measurements::*;
use objects::*;
use uis::build_hot_bar;

#[derive(Clone)]
#[allow(unused)]
enum MouseMode {
    Drag,
    DrawSquare,
    DrawRectangele,
    DrawBall,
}

fn conf() -> Conf {
    Conf {
        window_title: "Rory Vivian Computer Science NEA".parse().unwrap(),
        fullscreen: true,
        ..Default::default()
    }
}

pub fn cam_to_world(input: Vec2, camera: &Camera2D) -> Vec2 {
    //zoom coeficient
    let co_x = camera.zoom.x * (screen_width() * 10.);
    let co_y = camera.zoom.y * (screen_height() * 10.);

    //create the return
    let mut output = Vec2::new(0., 0.);
    output.x = input.x / co_x;
    output.y = input.y / co_y;

    //retrun the new vector
    output
}

//Main function called by macroquad as to allow the program to render.
#[macroquad::main(conf)]
async fn main() {
    let mut active = true;
    let mut mouse_mode = MouseMode::Drag;
    //Infinite loop
    let circ = Circle::new(Vec2::new(1.0, 1.0), 0.1, ORANGE);
    let sqr = Rectangle::new(Vec2::new(0.0, 0.0), 1.0, 1.0, WHITE);
    
    let mut ball = Object::create(circ, 3.85, PhysicsType::Dynamic);
    let mut rect = Object::create(sqr, 3.85, PhysicsType::Static);

    let mut camera = Camera2D {
        zoom: Vec2::new(0.05, 0.05),
        ..Default::default()
    };
    set_camera(&camera);
    let mut zoom: f32 = 100.;
    camera.target = Vec2::new(0., 0.);
    let mut world_mouse_before: Option<Vec2> = None;
    // pause or play the program
    let mut pauorpla = false;
    let mut draw_mouse_storage: Option<Vec2> = None;

    let mut phys_object: Vec<Box<dyn PhysicsObeject>> = Vec::new();
    
    let mut ui_id: String = String::from("");

    loop {
        clear_background(Color::from_rgba(30, 30, 30, 255));
        // set camera and produce the next frame
        set_camera(&camera);
        if pauorpla {
            ball.physics_process();
        }

        let mut render: Vec<Box<dyn Render + 'static>> = Vec::new();

        render.push(rect.shape.clone_box());
        render.push(ball.shape.clone_box());
        for i in &mut phys_object {
            let x = i.get_render_shape();
            render.push(x);
        }
        
        build_ui(&mut zoom, &camera, &mut ui_id, &mut Some(Box::new(rect.clone())));

        if build_hot_bar(&mut pauorpla, &mut mouse_mode) {
            active = false;
        }
        // change the level of the cameras zoom
        camera.zoom = Vec2::new(
            zoom / (10.0 * screen_width()),
            zoom / (10.0 * screen_height()),
        );
        let scroll = mouse_wheel();

        if scroll.1 != 0. || is_key_down(KeyCode::Up) || is_key_down(KeyCode::Down) {
            let mouse_before = camera.screen_to_world(Vec2::from(mouse_position()));

            if scroll.1 != 0. {
                zoom += scroll.1 / 100.;
            } else {
                if is_key_down(KeyCode::Up) {
                    zoom += 1.
                } else {
                    zoom -= 1.
                }
            }
            zoom = zoom.clamp(10., 200.);

            camera.target = Vec2::from(mouse_position());
            camera.zoom = Vec2::new(
                zoom / (10.0 * screen_width()),
                zoom / (10.0 * screen_height()),
            );

            let mouse_after = camera.screen_to_world(Vec2::from(mouse_position()));
            let offset = mouse_after - mouse_before;

            camera.target -= offset;
        }

        let mut square: Option<Object<Rectangle>> = None;
        let mut rect: Option<Object<Rectangle>> = None;
        let mut ball: Option<Object<Circle>> = None;

        render_objects(&render);
        
        match mouse_mode {
            MouseMode::Drag => {
                if is_mouse_button_down(MouseButton::Left) {
                    let world_mouse_after = Vec2::from(mouse_position());
                    if let Some(last_pos) = world_mouse_before {
                        let offset = world_mouse_after - last_pos;
                        camera.target -= offset / (camera.zoom) / 1000.;
                    }
                    world_mouse_before = Some(world_mouse_after);
                } else {
                    world_mouse_before = None;
                }
            }
            MouseMode::DrawSquare => {
                square = draw_process_square(&mut draw_mouse_storage, &camera);
            }
            MouseMode::DrawRectangele => {
                rect = draw_porcess_rectangele(&mut draw_mouse_storage, &camera);
            }
            MouseMode::DrawBall => {
                ball = draw_porcess_ball(&mut draw_mouse_storage, &camera);
            } //_ => {}
        }
        if let Some(sqr) = square {
            phys_object.push(Box::new(sqr));
        }
        if let Some(rct) = rect {
            phys_object.push(Box::new(rct));
        }
        if let Some(crl) = ball {
            phys_object.push(Box::new(crl));
        }

        if !active {
            request_quit();
        }

        next_frame().await;
    }
}
