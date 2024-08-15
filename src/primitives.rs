use glam::UVec2;
use image::RgbaImage;

use gobs_core::Color;

use crate::math::barycentric_coords;

#[tracing::instrument(skip(img), level = "debug")]
pub fn pixel(x: u32, y: u32, img: &mut RgbaImage, color: Color) {
    img.put_pixel(x, y, color.into());
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn line(a: UVec2, b: UVec2, img: &mut RgbaImage, color: Color) {
    let mut x0 = a.x as i32;
    let mut x1 = b.x as i32;
    let mut y0 = a.y as i32;
    let mut y1 = b.y as i32;

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
    }

    let step = dy.abs() * 2;
    let mut error = 0;

    tracing::debug!(x0, y0, x1, y1, dx, dy, step, swapped, "Line");
    let mut y = y0;
    let dy_inc = dy.signum();
    let error_dec = dx * 2;

    if swapped {
        for x in x0..=x1 {
            pixel(y as u32, x as u32, img, color);

            error += step;
            if error > dx {
                y += dy_inc;
                error -= error_dec;
            }
        }
    } else {
        for x in x0..=x1 {
            pixel(x as u32, y as u32, img, color);

            error += step;
            if error > dx {
                y += dy_inc;
                error -= error_dec;
            }
        }
    }
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn triangle_line(v0: UVec2, v1: UVec2, v2: UVec2, img: &mut RgbaImage, color: Color) {
    line(v0, v1, img, color);
    line(v1, v2, img, color);
    line(v2, v0, img, color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn rasterize(
    v0: UVec2,
    v1: UVec2,
    v2: UVec2,
    img: &mut RgbaImage,
    color0: Color,
    color1: Color,
    color2: Color,
) {
    let min_x = v0.x.min(v1.x).min(v2.x);
    let min_y = v0.y.min(v1.y).min(v2.y);
    let max_x = v0.x.max(v1.x).max(v2.x);
    let max_y = v0.y.max(v1.y).max(v2.y);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let p = UVec2::new(x, y);

            let w = barycentric_coords(v0, v1, v2, p);

            if w.x > 0. && w.y > 0. && w.z > 0. {
                let color = color0 * w.x + color1 * w.y + color2 * w.z;
                pixel(x as u32, y as u32, img, color);
            }
        }
    }
}
