#![allow(unused, dead_code)]

use macroquad::input::mouse_position;
use macroquad::prelude::camera::mouse;
use macroquad::prelude::scene::camera_pos;
use macroquad::prelude::*;

use crate::MouseMode;
use crate::measurements::vec2_meter;
use crate::objects::physics::{Material, PhysicsType};
use crate::objects::shapes::{Circle, Rectangle, Square};
use crate::objects::{self, Object, Render};

//Created the rendered shape for a Square
fn create_square_render(pos1: Vec2, pos2: Vec2, colour: Color) -> Rectangle {
    //The top corner of the shape
    let pos1 = vec2_meter(pos1);
    //The bottom corner of the shape
    let pos2 = vec2_meter(pos2);

    //The length in the x and y
    let dx: f32 = (pos2.x - pos1.x);
    let dy: f32 = (pos2.y - pos1.y);

    //The largest of the two lengths
    let length = dx.abs().max(dy.abs());
    //Calculate the length on each side
    let x: f32;
    let y: f32;
    if pos2.x >= pos1.x {
        x = length.abs();
    } else {
        x = -length.abs();
    }
    if pos2.y >= pos1.y {
        y = length.abs();
    } else {
        y = -length.abs();
    }

    //Create a rectangle with the specified dimensions
    Rectangle::new(pos1, x, y, colour)
}

//Create the render shape of a rectangle
fn create_rectangle_render(pos1: Vec2, pos2: Vec2, colour: Color) -> Rectangle {
    //The top and bottom corners of the rectangle
    let pos1 = vec2_meter(pos1);
    let pos2 = vec2_meter(pos2);

    //The length in the x and the y of the shape
    let dx: f32 = (pos2.x - pos1.x);
    let dy: f32 = (pos2.y - pos1.y);

    //Function to create the Render shape for the rectangle
    Rectangle::new(pos1, dx, dy, colour)
}

//Create the render shape of a ball
fn create_ball_render(pos1: Vec2, pos2: Vec2, colour: Color) -> Circle {
    //Center and radius positions of the mouse
    let pos1 = vec2_meter(pos1);
    let pos2 = vec2_meter(pos2);

    //Calculate the radius of the circle, from the center position
    let dx: f32 = (pos2.x - pos1.x).abs();
    let dy: f32 = (pos2.y - pos1.y).abs();
    let r = dx.max(dy);

    //Create the circle render
    Circle::new(pos1, r, colour)
}

//Create a square object, by creating the render shape, and the material for the object
fn create_square(pos_1: Vec2, pos_2: Vec2) -> Object<Rectangle> {
    let square = create_square_render(pos_1, pos_2, WHITE);
    let material = Material::new(square.get_area() * 0.98, square.get_area());
    Object::new(square, material, PhysicsType::Static)
}

//Create a rectangle, by creating the render shape, and the material for the object
fn create_rectangle(pos1: Vec2, pos2: Vec2) -> Object<Rectangle> {
    let rect = create_rectangle_render(pos1, pos2, WHITE);
    let material = Material::new(rect.get_area() * 0.89, rect.get_area());
    Object::new(rect, material, PhysicsType::Static)
}

//Create a ball, by creating the render shape, and the material for the object
fn create_ball(pos1: Vec2, pos2: Vec2) -> Object<Circle> {
    let circle = create_ball_render(pos1, pos2, WHITE);
    let material = Material::new(circle.get_area() * 0.98, circle.get_area());
    Object::new(circle, material, PhysicsType::Static)
}

//Create the object the user would like to produce
pub fn draw_process_square(first_mouse_pos: &mut Option<Vec2>, camera: &Camera2D) -> Option<Object<Rectangle>> {
    if draw_process(MouseMode::DrawSquare, first_mouse_pos, camera) {
        //Use the mouse position saved
        if let Some(pos1) = *first_mouse_pos {
            //Create an object from the two mouse positions
            let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
            let square = create_square(pos1, pos2);
            //Clear the saved mouse position
            *first_mouse_pos = None;
            //Return the object back to the function
            return Some(square);
        }
    }
    None
}

