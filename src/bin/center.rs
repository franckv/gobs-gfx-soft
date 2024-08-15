use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, BLACK);

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

    line(center, top_left, img, RED);
    line(center, top_right, img, GREEN);
    line(center, bottom_left, img, BLUE);
    line(center, bottom_right, img, YELLOW);
}

fn main() {
    init_logger();

    draw_image();
}
