
use std::io::Cursor;

use glium::{Display, texture::RawImage2d};

pub fn get_image(file_content: &[u8], image_format: image::ImageFormat) -> RawImage2d<u8> {
    let image = image::load(Cursor::new(file_content), image_format).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

pub fn get_normal_texture(display: &Display) -> glium::texture::Texture2d {
    glium::texture::Texture2d::new(
        display, 
        get_image(&include_bytes!("../../../images/brick-normal.png")[..], image::ImageFormat::Png))
        .unwrap()
}