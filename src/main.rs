mod draw;

use obj::Obj;
use tgaimage::{TGAColor, TGAImage};
use crate::draw::line;

fn main() {
    let obj = Obj::load("obj/african_head.obj").unwrap();
    let width = 1000;
    let height = 1000;
    let mut image = TGAImage::new(width, height, 3);
    let vertex = obj.data.position;
    for object in obj.data.objects {
        for group in object.groups {
            for poly in group.polys {
                let vec3 = poly.0;
                for i in 0..3 {
                    let v0 = vertex[vec3[i].0 as usize];
                    let v1 = vertex[vec3[(i + 1) % 3].0 as usize];
                    let x0 = ((v0[0] + 1.0) * width as f32 / 2.0) as i32;
                    let y0 = ((v0[1] + 1.0) * height as f32 / 2.0) as i32;
                    let x1 = ((v1[0] + 1.0) * width as f32 / 2.0) as i32;
                    let y1 = ((v1[1] + 1.0) * height as f32 / 2.0) as i32;
                    line(x0, y0, x1, y1, &mut image, &TGAColor::rgb(255, 255, 255));
                }
            }
        }
    }
    image.flip_vertically();
    image.write_tga_file("output.tga", true);
}
