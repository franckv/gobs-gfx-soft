use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use gobs_core::Color;

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Color::BLACK.into());

    let center = Vec3::new(0., 0., 0.);
    dot(center, &mut img, Color::WHITE);

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn main() {
    init_logger();

    draw_image();
}
