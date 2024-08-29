use gobs_core::{Color, ImageExtent2D};
use gobs_gfx_soft::*;

#[tracing::instrument]
pub fn draw_image() {
    let mut img = Image::new(ImageExtent2D::new(WIDTH, HEIGHT), Color::BLACK);

    let mesh_data = loader::load_model("assets/model.obj");

    wire(&mesh_data, &mut img);

    img.save(FILENAME).expect("Save image");
}

fn main() {
    init_logger();

    draw_image();
}
