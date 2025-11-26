use macroquad::color::{Color, BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, SKYBLUE, WHITE, YELLOW};
use macroquad::math::RectOffset;
use macroquad::prelude::Vec2;
use macroquad::ui::*;
use macroquad::window::{screen_height, screen_width};
use crate::objects::physics::{PhysicsObject, PhysicsType};

//Check if the given string is only comprised of numbers
fn is_only_numbers(s: &str) -> bool {
    s.trim().parse::<f32>().is_ok()
}

//Create the physics type dropdown
fn create_types_drop(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize, ui_id: &mut String) {
    //Check if the user has pressed the dropdown button
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
        //Un-drop the drop-down if the user click the drop-down button when it is down
        *ui_id = "".parse().unwrap();
    } else if x {
        //Open the drop-down when the user presses the drop-down button
        *ui_id = "dropdown_types".parse().unwrap();
    }
    //Show the drop-down when the drop-down is open
    if *ui_id == "dropdown_types" {
        //Check if the user has clicked the static type button, and then change the type
        if ui.button(None, "Static") {
            objects.get_mut(selected_index).unwrap().set_physics_type(PhysicsType::Static);
            *ui_id = "".parse().unwrap();
        }
        //Check if the user has clicked the kinematic type button, and then change the type
        if ui.button(None, "Kinematic") {
            objects.get_mut(selected_index).unwrap().set_physics_type(PhysicsType::Kinematic);
            *ui_id = "".parse().unwrap();
        }
        //Check if the user has clicked the dynamic type button, and then change the type
        if ui.button(None, "Dynamic") {
            objects.get_mut(selected_index).unwrap().set_physics_type(PhysicsType::Dynamic);
            *ui_id = "".parse().unwrap();
        }
    }
}

//Create a style for a colour the user would like to use
fn create_colour_button_skin(color: Color, root_ui: &mut Ui) -> Style {
    root_ui
        //use the defult button style
        .style_builder()
        .background_margin(RectOffset::new(0.0, 16.0, 0.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, 16.0, -16.0))
        //set all the colour options
        .color(color)
        .color_inactive(color)
        .color_hovered(color)
        .color_clicked(color)
        //continue to use the defult text colour for the buttons
        .text_color(WHITE)
        .text_color_hovered(WHITE)
        .text_color_clicked(WHITE)
        .build()
}

//Create the buttons for the user to change the colour of the shape they would like ot edit
fn create_colour_buttons(ui: &mut Ui, colour_button_style: Style, defult_skin: &mut Skin, ui_id: &mut String) -> Color {
    //Hold the defult skin cloned for later
    let hold_style = defult_skin.button_style.clone();
    //Set the button style to the colour of the shape
    defult_skin.button_style = colour_button_style;
    //Hold the skin so it is dropped at the end of the function
    let _skin_hold = ui.push_skin(defult_skin);
    //Check if the user needs the colours drop-down
    if ui.button(None, "           ") {
        if *ui_id == "colour_dropdown_options" {
            *ui_id = "".parse().unwrap();
        }else {
            *ui_id = "colour_dropdown_options".parse().unwrap();
        }
    }

    //Create a list of colours that the user may want to use
    let colours = vec!(RED, ORANGE, YELLOW, GREEN, SKYBLUE, BLUE, PURPLE, WHITE, BLACK);

    //Check if the user needs all the colour options available to them
    if *ui_id == "colour_dropdown_options" {
        //Loop through every colour the user could want to use
        for i in colours {
            //Let the colour button style be the current colour
            let colour_button = create_colour_button_skin(i, ui);
            //Change the colour of the skin used
            defult_skin.button_style = colour_button;
            //Update the skin of the button
            let _skin_hold = ui.push_skin(defult_skin);
            //Check if the user has clicked on that colour
            if ui.button(None, "") {
                defult_skin.button_style = hold_style;
                *ui_id = "".parse().unwrap();
                return i;
            }
            ui.same_line(0.);
        }
        ui.label(None, "");
    }
    //Make the skin the defult skin, and then return the null value for the colour
    defult_skin.button_style = hold_style;
    Color::new(1., 1., 1., 255.)
}

