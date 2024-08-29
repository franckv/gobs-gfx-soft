use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};
use gobs_gfx_soft::*;
use gobs_resource::geometry::VertexData;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);
    let mut depth = Depth::new(ImageExtent2D::new(WIDTH, HEIGHT));

    let shader = FragmentShader::new();

    let v0 = VertexData::builder()
        .position(Vec3::new(-0.9, -0.3, 0.))
        .color(Color::RED)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v1 = VertexData::builder()
        .position(Vec3::new(-0.5, 0.6, 0.))
        .color(Color::GREEN)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v2 = VertexData::builder()
        .position(Vec3::new(-0.3, -0.2, 0.))
        .color(Color::BLUE)
        .normal(Vec3::Z)
        .padding(false)
        .build();

    triangle(v0, v1, v2, &mut img, &mut depth, &shader);

    let v0 = VertexData::builder()
        .position(Vec3::new(0.8, -0.5, 0.))
        .color(Color::MAGENTA)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v1 = VertexData::builder()
        .position(Vec3::new(0.5, -1., 0.))
        .color(Color::YELLOW)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v2 = VertexData::builder()
        .position(Vec3::new(-0.3, 0.8, 0.))
        .color(Color::WHITE)
        .normal(Vec3::Z)
        .padding(false)
        .build();

    triangle(v0, v1, v2, &mut img, &mut depth, &shader);

    let v0 = VertexData::builder()
        .position(Vec3::new(0.8, 0.5, 0.))
        .color(Color::GREEN)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v1 = VertexData::builder()
        .position(Vec3::new(0.2, 0.6, 0.))
        .color(Color::BLUE)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v2 = VertexData::builder()
        .position(Vec3::new(0.3, 0.8, 0.))
        .color(Color::CYAN)
        .normal(Vec3::Z)
        .padding(false)
        .build();

    triangle(v0, v1, v2, &mut img, &mut depth, &shader);

    let v0 = VertexData::builder()
        .position(Vec3::new(-1.5, -0.9, 0.))
        .color(Color::GREEN)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v1 = VertexData::builder()
        .position(Vec3::new(-0.1, -0.9, 0.))
        .color(Color::RED)
        .normal(Vec3::Z)
        .padding(false)
        .build();
    let v2 = VertexData::builder()
        .position(Vec3::new(-0.5, -0.5, 0.))
        .color(Color::MAGENTA)
        .normal(Vec3::Z)
        .padding(false)
        .build();

    triangle(v0, v1, v2, &mut img, &mut depth, &shader);

    img.save(FILENAME).expect("Save image");
}

fn main() {
    init_logger();

    draw_image();
}
