use glam::Vec3;

use gobs_core::Color;
use gobs_resource::geometry::Mesh;

use crate::{line, math::to_screen_coord, pixel, rasterize, triangle_line, Image};

#[tracing::instrument(skip(img), level = "debug")]
pub fn mesh(mesh: &Mesh, img: &mut Image, color: Color) {
    for idx in mesh.indices.chunks(3) {
        let v0 = mesh.vertices[idx[0] as usize];
        let v1 = mesh.vertices[idx[1] as usize];
        let v2 = mesh.vertices[idx[2] as usize];

        triangle_wire(v0.position, v1.position, v2.position, img, color);
    }
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn dot(pos: Vec3, img: &mut Image, color: Color) {
    let pos = to_screen_coord(pos, img.width(), img.height());

    pixel(pos.x as u32, pos.y as u32, img, color);
}

pub fn line_wire(v0: Vec3, v1: Vec3, img: &mut Image, color: Color) {
    let v0 = to_screen_coord(v0, img.width(), img.height());
    let v1 = to_screen_coord(v1, img.width(), img.height());

    line(v0, v1, img, color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn triangle_wire(v0: Vec3, v1: Vec3, v2: Vec3, img: &mut Image, color: Color) {
    let v0 = to_screen_coord(v0, img.width(), img.height());
    let v1 = to_screen_coord(v1, img.width(), img.height());
    let v2 = to_screen_coord(v2, img.width(), img.height());

    triangle_line(v0, v1, v2, img, color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn triangle(
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    img: &mut Image,
    color0: Color,
    color1: Color,
    color2: Color,
) {
    let v0 = to_screen_coord(v0, img.width(), img.height());
    let v1 = to_screen_coord(v1, img.width(), img.height());
    let v2 = to_screen_coord(v2, img.width(), img.height());

    rasterize(v0, v1, v2, img, color0, color1, color2);
}
