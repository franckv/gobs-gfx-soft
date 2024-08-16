use glam::Vec3;

use gobs_core::{Color, ImageExtent2D};
use gobs_resource::geometry::VertexData;

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
    let center = VertexData::builder()
        .position(Vec3::ZERO)
        .padding(false)
        .color(Color::WHITE)
        .build();

    let mut c = 0;
    for x in 0..WIDTH {
        let x = 2. * (x as f32 / WIDTH as f32) - 1.;
        let v = VertexData::builder()
            .position(Vec3::new(x, 1., 0.))
            .padding(false)
            .color(Color::GREEN)
            .build();
        line(center, v, img);
        let v = VertexData::builder()
            .position(Vec3::new(x, -1., 0.))
            .padding(false)
            .color(Color::RED)
            .build();
        line(center, v, img);
        c += 2;
    }
    for y in 0..HEIGHT {
        let y = 2. * (y as f32 / HEIGHT as f32) - 1.;
        let v = VertexData::builder()
            .position(Vec3::new(1., y, 0.))
            .padding(false)
            .color(Color::BLUE)
            .build();
        line(center, v, img);
        let v = VertexData::builder()
            .position(Vec3::new(-1., y, 0.))
            .padding(false)
            .color(Color::YELLOW)
            .build();
        line(center, v, img);
        c += 2;
    }

    c
}

fn main() {
    init_logger();

    draw_image();
}
