use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, BLACK);

    let mut c = 0;
    c += fill(&mut img);

    tracing::info!(c, "Draw");

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn fill(img: &mut RgbaImage) -> u32 {
    let center = Vec3::ZERO;
    let mut c = 0;

    for x in 0..WIDTH {
        let x = 2. * (x as f32 / WIDTH as f32) - 1.;
        let pos = Vec3::new(x, 1., 0.);
        line(center, pos, img, GREEN);
        let pos = Vec3::new(x, -1., 0.);
        line(center, pos, img, RED);
        c += 2;
    }
    for y in 0..HEIGHT {
        let y = 2. * (y as f32 / HEIGHT as f32) - 1.;
        let pos = Vec3::new(1., y, 0.);
        line(center, pos, img, BLUE);
        let pos = Vec3::new(-1., y, 0.);
        line(center, pos, img, YELLOW);
        c += 2;
    }

    c
}

fn main() {
    init_logger();

    draw_image();
}
