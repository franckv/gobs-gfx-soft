use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, BLACK);

    let center = Vec3::new(0., 0., 0.);
    dot(center, &mut img, WHITE);

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn main() {
    init_logger();

    draw_image();
}
