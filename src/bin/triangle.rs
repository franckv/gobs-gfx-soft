use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);

    let v0 = Vec3::new(-0.9, -0.3, 0.);
    let v1 = Vec3::new(-0.5, 0.6, 0.);
    let v2 = Vec3::new(-0.3, -0.2, 0.);
    triangle(v0, v1, v2, &mut img, Color::RED, Color::GREEN, Color::BLUE);

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
    triangle(v0, v1, v2, &mut img, Color::GREEN, Color::BLUE, Color::CYAN);

    let v0 = Vec3::new(-1.5, -0.9, 0.);
    let v1 = Vec3::new(-0.1, -0.9, 0.);
    let v2 = Vec3::new(-0.5, -0.5, 0.);
    triangle(v0, v1, v2, &mut img, Color::RED, Color::GREEN, Color::BLUE);

    img.save(FILENAME).expect("Save image");
}

fn main() {
    init_logger();

    draw_image();
}
