mod draw;
mod vec;

use crate::draw::{triangle};

use tgaimage::{TGAColor, TGAImage};
use crate::vec::Vec2i;

fn main() {
    let width = 200;
    let height = 200;
    let mut image = TGAImage::new(width, height, 3);
    triangle(
        Vec2i::new(10, 10),
        Vec2i::new(100, 30),
        Vec2i::new(190, 160),
        &mut image,
        &TGAColor::rgb(255, 0, 0),
    );
    image.flip_vertically();
    image.write_tga_file("output.tga", true);
}
