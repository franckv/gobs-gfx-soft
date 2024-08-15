use image::{Rgba, RgbaImage};

use gobs_resource::geometry::Mesh;

use crate::line;

pub fn mesh(mesh: &Mesh, img: &mut RgbaImage, color: Rgba<u8>) {
    for idx in mesh.indices.chunks(3) {
        let v0 = mesh.vertices[idx[0] as usize];
        let v1 = mesh.vertices[idx[1] as usize];
        let v2 = mesh.vertices[idx[2] as usize];

        line(v0.position, v1.position, img, color);
        line(v0.position, v2.position, img, color);
        line(v2.position, v1.position, img, color);
    }
}
