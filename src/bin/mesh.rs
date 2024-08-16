use gobs_core::{Color, ImageExtent2D};

use tinyrenderer::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);
    let mut depth = Depth::new(ImageExtent2D::new(WIDTH, HEIGHT));

    let shader = FragmentShader::new();

    let mesh_data = loader::load_model("assets/model.obj");

    mesh(&mesh_data, &mut img, &mut depth, &shader);

    img.save(FILENAME).expect("Save image");
    depth.save(DFILENAME).expect("Save image");
}

fn main() {
    init_logger();

    draw_image();
}
