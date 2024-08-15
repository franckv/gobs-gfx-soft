use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use gobs_core::Color;

use mesh::line_wire;
use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, Color::BLACK.into());

    draw_centers(&mut img);

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn draw_centers(img: &mut RgbaImage) {
    let center = Vec3::ZERO;
    let top_left = Vec3::new(-1., 1., 0.);
    let top_right = Vec3::new(1., 1., 0.);
    let bottom_left = Vec3::new(-1., -1., 0.);
    let bottom_right = Vec3::new(1., -1., 0.);

    line_wire(center, top_left, img, Color::RED);
    line_wire(center, top_right, img, Color::GREEN);
    line_wire(center, bottom_left, img, Color::BLUE);
    line_wire(center, bottom_right, img, Color::YELLOW);
}

fn main() {
    init_logger();

    draw_image();
}
