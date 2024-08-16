use glam::IVec2;

use gobs_core::Color;
use gobs_resource::geometry::VertexData;

use crate::{
    math::{barycentric_coords, to_screen_coord},
    Depth, FragmentShader, Image,
};

#[tracing::instrument(skip(img), level = "debug")]
pub fn pixel(x: i32, y: i32, img: &mut Image, color: Color) {
    if x >= 0 && y >= 0 {
        img.put_pixel(x as u32, y as u32, color);
    }
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn dot(v: VertexData, img: &mut Image) {
    let pos = to_screen_coord(v.position, img.width(), img.height());

    pixel(pos.x, pos.y, img, v.color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn line(v0: VertexData, v1: VertexData, img: &mut Image) {
    let mut v0 = v0;
    let mut v1 = v1;

    let v0_s = to_screen_coord(v0.position, img.width(), img.height());
    let v1_s = to_screen_coord(v1.position, img.width(), img.height());

    let mut x0 = v0_s.x;
    let mut x1 = v1_s.x;
    let mut y0 = v0_s.y;
    let mut y1 = v1_s.y;

    let mut dx = x1 - x0;
    let mut dy = y1 - y0;

    let mut swapped = false;

    if dy.abs() > dx.abs() {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        (dx, dy) = (dy, dx);
        swapped = true;
    }

    if dx < 0 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
        (dx, dy) = (-dx, -dy);
        (v0, v1) = (v1, v0);
    }

    let step = 1. / dx as f32;

    tracing::debug!(x0, y0, x1, y1, dx, dy, step, swapped, "Line");

    for x in x0..=x1 {
        let t = (x - x0) as f32 * step;
        let y = y0 + (t * dy as f32) as i32;
        let color = v1.color * t + v0.color * (1. - t);
        if swapped {
            pixel(y, x, img, color);
        } else {
            pixel(x, y, img, color);
        }
    }
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn triangle_wire(v0: VertexData, v1: VertexData, v2: VertexData, img: &mut Image) {
    line(v0, v1, img);
    line(v1, v2, img);
    line(v2, v0, img);
}

#[tracing::instrument(skip(img, depth, shader), level = "debug")]
pub fn triangle(
    v0: VertexData,
    v1: VertexData,
    v2: VertexData,
    img: &mut Image,
    depth: &mut Depth,
    shader: &FragmentShader,
) {
    let v0_s = to_screen_coord(v0.position, img.width(), img.height());
    let v1_s = to_screen_coord(v1.position, img.width(), img.height());
    let v2_s = to_screen_coord(v2.position, img.width(), img.height());

    let min_x = v0_s
        .x
        .min(v1_s.x)
        .min(v2_s.x)
        .min(img.width() as i32 - 1)
        .max(0);
    let min_y = v0_s
        .y
        .min(v1_s.y)
        .min(v2_s.y)
        .min(img.height() as i32 - 1)
        .max(0);
    let max_x = v0_s
        .x
        .max(v1_s.x)
        .max(v2_s.x)
        .min(img.width() as i32 - 1)
        .max(0);
    let max_y = v0_s
        .y
        .max(v1_s.y)
        .max(v2_s.y)
        .min(img.height() as i32 - 1)
        .max(0);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = IVec2::new(x, y);

            let w = barycentric_coords(v0_s, v1_s, v2_s, p);

            if w.x >= 0. && w.y >= 0. && w.z >= 0. {
                let v = v0 * w.x + v1 * w.y + v2 * w.z;

                if depth.depth_test(x as u32, y as u32, v.position.z) {
                    let colour = shader.shade(p, &v);
                    if let Some(colour) = colour {
                        pixel(x, y, img, colour);
                    }
                }
            }
        }
    }
}
