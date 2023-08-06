use std::ops::{Add, Mul, Sub};

pub trait Vec {
    fn cross(self, rhs: Self) -> Self;
    fn normalize(self) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Vec for Vec2i {
    fn cross(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.y - self.y * rhs.x, self.y * rhs.x - self.x * rhs.y)
    }

    fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y) as f32;
        let len = len.sqrt();
        Self::new((self.x as f32 / len) as i32, (self.y as f32 / len) as i32)
    }
}

impl Add for Vec2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2i {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new((self.x as f32 * rhs) as i32, (self.y as f32 * rhs) as i32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Vec for Vec3f {
    fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self::new(self.x / len, self.y / len, self.z / len)
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul for Vec3f {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl From<[f32; 3]> for Vec3f {
    fn from(arr: [f32; 3]) -> Self {
        Self::new(arr[0], arr[1], arr[2])
    }
}