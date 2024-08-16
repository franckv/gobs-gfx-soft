use glam::{IVec2, Vec3};

use gobs_core::Color;
use gobs_resource::{entity::light::Light, geometry::VertexData};

#[derive(Default)]
pub struct FragmentShader {
    light: Light,
    light_position: Vec3,
}

impl FragmentShader {
    pub fn new() -> Self {
        Self {
            light: Light::new(Color::WHITE),
            light_position: Vec3::new(5., 0., -10.),
        }
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub fn shade(&self, pos: IVec2, v: &VertexData) -> Option<Color> {
        let light_dir = (v.position - self.light_position).normalize();
        let intensity = v.normal.dot(light_dir);

        tracing::debug!(intensity);
        if intensity > 0. {
            Some(v.color * self.light.colour * intensity)
        } else {
            None
        }
    }
}
