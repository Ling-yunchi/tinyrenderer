use std::cmp::{max, min};
use std::mem::swap;
use tgaimage::{TGAColor, TGAImage};
use crate::vec::{Vec, Vec2i, Vec3f};

pub fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    image: &mut TGAImage,
    color: &TGAColor,
) {
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

fn barycentric(a: Vec2i, b: Vec2i, c: Vec2i, p: Vec2i) -> Vec3f {
    let s = Vec3f::new(
        c.x as f32 - a.x as f32,
        b.x as f32 - a.x as f32,
        a.x as f32 - p.x as f32,
    )
        .cross(Vec3f::new(
            c.y as f32 - a.y as f32,
            b.y as f32 - a.y as f32,
            a.y as f32 - p.y as f32,
        ));
    if s.z.abs() < 1.0 {
        // triangle is degenerate, in this case return something with negative coordinates
        Vec3f::new(-1.0, 1.0, 1.0)
    } else {
        Vec3f::new(1.0 - (s.x + s.y) / s.z, s.y / s.z, s.x / s.z)
    }
}

#[rustfmt::skip]
fn bounding_box(mut t0: Vec2i, mut t1: Vec2i, mut t2: Vec2i, clamp: Vec2i) -> (Vec2i, Vec2i) {
    if t0.x > t1.x { swap(&mut t0, &mut t1); }
    if t0.x > t2.x { swap(&mut t0, &mut t2); }
    if t1.x > t2.x { swap(&mut t1, &mut t2); }
    let min_x = max(0, t0.x);
    let max_x = min(clamp.x, t2.x);
    if t0.y > t1.y { swap(&mut t0, &mut t1); }
    if t0.y > t2.y { swap(&mut t0, &mut t2); }
    if t1.y > t2.y { swap(&mut t1, &mut t2); }
    let min_y = max(0, t0.y);
    let max_y = min(clamp.y, t2.y);
    (Vec2i::new(min_x, min_y), Vec2i::new(max_x, max_y))
}

pub fn triangle(t0: Vec2i, t1: Vec2i, t2: Vec2i, image: &mut TGAImage, color: &TGAColor) {
    let (bbox_min, bbox_max) = bounding_box(
        t0,
        t1,
        t2,
        Vec2i::new(image.width() as i32, image.height() as i32),
    );
    for x in bbox_min.x..=bbox_max.x {
        for y in bbox_min.y..=bbox_max.y {
            let bc_screen = barycentric(t0, t1, t2, Vec2i::new(x, y));
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set(x as usize, y as usize, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::draw::{line, triangle, Vec2i, Vec3f};
    use obj::Obj;
    use tgaimage::{TGAColor, TGAImage};
    use crate::vec::Vec;

    #[test]
    fn test_line() {
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
        image.write_tga_file("african_head.tga", true);
    }

    #[test]
    fn test_triangle() {
        let obj = Obj::load("obj/african_head.obj").unwrap();
        let width = 1000;
        let height = 1000;
        let mut image = TGAImage::new(width, height, 3);
        let light_dir = Vec3f::new(0.0, 0.0, -1.0);
        let vertex = obj.data.position;
        for object in obj.data.objects {
            for group in object.groups {
                for poly in group.polys {
                    let vec3 = &poly.0;
                    let v0 = Vec3f::from(vertex[vec3[0].0 as usize]);
                    let v1 = Vec3f::from(vertex[vec3[1].0 as usize]);
                    let v2 = Vec3f::from(vertex[vec3[2].0 as usize]);
                    let x0 = ((v0.x + 1.0) * width as f32 / 2.0) as i32;
                    let y0 = ((v0.y + 1.0) * height as f32 / 2.0) as i32;
                    let x1 = ((v1.x + 1.0) * width as f32 / 2.0) as i32;
                    let y1 = ((v1.y + 1.0) * height as f32 / 2.0) as i32;
                    let x2 = ((v2.x + 1.0) * width as f32 / 2.0) as i32;
                    let y2 = ((v2.y + 1.0) * height as f32 / 2.0) as i32;
                    let normal = (v2 - v0).cross(v1 - v0).normalize();
                    let intensity = normal * light_dir;
                    if intensity > 0.0 {
                        triangle(
                            Vec2i::new(x0, y0),
                            Vec2i::new(x1, y1),
                            Vec2i::new(x2, y2),
                            &mut image,
                            &TGAColor::rgb(
                                (intensity * 255.0) as u8,
                                (intensity * 255.0) as u8,
                                (intensity * 255.0) as u8,
                            ),
                        );
                    }
                }
            }
        }
        image.flip_vertically();
        image.write_tga_file("african_head_1.tga", true);
    }
}
