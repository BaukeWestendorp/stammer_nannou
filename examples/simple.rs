use std::rc::Rc;

use nannou::prelude::*;
use stammer::elements::builder::ElementBuilder;
use stammer::elements::Element;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() {
    nannou::app(model).loop_mode(LoopMode::Wait).run();
}

struct Model {
    window_id: WindowId,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model { window_id }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window(model.window_id).unwrap();

    let font = stammer::Font::load_from_file("/etc/tid/fonts/times15.uf2").unwrap();

    // Create simple element to show on the panel
    let element = Element::paragraph("Hello, world!", &Rc::new(font))
        .build()
        .with_fixedwidth(WIDTH)
        .with_fixedheight(HEIGHT);

    // Create the panel without any data, as we do not need it for this example
    let state = stammer::Panel::new(
        element,
        [0u8, 0u8, 0u8, 255u8],
        [0u8, 255u8, 0u8, 255u8],
        (),
    );

    // Create the nannout::wgpu::Texture from the stammer::Panel
    let texture = stammer_nannou::panel_to_texture(&state, &window);
    // Draw the texture
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