//Create the object the user would like to produce
pub fn draw_process_rectangle(first_mouse_pos: &mut Option<Vec2>, camera: &Camera2D) -> Option<Object<Rectangle>> {
    if draw_process(MouseMode::DrawRectangele, first_mouse_pos, camera) {
        //Use the mouse position saved
        if let Some(pos1) = *first_mouse_pos {
            //Create an object from the two mouse positions
            let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
            let rectangle = create_rectangle(pos1, pos2);
            //Clear the saved mouse position
            *first_mouse_pos = None;
            //Return the object back to the function
            return Some(rectangle);
        }
    }
    None
}

//Create the object the user would like to produce
pub fn draw_process_ball(first_mouse_pos: &mut Option<Vec2>, camera: &Camera2D) -> Option<Object<Circle>> {
    if draw_process(MouseMode::DrawBall, first_mouse_pos, camera) {
        //Used the mouse position saved
        if let Some(pos1) = *first_mouse_pos {
            //Create the object from the 2 mouse positons
            let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
            let ball = create_ball(pos1, pos2);
            //Clear the saved mouse position
            *first_mouse_pos = None;
            //Return the object back to the function
            return Some(ball);
        }
    }
    None
}

//Draw the shape of the object that the user would like to draw in preview mode
pub fn draw_process(mouse_mode: MouseMode, first_mouse_pos: &mut Option<Vec2>, camera: &Camera2D) -> bool {
    //Find the mouse mode the user is using.
    match mouse_mode {
        //The user is drawing a square
        MouseMode::DrawSquare => {
            if is_mouse_button_down(MouseButton::Left) {
                //Get the current mouse position, or save the first mouse position the usr has used
                let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
                if let Some(pos1) = *first_mouse_pos {
                    //Render the square between these points, and make the colour purple as to show it's highlighted
                    let mut square = create_square_render(pos1, pos2, PURPLE);
                    //Force the shape to render
                    square.render();
                } else {
                    *first_mouse_pos = Some(pos2);
                }
            } else {
                //Return true, as to signify that the user needs to make the mouse object, or clear the saved position
                if let Some(pos1) = *first_mouse_pos {
                    return true;
                }
                *first_mouse_pos = None;
            }
        }
        //The user is drawing a rectangle
        MouseMode::DrawRectangele => {
            if is_mouse_button_down(MouseButton::Left) {
                //Get the current mouse position, or save the current mouse position the user has used
                let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
                if let Some(pos1) = *first_mouse_pos {
                    //Create a render object between the two points the user has specified
                    let mut rectangle = create_rectangle_render(pos1, pos2, PURPLE);
                    //Force this object to render
                    rectangle.render();
                } else {
                    //Save the original mouse position when drawing a new object
                    *first_mouse_pos = Some(pos2);
                }
            } else {
                //Return true if the user has finished drawing there shape, or clear the first mouse position
                if let Some(pos1) = *first_mouse_pos {
                    return true;
                }
                *first_mouse_pos = None;
            }
        }
        //The user is drawing a ball
        MouseMode::DrawBall => {
            if is_mouse_button_down(MouseButton::Left) {
                //Get the current mouse position, or save the mouse position the user has used
                let pos2 = camera.screen_to_world(Vec2::from(mouse_position()));
                if let Some(pos1) = *first_mouse_pos {
                    //Create a render object for the shape the user would like to draw
                    let mut ball = create_ball_render(pos1, pos2, PURPLE);
                    //Force said object to render
                    ball.render();
                } else {
                    //Save the original mouse position the user would like to use
                    *first_mouse_pos = Some(pos2);
                }
            } else {
                //Return ture if the user has finished drawing, or clear the original value
                if let Some(pos1) = *first_mouse_pos {
                    return true;
                }
                *first_mouse_pos = None;
            }
        }
        //Do nothing if the mouse_mode is something else
        _ => {}
    }
    //Return false as a default
    false
}
