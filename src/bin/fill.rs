use image::{ImageFormat, RgbaImage};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, BLACK);

    let mut c = 0;
    for _ in 0..LOOPS {
        c += fill(&mut img);
    }

    tracing::info!(c, "Draws");
    dot(CENTER_X, CENTER_Y, &mut img, WHITE);

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn fill(img: &mut RgbaImage) -> u32 {
    let mut c = 0;
    for x in 0..WIDTH {
        line(CENTER_X, CENTER_Y, x, 0, img, GREEN);
        line(CENTER_X, CENTER_Y, x, HEIGHT - 1, img, RED);
        c += 2;
    }
    for y in 0..HEIGHT {
        line(CENTER_X, CENTER_Y, 0, y, img, BLUE);
        line(CENTER_X, CENTER_Y, WIDTH - 1, y, img, YELLOW);
        c += 2;
    }

    c
}

fn main() {
    init_logger();

    draw_image();
}
