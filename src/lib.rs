use nannou::wgpu::{Texture, TextureUsages};
use nannou::window::Window;
use stammer::Panel;

/// Convert a [`Panel`] to a [`Texture`].
pub fn panel_to_texture<D>(panel: &Panel<D>, window: &Window) -> Texture {
    let mut pixels = [panel.background; 512 * 512].concat().to_vec();

    panel.draw(&mut pixels);

    let buffer = nannou::image::ImageBuffer::from_fn(512, 512, |x, y| {
        let index = (y * 512 + x) as usize;
        let pixel = [
            pixels[index * 4],
            pixels[index * 4 + 1],
            pixels[index * 4 + 2],
            pixels[index * 4 + 3],
        ];
        nannou::image::Rgba(pixel)
    });

    Texture::load_from_image_buffer(
        window.device(),
        window.queue(),
        TextureUsages::TEXTURE_BINDING,
        &buffer,
    )
}
