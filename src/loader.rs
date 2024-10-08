use std::sync::Arc;

use gobs_resource::geometry::{Mesh, VertexData};

pub fn load_model(file: &str) -> Arc<Mesh> {
    let (models, _materials) = tobj::load_obj(file, &tobj::GPU_LOAD_OPTIONS).unwrap();

    let mut mesh = Mesh::builder(file);

    let mut min_pos: f32 = 0.;
    let mut max_pos: f32 = 0.;

    for model in models {
        tracing::info!(
            model.name,
            "Load model (vertices={}, indices={})",
            model.mesh.positions.len(),
            model.mesh.indices.len()
        );

        for idx in 0..model.mesh.positions.len() / 3 {
            let position = [
                model.mesh.positions[3 * idx as usize],
                model.mesh.positions[3 * idx as usize + 1],
                model.mesh.positions[3 * idx as usize + 2],
            ];
            min_pos = min_pos.min(position[0]);
            min_pos = min_pos.min(position[1]);
            min_pos = min_pos.min(position[2]);
            max_pos = max_pos.max(position[0]);
            max_pos = max_pos.max(position[1]);
            max_pos = max_pos.max(position[2]);

            let color = [
                *model.mesh.vertex_color.get(3 * idx).unwrap_or(&1.),
                *model.mesh.vertex_color.get(3 * idx + 1).unwrap_or(&1.),
                *model.mesh.vertex_color.get(3 * idx + 2).unwrap_or(&1.),
                1.,
            ];

            let normal = [
                *model.mesh.normals.get(3 * idx).unwrap_or(&0.),
                *model.mesh.normals.get(3 * idx + 1).unwrap_or(&0.),
                *model.mesh.normals.get(3 * idx + 2).unwrap_or(&1.),
            ];

            mesh = mesh.vertex(
                VertexData::builder()
                    .padding(false)
                    .position(position.into())
                    .color(color.into())
                    .normal(normal.into())
                    .build(),
            );
        }

        mesh = mesh.indices(&model.mesh.indices);
    }

    tracing::info!(min_pos, max_pos, "Model loaded");
    mesh.build()
}
