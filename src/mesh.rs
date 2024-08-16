use gobs_resource::geometry::Mesh;

use crate::{triangle, triangle_wire, Depth, FragmentShader, Image};

#[tracing::instrument(skip(img, depth, shader), level = "debug")]
pub fn mesh(mesh: &Mesh, img: &mut Image, depth: &mut Depth, shader: &FragmentShader) {
    for idx in mesh.indices.chunks(3) {
        let v0 = mesh.vertices[idx[0] as usize];
        let v1 = mesh.vertices[idx[1] as usize];
        let v2 = mesh.vertices[idx[2] as usize];

        triangle(v0, v1, v2, img, depth, shader);
    }
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn wire(mesh: &Mesh, img: &mut Image) {
    for idx in mesh.indices.chunks(3) {
        let v0 = mesh.vertices[idx[0] as usize];
        let v1 = mesh.vertices[idx[1] as usize];
        let v2 = mesh.vertices[idx[2] as usize];

        triangle_wire(v0, v1, v2, img);
    }
}
