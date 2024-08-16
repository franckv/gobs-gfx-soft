mod depth;
mod image;
pub mod loader;
pub mod math;
mod mesh;
mod primitives;
mod shader;

pub use depth::Depth;
pub use image::Image;
pub use mesh::{mesh, wire};
pub use primitives::{dot, line, pixel, triangle, triangle_wire};
pub use shader::FragmentShader;

use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const FILENAME: &str = "test.tga";
pub const DFILENAME: &str = "depth.tga";

pub fn init_logger() {
    let sub = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();
}
