use glam::Vec3;
use image::{ImageFormat, RgbaImage};

use gobs_resource::geometry::{Mesh, VertexData};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, BLACK);

    let v0 = Vec3::new(-1., -1., 0.);
    let v1 = Vec3::new(1., -1., 0.);
    let v2 = Vec3::new(0., 1., 0.);

    let mesh_data = Mesh::builder("wireframe")
        .vertex(VertexData::builder().padding(false).position(v0).build())
        .vertex(VertexData::builder().padding(false).position(v1).build())
        .vertex(VertexData::builder().padding(false).position(v2).build())
        .build();

    mesh(&mesh_data, &mut img, RED);

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn main() {
    init_logger();

    draw_image();
}
