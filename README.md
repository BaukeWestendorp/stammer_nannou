# stammer_nannou
This crate makes it easy to show a `stammer` Panel in a `nannou` program.

# Usage
for now the easiest way to add this crate to your project is to add it to
your `Cargo.toml` by adding the dependency using git.
```toml
stammer_nannou = { git = "https://github.com/BaukeWestendorp/stammer_nannou", branch = "main" }
```

# Examples
Here is a simple view function for `nannou` that uses the `panel_to_texture` function.
```rust
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
```


Made with <3 by Bauke