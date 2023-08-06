use crate::utils::f2i_2;
use glam::{IVec2, Vec2, Vec3, Vec3Swizzles};
use std::cmp::{max, min};
use std::mem::swap;
use tgaimage::{TGAColor, TGAImage};

pub fn line(p0: IVec2, p1: IVec2, image: &mut TGAImage, color: &TGAColor) {
    let (mut x0, mut y0) = (p0.x, p0.y);
    let (mut x1, mut y1) = (p1.x, p1.y);
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

fn barycentric_2d(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> Vec3 {
    let s =
        Vec3::new(c.x - a.x, b.x - a.x, a.x - p.x).cross((c.y - a.y, b.y - a.y, a.y - p.y).into());
    if s.z.abs() < 1e-2 {
        (-1.0, 1.0, 1.0).into()
    } else {
        (1.0 - (s.x + s.y) / s.z, s.y / s.z, s.x / s.z).into()
    }
}

fn barycentric_3d(a: Vec3, b: Vec3, c: Vec3, p: Vec3) -> Vec3 {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let dot00 = v0.dot(v0);
    let dot01 = v0.dot(v1);
    let dot02 = v0.dot(v2);
    let dot11 = v1.dot(v1);
    let dot12 = v1.dot(v2);

    let denom = dot00 * dot11 - dot01 * dot01;

    let u = (dot11 * dot02 - dot01 * dot12) / denom;
    let v = (dot00 * dot12 - dot01 * dot02) / denom;
    let w = 1.0 - u - v;

    (u, v, w).into()
}

#[rustfmt::skip]
fn bounding_box(mut t0: IVec2, mut t1: IVec2, mut t2: IVec2, clamp: IVec2) -> (IVec2, IVec2) {
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
    ((min_x, min_y).into(), (max_x, max_y).into())
}

pub fn triangle(t: [Vec3; 3], z_buffer: &mut [f32], image: &mut TGAImage, color: &TGAColor) {
    assert_eq!(
        z_buffer.len(),
        image.width() * image.height(),
        "z_buffer size error"
    );
    let [t0, t1, t2] = t;
    let (bbox_min, bbox_max) = bounding_box(
        f2i_2(t0.xy()),
        f2i_2(t1.xy()),
        f2i_2(t2.xy()),
        (image.width() as i32 - 1, image.height() as i32 - 1).into(),
    );

    for x in bbox_min.x..=bbox_max.x {
        for y in bbox_min.y..=bbox_max.y {
            let bc_screen = barycentric_2d(t0.xy(), t1.xy(), t2.xy(), (x as f32, y as f32).into());
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
    t: [Vec3; 3],
    uv: [Vec2; 3],
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
    let (bbox_min, bbox_max) = bounding_box(
        f2i_2(t0.xy()),
        f2i_2(t1.xy()),
        f2i_2(t2.xy()),
        (image.width() as i32 - 1, image.height() as i32 - 1).into(),
    );
    for x in bbox_min.x..=bbox_max.x {
        for y in bbox_min.y..=bbox_max.y {
            let bc_screen = barycentric_2d(t0.xy(), t1.xy(), t2.xy(), (x as f32, y as f32).into());
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            let z =
                t0.z as f32 * bc_screen.x + t1.z as f32 * bc_screen.y + t2.z as f32 * bc_screen.z;
            let uv_p = uv[0] * bc_screen.x + uv[1] * bc_screen.y + uv[2] * bc_screen.z;
            let uv_p = Vec2::new(1.0, 1.0) - uv_p;
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
                TGAColor::Rgba(c) => TGAColor::rgb(
                    (c.r as f32 * intensity) as u8,
                    (c.g as f32 * intensity) as u8,
                    (c.b as f32 * intensity) as u8,
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

#[cfg(test)]
mod tests {
    use crate::draw::{line, triangle, triangle_texture};
    use glam::{IVec2, Vec2, Vec3};
    use obj::Obj;
    use tgaimage::{TGAColor, TGAImage};

    #[test]
    fn test_line() {
        let obj = Obj::load("obj/african_head.obj").unwrap();
        let width = 1000;
        let height = 1000;
        let mut image = TGAImage::new(width, height, 3);
        let vertex = obj.data.position;
        let trans = |v: [f32; 3]| -> IVec2 {
            IVec2::new(
                ((v[0] + 1.0) * width as f32 / 2.0) as i32,
                ((v[1] + 1.0) * height as f32 / 2.0) as i32,
            )
        };

        for object in obj.data.objects {
            for group in object.groups {
                for poly in group.polys {
                    let vec3 = poly.0;
                    for i in 0..3 {
                        let v0 = vertex[vec3[i].0 as usize];
                        let v1 = vertex[vec3[(i + 1) % 3].0 as usize];
                        let v0 = trans(v0);
                        let v1 = trans(v1);
                        line(v0, v1, &mut image, &TGAColor::rgb(255, 255, 255));
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
        let light_dir = Vec3::new(0.0, 0.0, -1.0);
        let vertex = obj.data.position;
        let w2c = |v: Vec3| -> Vec3 {
            (
                (v.x + 1.0) * width as f32 / 2.0,
                (v.y + 1.0) * height as f32 / 2.0,
                v.z,
            )
                .into()
        };

        for object in obj.data.objects {
            for group in object.groups {
                let mut z_buffer = vec![f32::MIN; width * height];
                for poly in group.polys {
                    let vec3 = &poly.0;
                    let v0 = Vec3::from(vertex[vec3[0].0 as usize]);
                    let v1 = Vec3::from(vertex[vec3[1].0 as usize]);
                    let v2 = Vec3::from(vertex[vec3[2].0 as usize]);
                    let normal = (v2 - v0).cross(v1 - v0).normalize();
                    let intensity = normal.dot(light_dir);
                    if intensity > 0.0 {
                        triangle(
                            [w2c(v0), w2c(v1), w2c(v2)],
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
        let light_dir = Vec3::new(0.0, 0.0, -1.0);
        let vertex = obj.data.position;
        let coord = obj.data.texture;
        let texture = TGAImage::from_tga_file("obj/african_head_diffuse.tga");
        let w2c = |v: Vec3| -> Vec3 {
            (
                (v.x + 1.0) * width as f32 / 2.0,
                (v.y + 1.0) * height as f32 / 2.0,
                v.z,
            )
                .into()
        };

        for object in obj.data.objects {
            for group in object.groups {
                let mut z_buffer = vec![f32::MIN; width * height];
                for poly in group.polys {
                    let vec3 = &poly.0;
                    let v0 = Vec3::from(vertex[vec3[0].0 as usize]);
                    let v1 = Vec3::from(vertex[vec3[1].0 as usize]);
                    let v2 = Vec3::from(vertex[vec3[2].0 as usize]);
                    let normal = (v2 - v0).cross(v1 - v0).normalize();
                    let intensity = normal.dot(light_dir);
                    if intensity > 0.0 {
                        let uv0 = Vec2::from(coord[vec3[0].1.unwrap() as usize]);
                        let uv1 = Vec2::from(coord[vec3[1].1.unwrap() as usize]);
                        let uv2 = Vec2::from(coord[vec3[2].1.unwrap() as usize]);
                        triangle_texture(
                            [w2c(v0), w2c(v1), w2c(v2)],
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
