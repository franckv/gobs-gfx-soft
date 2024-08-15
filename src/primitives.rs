use glam::Vec3;
use image::{Rgba, RgbaImage};

use crate::to_screen_coord;

#[tracing::instrument(skip(img), level = "debug")]
pub fn dot(pos: Vec3, img: &mut RgbaImage, color: Rgba<u8>) {
    let pos = to_screen_coord(pos, img.width(), img.height());

    pixel(pos.x as u32, pos.y as u32, img, color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn pixel(x: u32, y: u32, img: &mut RgbaImage, color: Rgba<u8>) {
    img.put_pixel(x, y, color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn line(a: Vec3, b: Vec3, img: &mut RgbaImage, color: Rgba<u8>) {
    let pos_a = to_screen_coord(a, img.width(), img.height());
    let pos_b = to_screen_coord(b, img.width(), img.height());

    let mut x0 = pos_a.x as i32;
    let mut x1 = pos_b.x as i32;
    let mut y0 = pos_a.y as i32;
    let mut y1 = pos_b.y as i32;

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
