use std::ops::{Add, Div, Mul, Sub};

pub struct TextureCoordinates {  
}

impl TextureCoordinates {
    pub fn bottom_left() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    pub fn bottom_right() -> Vector2 {
        Vector2::new(1.0, 0.0)
    }

    pub fn top_left() -> Vector2 {
        Vector2::new(0.0, 1.0)
    }

    pub fn top_right() -> Vector2 {
        Vector2::new(1.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32,  y: f32) -> Self {
        Self { x, y }
    }

    pub fn zeroed(&self) -> bool {
        self == &Vector2::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn up() -> Self {
        Self::direction(0.0, 1.0, 0.0)
    }

    pub fn down() -> Self {
        Self::direction(0.0, -1.0, 0.0)
    }

    pub fn back() -> Self {
        Self::direction(0.0, 0.0, 1.0)
    }

    pub fn front() -> Self {
        Self::direction(0.0, 0.0, -1.0)
    }

    pub fn top() -> Self {
        Self::direction(0.0, 1.0, 0.0)
    }

    pub fn bottom() -> Self {
        Self::direction(0.0, -1.0, 0.0)
    }

    pub fn right() -> Self {
        Self::direction(1.0, 0.0, 0.0)
    }

    pub fn left() -> Self {
        Self::direction(-1.0, 0.0, 0.0)
    }

    pub fn position(x: f32,  y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    
    pub fn direction(x: f32,  y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn length(&self) -> f32 {
        let len = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
        len.sqrt()
    } 

    pub fn normalise(&self) -> Self {
        let len = self.length();
        Self::position(self.x / len, self.y / len, self.z / len)
    }

    pub fn cross(&self, to_cross: Vector4) -> Vector4 {
        Vector4::position(
            (self.y * to_cross.z) - (self.z * to_cross.y),  
            (self.z * to_cross.x) - (self.x * to_cross.z),
            (self.x * to_cross.y) - (self.y * to_cross.x)
        )
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::position(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Self) -> Self::Output {
        Self::position(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::position(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        Self::position(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Into<[f32;2]> for Vector2 {
    fn into(self) -> [f32;2] {
        [self.x, self.y]
    }
}

impl Into<[f32;3]> for Vector4 {
    fn into(self) -> [f32;3] {
        [self.x, self.y, self.z]
    }
}

impl Into<[f32;4]> for Vector4 {
    fn into(self) -> [f32;4] {
        [self.x, self.y, self.z, self.w]
    }
}