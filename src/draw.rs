use std::mem::swap;
use tgaimage::{TGAColor, TGAImage};

pub fn line(x0: i32, y0: i32, x1: i32, x2: i32, image: &mut TGAImage, color: &TGAColor) {
    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = x2;
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let d_error2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;
    for x in x0..x1 {
        if steep {
            image.set(y as usize, x as usize, &color);
        } else {
            image.set(x as usize, y as usize, &color);
        }
        error2 += d_error2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}