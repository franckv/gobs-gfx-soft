mod image;
pub mod math;
pub mod mesh;
pub mod primitives;

pub use image::Image;
pub use mesh::{dot, mesh, triangle};
pub use primitives::{line, pixel, rasterize, triangle_line};

use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub const FILENAME: &str = "test.tga";

pub fn init_logger() {
    let sub = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();
}
