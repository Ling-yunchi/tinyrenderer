use crate::vec::{Vec2f, Vec2i, Vec3f, Vector};
use std::cmp::{max, min};
use std::mem::swap;
use tgaimage::{TGAColor, TGAImage};

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

fn barycentric(a: Vec3f, b: Vec3f, c: Vec3f, p: Vec3f) -> Vec3f {
    let s = Vec3f::new(c.x - a.x, b.x - a.x, a.x - p.x).cross(Vec3f::new(
        c.y - a.y,
        b.y - a.y,
        a.y - p.y,
    ));
    if s.z.abs() > 1e-2 {
        Vec3f::new(1.0 - (s.x + s.y) / s.z, s.y / s.z, s.x / s.z)
    } else {
        Vec3f::new(-1.0, 1.0, 1.0)
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

pub fn triangle(t: [Vec3f; 3], z_buffer: &mut [f32], image: &mut TGAImage, color: &TGAColor) {
    assert_eq!(
        z_buffer.len(),
        image.width() * image.height(),
        "z_buffer size error"
    );
    let [t0, t1, t2] = t;
    let (t0_2d, t1_2d, t2_2d) = (
        Vec2i::new(t0.x as i32, t0.y as i32),
        Vec2i::new(t1.x as i32, t1.y as i32),
        Vec2i::new(t2.x as i32, t2.y as i32),
    );
    let (bbox_min, bbox_max) = bounding_box(
        t0_2d,
        t1_2d,
        t2_2d,
        Vec2i::new(image.width() as i32 - 1, image.height() as i32 - 1),
    );
    for x in bbox_min.x..=bbox_max.x {
        for y in bbox_min.y..=bbox_max.y {
            let bc_screen = barycentric(t0, t1, t2, Vec3f::new(x as f32, y as f32, 0.0));
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            let z =
                t0.z as f32 * bc_screen.x + t1.z as f32 * bc_screen.y + t2.z as f32 * bc_screen.z;
            let idx = (x + y * image.width() as i32) as usize;
            if z_buffer[idx] < z {
                z_buffer[idx] = z;
                image.set(x as usize, y as usize, color);
            }
        }
    }
}

pub fn triangle_texture(
    t: [Vec3f; 3],
    uv: [Vec2f; 3],
    z_buffer: &mut [f32],
    image: &mut TGAImage,
    texture: &TGAImage,
    intensity: f32,
) {
    assert_eq!(
        z_buffer.len(),
        image.width() * image.height(),
        "z_buffer size error"
    );
    let [t0, t1, t2] = t;
    let (t0_2d, t1_2d, t2_2d) = (
        Vec2i::new(t0.x as i32, t0.y as i32),
        Vec2i::new(t1.x as i32, t1.y as i32),
        Vec2i::new(t2.x as i32, t2.y as i32),
    );
    let (bbox_min, bbox_max) = bounding_box(
        t0_2d,
        t1_2d,
        t2_2d,
        Vec2i::new(image.width() as i32 - 1, image.height() as i32 - 1),
    );
    for x in bbox_min.x..=bbox_max.x {
        for y in bbox_min.y..=bbox_max.y {
            let bc_screen = barycentric(t0, t1, t2, Vec3f::new(x as f32, y as f32, 0.0));
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            let z =
                t0.z as f32 * bc_screen.x + t1.z as f32 * bc_screen.y + t2.z as f32 * bc_screen.z;
            let uv_p = uv[0] * bc_screen.x + uv[1] * bc_screen.y + uv[2] * bc_screen.z;
            let uv_p = Vec2f::new(1.0, 1.0) - uv_p;
            let color = texture.get(
                (uv_p.x * texture.width() as f32) as usize,
                (uv_p.y * texture.height() as f32) as usize,
            );
            let color = match color {
                TGAColor::Rgb(c) => TGAColor::rgb(
                    (c.r as f32 * intensity) as u8,
                    (c.g as f32 * intensity) as u8,
                    (c.b as f32 * intensity) as u8,
                ),
                TGAColor::Rgba(c) => TGAColor::rgba(
                    (c.r as f32 * intensity) as u8,
                    (c.g as f32 * intensity) as u8,
                    (c.b as f32 * intensity) as u8,
                    c.a,
                ),
            };
            let idx = (x + y * image.width() as i32) as usize;
            if z_buffer[idx] < z {
                z_buffer[idx] = z;
                image.set(x as usize, y as usize, &color);
            }
        }
    }
}

pub fn world2screen(v: Vec3f, width: i32, height: i32) -> Vec3f {
    Vec3f::new(
        (v.x + 1.0) * width as f32 / 2.0 + 0.5,
        (v.y + 1.0) * height as f32 / 2.0 + 0.5,
        v.z,
    )
}

#[cfg(test)]
mod tests {
    use crate::draw::{line, triangle, triangle_texture, world2screen, Vec3f};
    use crate::vec::{Vec2f, Vector};
    use obj::Obj;
    use tgaimage::{TGAColor, TGAImage};

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
        image.write_tga_file("african_head_line.tga", true);
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
                let mut z_buffer = vec![f32::MIN; width * height];
                for poly in group.polys {
                    let vec3 = &poly.0;
                    let v0 = Vec3f::from(vertex[vec3[0].0 as usize]);
                    let v1 = Vec3f::from(vertex[vec3[1].0 as usize]);
                    let v2 = Vec3f::from(vertex[vec3[2].0 as usize]);
                    let normal = (v2 - v0).cross(v1 - v0).normalize();
                    let intensity = normal * light_dir;
                    if intensity > 0.0 {
                        triangle(
                            [
                                world2screen(v0, width as i32, height as i32),
                                world2screen(v1, width as i32, height as i32),
                                world2screen(v2, width as i32, height as i32),
                            ],
                            z_buffer.as_mut_slice(),
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
        image.write_tga_file("african_head_triangle.tga", true);
    }

    #[test]
    fn test_texture() {
        let obj = Obj::load("obj/african_head.obj").unwrap();
        let width = 1000;
        let height = 1000;
        let mut image = TGAImage::new(width, height, 3);
        let light_dir = Vec3f::new(0.0, 0.0, -1.0);
        let vertex = obj.data.position;
        let coord = obj.data.texture;
        let texture = TGAImage::from_tga_file("obj/african_head_diffuse.tga");
        for object in obj.data.objects {
            for group in object.groups {
                let mut z_buffer = vec![f32::MIN; width * height];
                for poly in group.polys {
                    let vec3 = &poly.0;
                    let v0 = Vec3f::from(vertex[vec3[0].0 as usize]);
                    let v1 = Vec3f::from(vertex[vec3[1].0 as usize]);
                    let v2 = Vec3f::from(vertex[vec3[2].0 as usize]);
                    let normal = (v2 - v0).cross(v1 - v0).normalize();
                    let intensity = normal * light_dir;
                    if intensity > 0.0 {
                        let uv0 = Vec2f::from(coord[vec3[0].1.unwrap() as usize]);
                        let uv1 = Vec2f::from(coord[vec3[1].1.unwrap() as usize]);
                        let uv2 = Vec2f::from(coord[vec3[2].1.unwrap() as usize]);
                        triangle_texture(
                            [
                                world2screen(v0, width as i32, height as i32),
                                world2screen(v1, width as i32, height as i32),
                                world2screen(v2, width as i32, height as i32),
                            ],
                            [uv0, uv1, uv2],
                            z_buffer.as_mut_slice(),
                            &mut image,
                            &texture,
                            intensity,
                        );
                    }
                }
            }
        }
        image.flip_vertically();
        image.write_tga_file("african_head_texture.tga", true);
    }
}
