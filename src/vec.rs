use std::ops::{Add, Mul, Sub};

pub struct Vec<const SIZE: usize, T: Default + Copy> {
    data: [T; SIZE],
}

impl<const SIZE: usize, T: Default + Copy> Vec<SIZE, T> {
    pub fn new() -> Self {
        Self {
            data: [T::default(); SIZE],
        }
    }
}

impl<const SIZE: usize, T: Default + Copy> Add for Vec<SIZE, T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] + rhs.data[i];
        }
        result
    }
}

impl<const SIZE: usize, T: Default + Copy> Sub for Vec<SIZE, T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] - rhs.data[i];
        }
        result
    }
}

impl<const SIZE: usize, T: Default + Copy> Mul<f32> for Vec<SIZE, T>
where
    T: Mul<f32, Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Self::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] * rhs;
        }
        result
    }
}

impl<const SIZE: usize, T: Default + Copy> Mul for Vec<SIZE, T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] * rhs.data[i];
        }
        result
    }
}

pub trait Vector {
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

impl Vector for Vec2i {
    fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.x * rhs.y - self.y * rhs.x,
            self.y * rhs.x - self.x * rhs.y,
        )
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

impl Mul for Vec2i {
    type Output = i32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vector for Vec2f {
    fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.x * rhs.y - self.y * rhs.x,
            self.y * rhs.x - self.x * rhs.y,
        )
    }

    fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        Self::new(self.x / len, self.y / len)
    }
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul for Vec2f {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl From<[f32; 2]> for Vec2f {
    fn from(data: [f32; 2]) -> Self {
        Self::new(data[0], data[1])
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

impl Vector for Vec3f {
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
