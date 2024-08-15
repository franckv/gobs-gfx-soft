use image::{Rgba, RgbaImage};

#[tracing::instrument(skip(img), level = "debug")]
pub fn dot(x: u32, y: u32, img: &mut RgbaImage, color: Rgba<u8>) {
    img.put_pixel(x, y, color);
}

#[tracing::instrument(skip(img), level = "debug")]
pub fn line(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut RgbaImage, color: Rgba<u8>) {
    let mut x0 = x0 as i32;
    let mut x1 = x1 as i32;
    let mut y0 = y0 as i32;
    let mut y1 = y1 as i32;

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
            dot(y as u32, x as u32, img, color);

            error += step;
            if error > dx {
                y += dy_inc;
                error -= error_dec;
            }
        }
    } else {
        for x in x0..=x1 {
            dot(x as u32, y as u32, img, color);

            error += step;
            if error > dx {
                y += dy_inc;
                error -= error_dec;
            }
        }
    }
}
