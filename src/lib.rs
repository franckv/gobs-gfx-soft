pub mod primitives;

pub use primitives::{dot, line};

use image::Rgba;
use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};

pub const WIDTH: u32 = 250;
pub const HEIGHT: u32 = 250;

pub const CENTER_X: u32 = WIDTH / 2;
pub const CENTER_Y: u32 = HEIGHT / 2;

pub const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
pub const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
pub const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
pub const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
pub const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);
pub const YELLOW: Rgba<u8> = Rgba([255, 255, 0, 255]);

pub const LOOPS: u32 = 1000;

pub fn init_logger() {
    let sub = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(sub).unwrap();
}
