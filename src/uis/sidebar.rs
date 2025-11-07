use macroquad::color::{Color, BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW};
use macroquad::math::RectOffset;
use macroquad::prelude::Vec2;
use macroquad::ui::*;
use macroquad::window::{screen_height, screen_width};
use crate::objects::physics::{PhysicsObeject, PhysicsType};

fn is_only_numbers(s: &str) -> bool {
    s.trim().parse::<f32>().is_ok()
}

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
        ui.label(None, "");
    }
    defult_skin.button_style = hold_style;
    return Color::new(1., 1., 1., 255.)
}

fn create_x_and_y_input(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize,
                        ui_id: &mut String, ui_text_save: &mut String) {
    // Declare variables for the users to edit, and get the current position of the user
    let current_pos = objects.get_mut(selected_index).unwrap().get_render_shape().get_pos().clone();
    let mut x_str: String = if ui_id == "text_input_x_coordinate" { ui_text_save.clone() } else { current_pos.x.to_string() };
    let mut y_str: String = if ui_id == "text_input_y_coordinate" { ui_text_save.clone() } else { current_pos.y.to_string() };
    // Store original values for later use
    let x_original = x_str.clone();
    let y_original = y_str.clone();

    // Create UI and inputs for mass and density
    ui.label(None,"x:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut x_str);
    ui.same_line(0.);
    ui.label(None,"y:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut y_str);

    // Check if the user has changed the x value
    if x_str != x_original {
        *ui_id = "text_input_x_coordinate".into();
        *ui_text_save = x_str.clone();
    }
    // Check if the user has changed the y value
    if y_str != y_original {
        *ui_id = "text_input_y_coordinate".into();
        *ui_text_save = y_str.clone();
    }
    // Check if the value for the position of the object needs to be changed
    if is_only_numbers(&x_str) && is_only_numbers(&y_str) {
        let new_pos = Vec2::new(x_str.trim().parse::<f32>().unwrap(), y_str.trim().parse::<f32>().unwrap());
        if x_str != x_original || y_str != y_original {
            *objects.get_mut(selected_index).unwrap().get_render_shape_referance().get_pos() = new_pos;
        }
    }
}

fn create_mass_material_inputs(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize,
                                ui_id: &mut String, ui_text_save: &mut String) {
    // Declare variables for the user to edit
    let mut mass_str: String = if ui_id == "text_input_mass" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_material().mass.to_string() };
    let mut density_str: String = if ui_id == "text_input_density" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_material().density.to_string() };
    // Store orignal values for later use
    let mass_original: String = mass_str.clone();
    let density_original: String = density_str.clone();

    // Create UI and inputs for mass and density
    ui.label(None, "Mass:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut mass_str);
    ui.label(None, "Density:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut density_str);

    // Check if the user has changed the mass value
    if mass_str != mass_original {
        // Edit ui_id and text saves
        *ui_id = "text_input_mass".into();
        *ui_text_save = mass_str.clone();
        // Calculate new mass and density
        if is_only_numbers(&mass_str) {
            let new_mass = mass_str.trim().parse::<f32>().unwrap();
            let area = objects.get_mut(selected_index).unwrap().get_material().area;
            let new_density = new_mass/area;
            objects.get_mut(selected_index).unwrap().get_material().mass = new_mass;
            objects.get_mut(selected_index).unwrap().get_material().density = new_density;
        }
    }
    // Check if user has changed the density value
    if density_str != density_original {
        // Edit ui_id and text saves
        *ui_id = "text_input_density".into();
        *ui_text_save = density_str.clone();
        // Calculate new mass and density
        if is_only_numbers(&density_str) {
            let new_density = density_str.trim().parse::<f32>().unwrap();
            let area = objects.get_mut(selected_index).unwrap().get_material().area;
            let new_mass = new_density * area;
            objects.get_mut(selected_index).unwrap().get_material().mass = new_mass;
            objects.get_mut(selected_index).unwrap().get_material().density = new_density;
        }
    }
}

fn create_shape_inputs(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize,
                       ui_id: &mut String, ui_text_save: &mut String) {
    match objects.get_mut(selected_index).unwrap().get_render_shape_referance().get_id() {
        "Square" => {
            let mut text_box_1: String = if ui_id == "text_input_shape_1" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_referance().get_measurements().0.to_string() };
            let text_box_1_save = text_box_1.clone();
            ui.label(None, "Size:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_1);
            if text_box_1_save != text_box_1_save {
                *ui_id = "text_input_shape_1".into();
                *ui_text_save = text_box_1_save.clone();
                if is_only_numbers(&text_box_1) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_referance().set_measurements((text_box_1.trim().parse::<f32>().unwrap(), -1.));
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
        }
        "Rectangle" => {
            let mut text_box_1 = if ui_id == "text_input_shape_1" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_referance().get_measurements().0.to_string() };
            let mut text_box_2 = if ui_id == "text_input_shape_2" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_referance().get_measurements().1.to_string() };
            let text_box_1_save = text_box_1.clone();
            let text_box_2_save = text_box_2.clone();
            ui.label(None, "Width:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_1);
            ui.same_line(0.);
            ui.label(None, "Height:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_2);
            if text_box_1 != text_box_1_save {
                *ui_id = "text_input_shape_1".into();
                *ui_text_save = text_box_1.clone();
                if is_only_numbers(&text_box_1) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_referance().set_measurements(
                        (text_box_1.trim().parse::<f32>().unwrap(), text_box_2.trim().parse::<f32>().unwrap())
                    );
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
            if text_box_2 != text_box_2_save {
                *ui_id = "text_input_shape_2".into();
                *ui_text_save = text_box_2.clone();
                if is_only_numbers(&text_box_2) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_referance().set_measurements(
                        (text_box_1.trim().parse::<f32>().unwrap(), text_box_2.trim().parse::<f32>().unwrap())
                    );
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
        }
        "Circle" => {
            let mut text_box_1: String = if ui_id == "text_input_shape_1" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_referance().get_measurements().0.to_string() };
            let text_box_1_save = text_box_1.clone();
            ui.label(None, "Radius:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_1);
            if text_box_1 != text_box_1_save {
                *ui_id = "text_input_shape_1".into();
                *ui_text_save = text_box_1.clone();
                if is_only_numbers(&text_box_1) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_referance().set_measurements((text_box_1.trim().parse::<f32>().unwrap(), -1.));
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
        }
        _ => { panic!("Unsupported render shape"); }
    }
}

fn build_bin_button(ui: &mut Ui, _ui_id: &mut String, default_skin: &mut Skin, bin_button_style: Style, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize) {
    default_skin.button_style = bin_button_style;
    let _skin_hold = ui.push_skin(default_skin);
    if ui.button(None, "X delete") {
        *objects.get_mut(selected_index).unwrap().get_to_be_deleted() = true;
    }
}


pub(crate) fn create_side_bar(ui_id: &mut String, objects: &mut Vec<Box<dyn PhysicsObeject>>, selected_index: usize, ui_text_save: &mut String) {
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

    let input_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        .color(Color::from_rgba(42, 42, 42, 255))
        .color_inactive(Color::from_rgba(42, 42, 42, 255))
        .color_hovered(Color::from_rgba(85, 85, 85, 255))
        .color_selected(Color::from_rgba(85, 85, 85, 255))
        .color_clicked(PURPLE)
        .text_color(WHITE)
        .build();

    let bin_button_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        .color(Color::from_rgba(150, 0, 0, 255))
        .color_inactive(Color::from_rgba(100, 0, 0, 255))
        .color_hovered(Color::from_rgba(200, 0, 0, 255))
        .color_clicked(Color::from_rgba(255, 0, 0, 255))
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .build();

    let mut skin = Skin {
        window_style,
        button_style,
        editbox_style: input_style,
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
        Vec2::new((screen_width()) - 400., 40.),
        Vec2::new(screen_width(), screen_height()),
        |ui| {
            create_x_and_y_input(ui, objects, selected_index, ui_id, ui_text_save);
            create_mass_material_inputs(ui, objects, selected_index, ui_id, ui_text_save);
            create_types_drop(ui, objects, selected_index, ui_id);
            create_shape_inputs(ui, objects, selected_index, ui_id, ui_text_save);
            let colour_option = create_colour_buttons(ui, colour_button_style, &mut skin, ui_id);
            if colour_option != Color::new(1., 1., 1., 255.) {
                objects.get_mut(selected_index).unwrap().get_render_shape_referance().set_colour(colour_option);
            }
            build_bin_button(ui, ui_id, &mut skin, bin_button_style, objects, selected_index);
        },
    );
}
