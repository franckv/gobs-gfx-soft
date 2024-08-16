use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};
use gobs_resource::geometry::VertexData;

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);

    draw_centers(&mut img);

    img.save(FILENAME).expect("Save image");
}

fn draw_centers(img: &mut Image) {
    let center = VertexData::builder()
        .position(Vec3::ZERO)
        .padding(false)
        .build();
    let top_left = VertexData::builder()
        .position(Vec3::new(-1., 1., 0.))
        .padding(false)
        .color(Color::RED)
        .build();
    let top_right = VertexData::builder()
        .position(Vec3::new(1., 1., 0.))
        .padding(false)
        .color(Color::GREEN)
        .build();
    let bottom_left = VertexData::builder()
        .position(Vec3::new(-1., -1., 0.))
        .padding(false)
        .color(Color::BLUE)
        .build();
    let bottom_right = VertexData::builder()
        .position(Vec3::new(1., -1., 0.))
        .padding(false)
        .color(Color::YELLOW)
        .build();

    line(center, top_left, img);
    line(center, top_right, img);
    line(center, bottom_left, img);
    line(center, bottom_right, img);
}

fn main() {
    init_logger();

    draw_image();
}
