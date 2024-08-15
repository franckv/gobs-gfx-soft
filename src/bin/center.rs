use image::{ImageFormat, RgbaImage};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = RgbaImage::from_pixel(WIDTH, HEIGHT, BLACK);

    draw_centers(&mut img);

    dot(CENTER_X, CENTER_Y, &mut img, WHITE);

    img.save_with_format("test.tga", ImageFormat::Tga)
        .expect("Save test");
}

fn draw_centers(img: &mut RgbaImage) {
    line(CENTER_X, CENTER_Y, WIDTH - 1, HEIGHT - 1, img, RED);
    line(CENTER_X, CENTER_Y, WIDTH - 1, 0, img, GREEN);
    line(CENTER_X, CENTER_Y, 0, 0, img, BLUE);
    line(CENTER_X, CENTER_Y, 0, HEIGHT - 1, img, YELLOW);
}

fn main() {
    init_logger();

    draw_image();
}
