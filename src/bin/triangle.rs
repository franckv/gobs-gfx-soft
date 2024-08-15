use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use gobs_core::Color;

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Color::BLACK.into());

    let v0 = Vec3::new(-0.9, -0.3, 0.);
    let v1 = Vec3::new(-0.5, 0.6, 0.);
    let v2 = Vec3::new(-0.3, -0.2, 0.);
    triangle(
        v0,
        v1,
        v2,
        &mut img,
        Color::RED,
        Color::GREEN,
        Color::MAGENTA,
    );

    let v0 = Vec3::new(0.8, -0.5, 0.);
    let v1 = Vec3::new(0.5, -1., 0.);
    let v2 = Vec3::new(-0.3, 0.8, 0.);
    triangle(
        v0,
        v1,
        v2,
        &mut img,
        Color::MAGENTA,
        Color::YELLOW,
        Color::WHITE,
    );

    let v0 = Vec3::new(0.8, 0.5, 0.);
    let v1 = Vec3::new(0.2, 0.6, 0.);
    let v2 = Vec3::new(0.3, 0.8, 0.);
    triangle(
        v0,
        v1,
        v2,
        &mut img,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
    );

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn main() {
    init_logger();

    draw_image();
}
