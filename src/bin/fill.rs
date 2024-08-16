use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};

use mesh::line_wire;
use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);

    let mut c = 0;
    for _ in 0..400 {
        c += fill(&mut img);
    }

    tracing::info!(c, "Draw");

    img.save(FILENAME).expect("Save image");
}

fn fill(img: &mut Image) -> u32 {
    let center = Vec3::ZERO;
    let mut c = 0;

    for x in 0..WIDTH {
        let x = 2. * (x as f32 / WIDTH as f32) - 1.;
        let pos = Vec3::new(x, 1., 0.);
        line_wire(center, pos, img, Color::GREEN);
        let pos = Vec3::new(x, -1., 0.);
        line_wire(center, pos, img, Color::RED);
        c += 2;
    }
    for y in 0..HEIGHT {
        let y = 2. * (y as f32 / HEIGHT as f32) - 1.;
        let pos = Vec3::new(1., y, 0.);
        line_wire(center, pos, img, Color::BLUE);
        let pos = Vec3::new(-1., y, 0.);
        line_wire(center, pos, img, Color::YELLOW);
        c += 2;
    }

    c
}

fn main() {
    init_logger();

    draw_image();
}
