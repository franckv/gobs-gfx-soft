use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};
use gobs_gfx_soft::*;
use gobs_resource::geometry::VertexData;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);

    let center = VertexData::builder()
        .position(Vec3::new(0., 0., 0.))
        .color(Color::WHITE)
        .padding(false)
        .build();
    dot(center, &mut img);

    img.save(FILENAME).expect("Save image");
}

fn main() {
    init_logger();

    draw_image();
}