//Create the inputs for the x and y position of the shape
fn create_x_and_y_input(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize,
                        ui_id: &mut String, ui_text_save: &mut String) {
    //Declare variables for the users to edit, and get the current position of the user
    let current_pos = objects.get_mut(selected_index).unwrap().get_render_shape().get_pos().clone();
    let mut x_str: String = if ui_id == "text_input_x_coordinate" { ui_text_save.clone() } else { current_pos.x.to_string() };
    let mut y_str: String = if ui_id == "text_input_y_coordinate" { ui_text_save.clone() } else { current_pos.y.to_string() };
    //Store original values for later use
    let x_original = x_str.clone();
    let y_original = y_str.clone();

    //Create UI and inputs for x and y
    ui.label(None,"x:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut x_str);
    ui.same_line(0.);
    ui.label(None,"y:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut y_str);

    //Check if the user has changed the x value
    if x_str != x_original {
        *ui_id = "text_input_x_coordinate".into();
        *ui_text_save = x_str.clone();
    }
    //Check if the user has changed the y value
    if y_str != y_original {
        *ui_id = "text_input_y_coordinate".into();
        *ui_text_save = y_str.clone();
    }
    //Check if the value for the position of the object needs to be changed
    if is_only_numbers(&x_str) && is_only_numbers(&y_str) {
        let new_pos = Vec2::new(x_str.trim().parse::<f32>().unwrap(), y_str.trim().parse::<f32>().unwrap());
        if x_str != x_original || y_str != y_original {
            *objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_pos() = new_pos;
        }
    }
}

//Create inputs for the velocity of the selected object
fn create_velocity_inputs(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize,
                          ui_id: &mut String, ui_text_save: &mut String) {
    //Create variables for the user to edit, and get the current x and y position
    let mut vx_str: String = if ui_id == "text_input_velocity_x" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_velocity().x.to_string() };
    let mut vy_str: String = if ui_id == "text_input_velocity_y" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_velocity().y.to_string() };
    //Store the original values for later use
    let vx_original = vx_str.clone();
    let vy_original = vy_str.clone();

    //Create UI and inputs for the velocity_x and velocity_y
    ui.label(None,"vx:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut vx_str);
    ui.same_line(0.);
    ui.label(None,"vy:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut vy_str);

    //Check if the user has changed the vx
    if vx_str != vx_original {
        *ui_id = "text_input_velocity_x".into();
        *ui_text_save = vx_str.clone();
    }
    //Check if the user has changed the vy
    if vy_str != vy_original {
        *ui_id = "text_input_velocity_y".into();
        *ui_text_save = vy_str.clone();
    }
    //Check if the value for velocity, and if it needs to be changed
    if is_only_numbers(&vx_str) && is_only_numbers(&vy_str) && (vx_str != vx_original || vy_str != vy_original) {
        let new_velocity = Vec2::new(vx_str.trim().parse::<f32>().unwrap(), vy_str.trim().parse::<f32>().unwrap());
        objects.get_mut(selected_index).unwrap().set_velocity(new_velocity);
    }
}

//Create inputs for the mass and density of the selected object
fn create_mass_material_inputs(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize,
                               ui_id: &mut String, ui_text_save: &mut String) {
    //Declare variables for the user to edit
    let mut mass_str: String = if ui_id == "text_input_mass" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_material().mass.to_string() };
    let mut density_str: String = if ui_id == "text_input_density" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_material().density.to_string() };
    //Store original values for later use
    let mass_original: String = mass_str.clone();
    let density_original: String = density_str.clone();

    //Create UI and inputs for mass and density
    ui.label(None, "Mass:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut mass_str);
    ui.label(None, "Density:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut density_str);

    //Check if the user has changed the mass value
    if mass_str != mass_original {
        //Edit ui_id and text saves
        *ui_id = "text_input_mass".into();
        *ui_text_save = mass_str.clone();
        //Calculate new mass and density
        if is_only_numbers(&mass_str) {
            let new_mass = mass_str.trim().parse::<f32>().unwrap();
            let area = objects.get_mut(selected_index).unwrap().get_material().area;
            let new_density = new_mass/area;
            objects.get_mut(selected_index).unwrap().get_material().mass = new_mass;
            objects.get_mut(selected_index).unwrap().get_material().density = new_density;
        }
    }
    //Check if user has changed the density value
    if density_str != density_original {
        //Edit ui_id and text saves
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

//Create the gravity inputs for the selected object
fn build_gravity_inputs(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize,
                        ui_id: &mut String, ui_text_save: &mut String) {
    //Declare the variable for the user to edit
    let mut gravity_str: String = if ui_id == "text_input_gravity" { ui_text_save.to_string() }
    else {objects.get_mut(selected_index).unwrap().get_gravity().to_string()};
    let gravity_original: String = gravity_str.clone();

    //Create UI and inputs for gravity
    ui.label(None, "Gravity:");
    ui.same_line(0.);
    ui.editbox(hash!(), Vec2::new(100., 20.), &mut gravity_str);

    //Check if user has changed the value of gravity
    if gravity_str != gravity_original {
        *ui_id = "text_input_gravity".into();
        *ui_text_save = gravity_str.clone();
        if is_only_numbers(&gravity_str) {
            *objects.get_mut(selected_index).unwrap().get_gravity() = gravity_str.trim().parse::<f32>().unwrap();
        }
    }
}

//Create the shape inputs for all different types of shape, e.g. radius for the circle
fn create_shape_inputs(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize,
                       ui_id: &mut String, ui_text_save: &mut String) {
    //Find which shape the user has selected
    match objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_id() {
        //Create the shape inputs for a square
        "Square" => {
            //Create values of the square for the user to edit
            let mut text_box_1: String = if ui_id == "text_input_shape_1" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_measurements().0.to_string() };
            let text_box_1_save = text_box_1.clone();
            //Create the UI and inputs for the square
            ui.label(None, "Size:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_1);
            //If the user changes a value, update the square
            if text_box_1 != text_box_1_save {
                *ui_id = "text_input_shape_1".into();
                *ui_text_save = text_box_1_save.clone();
                //Update the square, and then update its material (changing density of the object)
                if is_only_numbers(&text_box_1) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_reference().set_measurements((text_box_1.trim().parse::<f32>().unwrap(), -1.));
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
        }
        //Create the shape inputs for the rectangle
        "Rectangle" => {
            //Create the variables for the user to edit, and hence change the values of shape
            let mut text_box_1 = if ui_id == "text_input_shape_1" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_measurements().0.to_string() };
            let mut text_box_2 = if ui_id == "text_input_shape_2" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_measurements().1.to_string() };
            let text_box_1_save = text_box_1.clone();
            let text_box_2_save = text_box_2.clone();
            //Create the UI and inputs for the rectangle
            ui.label(None, "Width:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_1);
            ui.same_line(0.);
            ui.label(None, "Height:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_2);
            //Check if the user has changed text_box_1
            if text_box_1 != text_box_1_save {
                *ui_id = "text_input_shape_1".into();
                *ui_text_save = text_box_1.clone();
                //Update the value of the said changed side of the rectangle
                if is_only_numbers(&text_box_1) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_reference().set_measurements(
                        (text_box_1.trim().parse::<f32>().unwrap(), text_box_2.trim().parse::<f32>().unwrap())
                    );
                    //Update the material of the object (density)
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
            //Check if the user has changed text_box_2
            if text_box_2 != text_box_2_save {
                *ui_id = "text_input_shape_2".into();
                *ui_text_save = text_box_2.clone();
                //Update the value of the said changed side of the rectangle
                if is_only_numbers(&text_box_2) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_reference().set_measurements(
                        (text_box_1.trim().parse::<f32>().unwrap(), text_box_2.trim().parse::<f32>().unwrap())
                    );
                    //Update the material of the object (density)
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
        }
        //Create the shape inputs for the circle
        "Circle" => {
            //Create the variable for the radius and hence the variable for the user to change
            let mut text_box_1: String = if ui_id == "text_input_shape_1" { ui_text_save.clone() } else { objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_measurements().0.to_string() };
            let text_box_1_save = text_box_1.clone();
            //Create the UI inputs for shape
            ui.label(None, "Radius:");
            ui.same_line(0.);
            ui.editbox(hash!(), Vec2::new(100., 20.), &mut text_box_1);
            //Check if the user has changed the value for the radius
            if text_box_1 != text_box_1_save {
                *ui_id = "text_input_shape_1".into();
                *ui_text_save = text_box_1.clone();
                //Update the value for the radius, and update the material (density)
                if is_only_numbers(&text_box_1) {
                    objects.get_mut(selected_index).unwrap().get_render_shape_reference().set_measurements((text_box_1.trim().parse::<f32>().unwrap(), -1.));
                    objects.get_mut(selected_index).unwrap().update_material();
                }
            }
        }
        //If the object is something it doesn't recognise, panic as i've been really stupid and forgot to add it
        _ => { panic!("Unsupported render shape"); }
    }
}

//Build the transparency slider for the colour of the object the user would like to use
fn build_transparency_slider(ui: &mut Ui, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize) {
    //Create the saved data for the user's object
    let mut colour = objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_colour();
    let mut data = colour.a * 100.;
    //Create the UI and inputs for the transparency slider
    ui.label(None, "Alpha:");
    ui.same_line(0.);
    //Use a group to control the size of the transparency slider
    ui.group(hash!(), Vec2::new(300.,30.), |ui| {
        ui.slider(hash!(), "", 0.0..100., &mut data);
    });
    //Reset the colour, and update the colour of the selected shape
    colour.a = data/100.;
    objects.get_mut(selected_index).unwrap().get_render_shape_reference().set_colour(colour);
}

//Create the delete button for the selected object
fn build_bin_button(ui: &mut Ui, _ui_id: &mut String, default_skin: &mut Skin, bin_button_style: Style, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize) {
    //Hold the defult skin for the user
    default_skin.button_style = bin_button_style;
    //Hold the skin so it will be dropped at the end of the function
    let _skin_hold = ui.push_skin(default_skin);
    //Check if the selected object needs to be flagged to be deleted
    if ui.button(None, "X delete") {
        *objects.get_mut(selected_index).unwrap().get_to_be_deleted() = true;
    }
}

//Create the sidebar for the user
pub(crate) fn create_side_bar(ui_id: &mut String, objects: &mut Vec<Box<dyn PhysicsObject>>, selected_index: usize, ui_text_save: &mut String) {
    //Use the normal style for the window for the whole project
    let window_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(46, 46, 46, 255))
        .color_inactive(Color::from_rgba(46, 46, 46, 255))
        .text_color(WHITE)
        .build();

    //Use the normal button style for the whole project
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

    //Use the normal input style for the whole project
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

    //Create the button style for the delete button
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

    //Create the skin for the sidebar to use for the UI
    let mut skin = Skin {
        label_style: window_style.clone(),
        window_style,
        button_style,
        editbox_style: input_style,
        ..root_ui().default_skin()
    };

    //Use a skin_hold so the skin will be dropped at the end of the function
    let _skin_hold = root_ui().push_skin(&skin);

    //Create the colour button style for the button showing the colour the user has selected
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

    //Create the window and hence sidebar
    root_ui().window(
        hash!(),
        Vec2::new((screen_width()) - 400., 40.),
        Vec2::new(screen_width(), screen_height()),
        //Build all the inputs for the sidebar
        |ui| {
            create_x_and_y_input(ui, objects, selected_index, ui_id, ui_text_save);
            create_velocity_inputs(ui, objects, selected_index, ui_id, ui_text_save);
            create_mass_material_inputs(ui, objects, selected_index, ui_id, ui_text_save);
            build_gravity_inputs(ui, objects, selected_index, ui_id, ui_text_save);
            create_types_drop(ui, objects, selected_index, ui_id);
            create_shape_inputs(ui, objects, selected_index, ui_id, ui_text_save);
            //Create a variable to store the value return from the colour buttons function
            let mut colour_option = create_colour_buttons(ui, colour_button_style, &mut skin, ui_id);
            //Check if the colour is not the defult colour (has changed)
            if colour_option != Color::new(1., 1., 1., 255.) {
                //Update the colour to the appropriate colour
                colour_option.a = objects.get_mut(selected_index).unwrap().get_render_shape_reference().get_colour().a;
                objects.get_mut(selected_index).unwrap().get_render_shape_reference().set_colour(colour_option);
            }
            build_transparency_slider(ui, objects, selected_index);
            build_bin_button(ui, ui_id, &mut skin, bin_button_style, objects, selected_index);
        },
    );
}
