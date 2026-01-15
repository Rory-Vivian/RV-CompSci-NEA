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
#[allow(unused)]
use crate::measurements::{dt, QuadTree, Rect, Particle, Point};
use crate::measurements::meter;
use crate::objects::physics::PhysicsType::Static;

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
    // let circ = Circle::new(Vec2::new(1.0, 1.0), 0.1, ORANGE);
    // let sqr = Rectangle::new(Vec2::new(0.0, 0.0), 1.0, 1.0, WHITE);
    
    // let ball = Object::create(circ, 3.85, PhysicsType::Dynamic);
    // let rect = Object::create(sqr, 3.85, PhysicsType::Static);

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

    //TEST LIST PLEASE DELETE
    let mut particles: Vec<Particle> = Vec::new();
    
    let mut ui_id: String = String::from("");
    let mut ui_text_save: String = String::from("");
    
    let mut selected_object_index: Option<usize> = None;

    let boundary = Rect::new(0., 0., 100.0, 100.0);

    for _i in 0..500 {
        let p = Particle::new(rand::gen_range(boundary.x - boundary.w, boundary.x + boundary.w), rand::gen_range(boundary.y - boundary.h, boundary.y + boundary.h), rand::gen_range(1., 4.));
        particles.push(p);
    }

    let mut last_mouse_drag_pos: Option<Vec2> = None;
    let mut before_phys_type: Option<PhysicsType> = None;

    //Main loop function
    loop {

        // let mut corner1 = Vec2::new(particles[0].x,particles[0].y);
        // let mut corner2 = Vec2::new(particles[0].x,particles[0].y);

        clear_background(Color::from_rgba(30, 30, 30, 255));
        // set camera and produce the next frame
        set_camera(&camera);

        // for p in &particles {
        //     if p.x < corner1.x { corner1.x = p.x; }
        //     if p.y < corner1.y { corner1.y = p.y; }
        //     if p.x > corner2.x { corner2.x = p.x; }
        //     if p.y > corner2.y { corner2.y = p.y; }
        // }

        // let w = (corner2.x - corner1.x).abs();
        // let h = (corner2.y - corner1.y).abs();
        //
        // boundary.x = corner1.x + w/2.;
        // boundary.y = corner2.y - h/2.;
        // boundary.w = w/2.;
        // boundary.h = h/2.;

        // let mut qtree = QuadTree::new(boundary, 4);
        //
        // for p in 0..particles.len() {
        //     particles[p].highlight = false;
        //     qtree.insert(Point::new(particles[p].x, particles[p].y, p));
        // }
        //
        // for p in 0..particles.len() {
        //     let others = qtree.query(&Rect::new(particles[p].x, particles[p].y,
        //                                         particles[p].r*2., particles[p].r*2.));
        //     for o in 0..others.len() {
        //         if p != others[o].index {
        //             if particles[p].intersects(&particles[others[o].index]) {
        //                 particles[p].highlight = true;
        //                 particles[others[o].index].highlight = true;
        //             }
        //         }
        //     }
        // }

        // for i in &mut particles {
        //     i.move_process();
        //     i.render();
        // }

        //qtree.show();

        //Create the list of objects to render
        let mut render: Vec<Box<dyn Render + 'static>> = Vec::new();

        for i in 0..phys_object.len() {
            if phys_object.get_mut(i).is_none() {
                continue;
            }else if *phys_object.get_mut(i).unwrap().get_to_be_deleted() {
                phys_object.remove(i);
                if selected_object_index == Option::from(i) {
                    selected_object_index = None;
                }
            }else if let Some(n) = selected_object_index {
                if n == i {
                    phys_object.get_mut(i).unwrap().get_render_shape_reference().set_outline_colour(PURPLE);

                    let mouse_pos = camera.screen_to_world(Vec2::from(mouse_position()));
                    let j = phys_object.get_mut(i).unwrap();
                    if is_mouse_button_down(MouseButton::Left) && matches!(mouse_mode, MouseMode::Drag) &&
                        (mouse_position().0 < screen_width() - 400.) && i == selected_object_index.unwrap() {
                        if before_phys_type.is_none() {
                            before_phys_type = Some(j.get_physics_type().clone());
                        }

                        if last_mouse_drag_pos.is_some() {
                            *j.get_render_shape_reference().get_pos() = *j.get_render_shape_reference().get_pos() + (mouse_pos - last_mouse_drag_pos.unwrap()) / meter(1.);
                            j.set_physics_type(Static);
                            if pauorpla { j.set_velocity(Vec2::new(0.0, 0.0)); }
                        }

                        if last_mouse_drag_pos.is_some() {
                            last_mouse_drag_pos = Some(camera.screen_to_world(Vec2::from(mouse_position())));
                        } else if j.get_render_shape_reference().mouse_in_area(camera.screen_to_world(Vec2::from(mouse_position()))) {
                            last_mouse_drag_pos = Some(camera.screen_to_world(Vec2::from(mouse_position())));
                        }
                    } else {
                        last_mouse_drag_pos = None;
                        if before_phys_type.is_some() {
                            j.set_physics_type(before_phys_type.unwrap());
                            before_phys_type = None;
                        }
                    }
                }  else {
                    phys_object.get_mut(i).unwrap().get_render_shape_reference().set_outline_colour(BLACK);
                }
            }
        }
        //Physics function for all physics objects
        for (index, i) in &mut phys_object.iter_mut().enumerate() {
            render.push(i.get_render_shape());
            if pauorpla {
                i.physics_process(&camera);
            }
            if is_mouse_button_down(MouseButton::Left) && matches!(mouse_mode, MouseMode::Drag) &&
                ((mouse_position().0 < screen_width() - 400.) || selected_object_index.is_none())
                && i.get_render_shape_reference().mouse_in_area(camera.screen_to_world(Vec2::from(mouse_position())))
                && last_mouse_drag_pos.is_none() {
                    //Select the object the player has clicked on
                    selected_object_index = Some(index);
                    ui_id = "".into();
            }
        }
        // Finding if an object needs to be deleted, and then removing it from the nesasary places

        //Allow the user to unselect any objects they have selected
        if is_key_pressed(KeyCode::Escape) { selected_object_index = None; }

        //Build the UI, and render any objects hte player would like to use
        build_ui(&camera, &mut ui_id, &mut phys_object, selected_object_index, &mut ui_text_save);
        render_objects(&render);

        //Build the hotbar, and figure out if the software should close
        let (stop, clear) = build_hot_bar(&mut pauorpla, &mut mouse_mode);
        if stop {
            active = false;
        }
        
        if clear {
            selected_object_index = None;
            for _i in 0..phys_object.len() {
                phys_object.pop();
            }
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
        let mut added_object = false;

        match mouse_mode {
            MouseMode::Drag => {
                if (is_mouse_button_down(MouseButton::Left) && (selected_object_index.is_none() || mouse_position().0 < screen_width() - 400.)) && last_mouse_drag_pos.is_none() {
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
            added_object = true;
        }
        if let Some(rct) = rect {
            phys_object.push(Box::new(rct));
            added_object = true;
        }
        if let Some(crl) = ball {
            phys_object.push(Box::new(crl));
            added_object = true;
        }

        if added_object {
            mouse_mode = MouseMode::Drag;
            selected_object_index = Some(phys_object.len() - 1);
        }

        //If required exit the program, and then move onto the next frame
        if !active {
            request_quit();
        }

        // let _position = camera.world_to_screen(Vec2::new(10., 100.));
        // draw_text(get_fps().to_string().as_str(), -200., -200., 10., WHITE);

        next_frame().await;
    }
}
