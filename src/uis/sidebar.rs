use macroquad::color::{Color, BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW};
use macroquad::math::RectOffset;
use macroquad::prelude::Vec2;
use macroquad::ui::*;
use macroquad::window::{screen_height, screen_width};
//use crate::objects::Object;
use crate::objects::physics::{PhysicsObeject, PhysicsType};

fn create_types_drop(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize, ui_id: &mut String) {
    let mut x = false;
    match objects.get_mut(selected_index).unwrap().get_physics_type() {
        PhysicsType::Static => {
            if ui.button(None, "Static  V") {
                x = true;
            }
        }
        PhysicsType::Kinematic => {
            if ui.button(None, "Kinematic V") {
                x = true;
            }
        }
        PhysicsType::Dynamic => {
            if ui.button(None, "Dynamic V") {
                x = true;
            }
        }
    }
    if x && *ui_id == "dropdown_types" {
        *ui_id = "".parse().unwrap();
    } else if x {
        *ui_id = "dropdown_types".parse().unwrap();
    }
    if *ui_id == "dropdown_types" {
        if ui.button(None, "Static") {
            objects.get_mut(selected_index).unwrap().set_physics_type(PhysicsType::Static);
            *ui_id = "".parse().unwrap();
        }
        if ui.button(None, "Kinematic") {
            objects.get_mut(selected_index).unwrap().set_physics_type(PhysicsType::Kinematic);
            *ui_id = "".parse().unwrap();
        }
        if ui.button(None, "Dynamic") {
            objects.get_mut(selected_index).unwrap().set_physics_type(PhysicsType::Dynamic);
            *ui_id = "".parse().unwrap();
        }
    }
}

fn create_colour_button_skin(color: Color, root_ui: &mut Ui) -> Style{
    return root_ui
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        .color(color)
        .color_inactive(color)
        .color_hovered(color)
        .color_clicked(color)
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .build();
}

fn create_colour_buttons(ui: &mut Ui, colour_button_style: Style, defult_skin: &mut Skin, ui_id: &mut String) -> Color {
    //let color = object.get_render_shape().get_colour();
    let hold_style = defult_skin.button_style.clone();
    defult_skin.button_style = colour_button_style;
    let _skin_hold = ui.push_skin(&defult_skin);
    if ui.button(None, "           ") {
        if *ui_id == "colour_dropdown_options" {
            *ui_id = "".parse().unwrap();
        }else {
            *ui_id = "colour_dropdown_options".parse().unwrap();
        }
    }
    
    let colours = vec!(RED, ORANGE, YELLOW, GREEN, BLUE, PURPLE, WHITE, BLACK);
    
    if *ui_id == "colour_dropdown_options" {
        for i in colours {
            let colour_button_red = create_colour_button_skin(i, ui);
            defult_skin.button_style = colour_button_red;
            let _skin_hold = ui.push_skin(defult_skin);
            if ui.button(None, "") {
                defult_skin.button_style = hold_style;
                *ui_id = "".parse().unwrap();
                return i;
            }
            ui.same_line(0.);
        }
    }
    defult_skin.button_style = hold_style;
    return Color::new(1., 1., 1., 255.)
}


pub(crate) fn create_side_bar(ui_id: &mut String, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize) {
    let window_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(46, 46, 46, 255))
        .color_inactive(Color::from_rgba(46, 46, 46, 255))
        .text_color(WHITE)
        .build();

    let button_style = root_ui()
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

    let mut skin = Skin {
        window_style,
        button_style,
        ..root_ui().default_skin()
    };

    let _skin_hold = root_ui().push_skin(&skin);
    
    let colour = objects.get_mut(selected_index).unwrap().get_render_shape().get_colour();
    let colour_button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        .color(colour)
        .color_inactive(colour)
        .color_hovered(colour)
        .color_clicked(colour)
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .build();
    
    root_ui().window(
        hash!(),
        Vec2::new((screen_width() * 2.) - 60., 40.),
        Vec2::new(screen_width(), screen_height()),
        |ui| {
            create_types_drop(ui, objects, selected_index, ui_id);
            let colour_option = create_colour_buttons(ui, colour_button_style, &mut skin, ui_id);
            if colour_option != Color::new(1., 1., 1., 255.) {
                objects.get_mut(selected_index).unwrap().get_render_shape_referance().set_colour(colour_option);
            }
        },
    );
}
