use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);

    let center = Vec3::new(0., 0., 0.);
    dot(center, &mut img, Color::WHITE);

    img.save(FILENAME).expect("Save image");
}

fn main() {
    init_logger();

    draw_image();
}
