use macroquad::miniquad::window::request_quit;
//External libraries
use macroquad::prelude::*;

//Internal Modules
mod measurements;
mod objects;
mod uis;

use crate::objects::create_objects::{
    draw_process_ball, draw_process_rectangle, draw_process_square,
};
use crate::objects::physics::{PhysicsObject, PhysicsType};
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

// Set up the config for the project window
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
    //Basic and required variables
    let mut active = true;
    let mut mouse_mode = MouseMode::Drag;

    //Create shape for testing (PLEASE REMOVE BEFORE FINAL BUILD)
    let circ = Circle::new(Vec2::new(1.0, 1.0), 0.1, ORANGE);
    let sqr = Rectangle::new(Vec2::new(0.0, 0.0), 1.0, 1.0, WHITE);
    
    let ball = Object::create(circ, 3.85, PhysicsType::Dynamic);
    let rect = Object::create(sqr, 3.85, PhysicsType::Static);

    //Create the camera for the project
    let mut camera = Camera2D {
        zoom: Vec2::new(0.05, 0.05),
        ..Default::default()
    };
    set_camera(&camera);
    let mut zoom: f32 = 100.;
    camera.target = Vec2::new(0., 0.);

    //Save the mouse position at all times
    let mut world_mouse_before: Option<Vec2> = None;
    // pause or play the program
    let mut pauorpla = false;
    let mut draw_mouse_storage: Option<Vec2> = None;

    //Create a list of all physics objects
    let mut phys_object: Vec<Box<dyn PhysicsObject>> = Vec::new();
    
    let mut ui_id: String = String::from("");
    let mut ui_text_save: String = String::from("");
    
    let mut selected_object_index: Option<usize> = None;
    //Simulate the test objects (PLEASE REMOVE BEFORE FINAL BUILD)
    phys_object.push(Box::new(rect));
    phys_object.push(Box::new(ball));

    //Main loop function
    loop {
        clear_background(Color::from_rgba(30, 30, 30, 255));
        // set camera and produce the next frame
        set_camera(&camera);

        //Create the list of objects to render
        let mut render: Vec<Box<dyn Render + 'static>> = Vec::new();

        //Physics function for all physics objects
        for (index, i) in &mut phys_object.iter_mut().enumerate() {
            render.push(i.get_render_shape());
            if pauorpla {
                i.physics_process(&camera);
            }
            if is_mouse_button_down(MouseButton::Left) && matches!(mouse_mode, MouseMode::Drag) && 
                ((mouse_position().0 < screen_width() - 400.) || selected_object_index.is_none()) 
                && i.get_render_shape_reference().mouse_in_area(camera.screen_to_world(Vec2::from(mouse_position()))) {
                    //Select the object the player has clicked on
                    selected_object_index = Some(index);
                    ui_id = "".into();
            }
        }
        // Finding if an object needs to be deleted, and then removing it from the nesasary places
        for i in 0..phys_object.len() {
            if phys_object.get_mut(i).is_none() {
                continue;
            }else if *phys_object.get_mut(i).unwrap().get_to_be_deleted() {
                phys_object.remove(i);
                if selected_object_index == Option::from(i) {
                    selected_object_index = None;
                }
            }
        }

        //Allow the user to unselect any objects they have selected
        if is_key_pressed(KeyCode::Escape) { selected_object_index = None; }

        //Build the UI, and render any objects hte player would like to use
        build_ui(&camera, &mut ui_id, &mut phys_object, selected_object_index, &mut ui_text_save);
        render_objects(&render);

        //Build the hotbar, and figure out if the software should close
        if build_hot_bar(&mut pauorpla, &mut mouse_mode) {
            active = false;
        }

        //Change the level of the cameras zoom
        camera.zoom = Vec2::new(
            zoom / (10.0 * screen_width()),
            zoom / (10.0 * screen_height()),
        );
        let scroll = mouse_wheel();

        if scroll.1 != 0. || is_key_down(KeyCode::Up) || is_key_down(KeyCode::Down) {
            let mouse_before = camera.screen_to_world(Vec2::from(mouse_position()));

            if scroll.1 != 0. {
                zoom += scroll.1 / 100.;
            } else if is_key_down(KeyCode::Up) {
                zoom += 1.
            } else {
                zoom -= 1.
            }
            
            zoom = zoom.clamp(10., 200.);

            camera.target = Vec2::from(mouse_position());
            camera.zoom = Vec2::new(
                zoom / (10.0 * screen_width()),
                zoom / (10.0 * screen_height()),
            );

            //Calculate the offset created by zooming centred to the mouse position.
            let mouse_after = camera.screen_to_world(Vec2::from(mouse_position()));
            let offset = mouse_after - mouse_before;

            camera.target -= offset;
        }

        //Create variables to store the potential outcome of drawing.
        let mut square: Option<Object<Rectangle>> = None;
        let mut rect: Option<Object<Rectangle>> = None;
        let mut ball: Option<Object<Circle>> = None;

        match mouse_mode {
            MouseMode::Drag => {
                if is_mouse_button_down(MouseButton::Left) && (selected_object_index.is_none() || mouse_position().0 < screen_width() - 400.) {
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
            //Run the process to draw a square rectangle or circle
            MouseMode::DrawSquare => {
                square = draw_process_square(&mut draw_mouse_storage, &camera);
            }
            MouseMode::DrawRectangele => {
                rect = draw_process_rectangle(&mut draw_mouse_storage, &camera);
            }
            MouseMode::DrawBall => {
                ball = draw_process_ball(&mut draw_mouse_storage, &camera);
            } //_ => {}
        }
        //Push the square circle or rectangle into the physics objects list
        if let Some(sqr) = square {
            phys_object.push(Box::new(sqr));
        }
        if let Some(rct) = rect {
            phys_object.push(Box::new(rct));
        }
        if let Some(crl) = ball {
            phys_object.push(Box::new(crl));
        }

        //If required exit the program, and then move onto the next frame
        if !active {
            request_quit();
        }
        
        next_frame().await;
    }
}
