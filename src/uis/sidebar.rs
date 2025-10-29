use macroquad::color::{Color, PURPLE, WHITE};
use macroquad::math::RectOffset;
use macroquad::prelude::Vec2;
use macroquad::ui::*;
use macroquad::window::screen_width;
//use crate::objects::Object;
use crate::objects::physics::{PhysicsObeject, PhysicsType};

pub(crate) fn create_side_bar(ui_id: &mut String, object: &mut Box<dyn PhysicsObeject>) {
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

    let skin = Skin {
        window_style,
        button_style,
        ..root_ui().default_skin()
    };

    let _skin_hold = root_ui().push_skin(&skin);
    root_ui().window(
        hash!(),
        Vec2::new(screen_width() * (3./4.),40.),
        Vec2::new(screen_width(), screen_width()),
        |ui| {
            let mut x = false;
            match object.get_physics_type() {
                PhysicsType::Static => { if ui.button(None, "Static  V") { x = true; } }
                PhysicsType::Kinematic => { if ui.button(None, "Kinematic V") { x = true; } }
                PhysicsType::Dynamic => { if ui.button(None, "Dynamic V") { x = true; } }
            }
            if x && *ui_id == "dropdown_types" {
                *ui_id = "".parse().unwrap();
            }else if x {
                *ui_id = "dropdown_types".parse().unwrap();
            }
            if *ui_id == "dropdown_types" {
                ui.same_line(0.);
                if ui.button(None, "Static") {object.set_physics_type(PhysicsType::Static);}
                if ui.button(None, "Kinematic") {object.set_physics_type(PhysicsType::Kinematic);}
                if ui.button(None, "Kinematic") {object.set_physics_type(PhysicsType::Dynamic);}
            }
        },
    );
}