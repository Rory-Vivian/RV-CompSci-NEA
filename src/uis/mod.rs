use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{Ui, Skin, Style, root_ui};

use crate::MouseMode;
use crate::objects::physics::PhysicsObject;
mod guidlines;
mod sidebar;

//Function to produce a button that changes colour based on a variable
fn active_button(ui: &mut Ui,is_active: bool, active: &Style, mut inactive: Skin, label: &'static str) -> bool {
    let mut ret_val = false;
    //Check if the button should appear as active
    if is_active {
        //Create the skin style for the active button
        inactive.button_style = active.clone();
        ui.push_skin(&inactive);
        //Create the button and return the value of the press
        if ui.button(None, label) {
            ret_val = true;
        }
        ui.pop_skin();
    }else {
        //Use the default skin for the button
        if ui.button(None, label) { 
            ret_val = true;
        }
    }
    ret_val
}

//Build the hotbar for the UI
pub(crate) fn build_hot_bar(simulate: &mut bool, mouse_mode: &mut MouseMode) -> (bool, bool) {
    let mut self_return = false;
    //Use the defult bar style for the whole of the project
    let bar_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(36, 36, 36, 255))
        .color_inactive(Color::from_rgba(36, 36, 36, 255))
        .text_color(WHITE)
        .build();

    //Use the defult button style for the whole of the project
    let button_bar = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        .color(Color::from_rgba(42, 42, 42, 255))
        .color_inactive(Color::from_rgba(42, 42, 42, 255))
        .color_hovered(Color::from_rgba(85, 85, 85, 255))
        .color_clicked(PURPLE)
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .build();

    //Build the button active skin for this bar
    let button_bar_active = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        .color(PURPLE)
        .color_inactive(PURPLE)
        .color_hovered(Color::from_rgba(143, 2, 244, 255))
        .color_clicked(WHITE)
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .build();

    //Create the bar skin
    let bar_skin = Skin {
        window_style: bar_style,
        button_style: button_bar,
        ..root_ui().default_skin()
    };

    //Construct the top selection bar
    let mut return_2 = false;
    root_ui().push_skin(&bar_skin);
    root_ui().window(
        hash!(),
        Vec2::new(0., 0.),
        Vec2::new(screen_width(), 40.),
        |ui| {
            //Use an active button for the mouse mode being drawing a square
            if active_button(ui, matches!(mouse_mode, MouseMode::DrawSquare), &button_bar_active, bar_skin.clone(), "Square") {
                if matches!(mouse_mode, MouseMode::DrawSquare) { *mouse_mode = MouseMode::Drag; }
                else { *mouse_mode = MouseMode::DrawSquare; }
            }
            ui.same_line(0.0);
            //Use an active button for the mouse mode being for drawing a rectangle
            if active_button(ui, matches!(mouse_mode, MouseMode::DrawRectangele), &button_bar_active, bar_skin.clone(), "Rectangle") {
                if matches!(mouse_mode, MouseMode::DrawRectangele) { *mouse_mode = MouseMode::Drag; }
                else { *mouse_mode = MouseMode::DrawRectangele; }
            }
            ui.same_line(0.0);
            //Use an active button for the mouse mode being for drawing a ball
            if active_button(ui, matches!(mouse_mode, MouseMode::DrawBall), &button_bar_active, bar_skin.clone(), "Ball") {
                if matches!(mouse_mode, MouseMode::DrawBall) { *mouse_mode = MouseMode::Drag; }
                else { *mouse_mode = MouseMode::DrawBall; }
            }
            ui.same_line(0.0);
            //Check the game should be simulating the game, and display the pause/play button accordingly
            if *simulate {
                if ui.button(None, "pause") {
                    *simulate = false;
                }
            } else if ui.button(None, "play") {
                *simulate = true;
            }
            //Check if the user would like to exit the program
            ui.same_line(0.0);
            if ui.button(None, "esc") {
                self_return = true;
            }
            ui.same_line(0.0);
            return_2 = if ui.button(None, "clear") { true } else { false }
        },
    );

    root_ui().pop_skin();
    (self_return, return_2)
}

//Construct the UI from the build_ui function
pub fn build_ui(camera: &Camera2D, ui_id: &mut String,
                objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: Option<usize>, ui_save_text: &mut String) {
    guidlines::draw_guidelines(camera);
    //Make sure an object is selected, and then allow the sidebar to be created
    if let Some(selected_object_index) = selected_index {
        sidebar::create_side_bar(ui_id, objects, selected_object_index, ui_save_text);
    }
}
