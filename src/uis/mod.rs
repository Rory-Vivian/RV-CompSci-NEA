use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

fn build_hot_bar() {
    let bar_style = root_ui().style_builder()
        .color(Color::from_rgba(36,36,36,255))
        .color_inactive(Color::from_rgba(36,36,36,255))
        .text_color(WHITE)
        .build();

    let button_bar = root_ui().style_builder()
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

    let bar_skin = Skin {
        window_style: bar_style,
        button_style: button_bar,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&bar_skin);
    root_ui().window(hash!(), Vec2::new(0., 0.), Vec2::new(screen_width(), 40.), |ui| {
        ui.button(None, "Ball");
        ui.same_line(0.0);
        ui.button(None, "Square");
    });
    root_ui().pop_skin();
}

fn build_zoom_bar(zoom: &mut f32) {
    // Build a simple semi-transparent purple skin
    let window_style = root_ui()
        .style_builder()
        .color(Color::from_rgba(126, 29, 251, 0)) // semi-transparent
        .text_color(WHITE)
        .build();
    let bar_style = root_ui()
        .style_builder()
        .text_color(WHITE)
        .build();

    // Apply the style to the window; inherit the rest from the default skin
    let skin = Skin {
        window_style,
        label_style: bar_style,
        ..root_ui().default_skin()
    };

    let bar_size = vec2(500.0, 40.0);
    let bar_pos = vec2(
        0.0, // center horizontally
        screen_height(), // 10px from bottom
    );

    // Push skin for the duration of this UI block
    let _skin_guard = root_ui().push_skin(&skin);

    root_ui().window(hash!("zoom_window"), bar_pos, bar_size, |ui| {
        ui.slider(hash!("zoom_slider"), "Zoom", 10.0..200.0, zoom);
    });
}

pub fn build_ui(zoom: &mut f32) {
    build_hot_bar();
    build_zoom_bar(zoom);
}