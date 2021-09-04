use std::ops::Mul;
use super::vectors;

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4x4 {
    m11:f32,
    m12:f32,
    m13:f32,
    m14:f32,
    m21:f32,
    m22:f32,
    m23:f32,
    m24:f32,
    m31:f32,
    m32:f32,
    m33:f32,
    m34:f32,
    m41:f32,
    m42:f32,
    m43:f32,
    m44:f32
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ])
    }

    fn from_column_major(data: [[f32; 4]; 4]) -> Self {
        Self {
            m11:data[0][0],
            m12:data[1][0],
            m13:data[2][0],
            m14:data[3][0],
            m21:data[0][1],
            m22:data[1][1],
            m23:data[2][1],
            m24:data[3][1],
            m31:data[0][2],
            m32:data[1][2],
            m33:data[2][2],
            m34:data[3][2],
            m41:data[0][3],
            m42:data[1][3],
            m43:data[2][3],
            m44:data[3][3] 
        }
    }

    pub fn into_column_major(self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m21, self.m31, self.m41],  
            [self.m12, self.m22, self.m32, self.m42],  
            [self.m13, self.m23, self.m33, self.m43],  
            [self.m14, self.m24, self.m34, self.m44],  
        ]
    }
    
    pub fn y_rotation(angle: f32) -> Matrix4x4 {
        Matrix4x4::from_column_major([
            [angle.cos(), 0.0, -angle.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [angle.sin(), 0.0, angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ])
    }

    pub fn x_rotation(angle: f32) -> Self {
        Self::from_column_major([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle.cos(), -angle.sin(), 0.0],
            [0.0, angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ])
    }

    pub fn translation(to_translate: vectors::Vector4) -> Self {
        Self::from([
            [1.0, 0.0, 0.0, to_translate.x],
            [0.0, 1.0, 0.0, to_translate.y],
            [0.0, 0.0, 1.0, to_translate.z],
            [0.0, 0.0, 0.0, to_translate.w]
        ])
    }

    pub fn perspective(width: u32, height: u32) -> Self {
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = super::PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        Self::from_column_major([
            [f *   aspect_ratio   ,    0.0,              0.0                      ,   0.0],
            [         0.0         ,     f ,              0.0                      ,   0.0],
            [         0.0         ,    0.0,  (zfar + znear) / (zfar - znear)      ,   1.0],
            [         0.0         ,    0.0, -(2.0 * zfar * znear) / (zfar - znear),   0.0],
        ])
    }

    pub fn view(position: vectors::Vector4, direction: vectors::Vector4, up: vectors::Vector4) -> Self {
        let f = direction.normalise();

        let s = vectors::Vector4::direction(
                up.y * f.z - up.z * f.y,
                up.z * f.x - up.x * f.z,
                up.x * f.y - up.y * f.x)
            .normalise();

        let u = vectors::Vector4::direction(
                f.y * s.z - f.z * s.y,
                f.z * s.x - f.x * s.z,
                f.x * s.y - f.y * s.x
            );

        let p =  vectors::Vector4::direction(
            -position.x * s.x - position.y * s.y - position.z * s.z,
            -position.x * u.x - position.y * u.y - position.z * u.z,
            -position.x * f.x - position.y * f.y - position.z * f.z);

            Self::from_column_major([
            [s.x, u.x, f.x, 0.0],
            [s.y, u.y, f.y, 0.0],
            [s.z, u.z, f.z, 0.0],
            [p.x, p.y, p.z, 1.0],
        ])
    }
}

impl From<[[f32; 4]; 4]> for Matrix4x4 {
    fn from(data: [[f32; 4]; 4]) -> Self {
        Self {
            m11:data[0][0],
            m12:data[0][1],
            m13:data[0][2],
            m14:data[0][3],
            m21:data[1][0],
            m22:data[1][1],
            m23:data[1][2],
            m24:data[1][3],
            m31:data[2][0],
            m32:data[2][1],
            m33:data[2][2],
            m34:data[2][3],
            m41:data[3][0],
            m42:data[3][1],
            m43:data[3][2],
            m44:data[3][3] 
        }
    }
}

impl Into<[[f32; 4]; 4]> for Matrix4x4 {
    fn into(self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m12, self.m13, self.m14],  
            [self.m21, self.m22, self.m23, self.m24],  
            [self.m31, self.m32, self.m33, self.m34],  
            [self.m41, self.m42, self.m43, self.m44],  
        ]
    }
}

impl Mul<vectors::Vector4> for Matrix4x4 {
    type Output = vectors::Vector4;

    fn mul(self, rhs: vectors::Vector4) -> Self::Output {
        vectors::Vector4::position(
            (self.m11 * rhs.x) + (self.m12 * rhs.y) + (self.m13 * rhs.z) + (self.m14 * rhs.w),
            (self.m21 * rhs.x) + (self.m22 * rhs.y) + (self.m23 * rhs.z) + (self.m24 * rhs.w),
            (self.m31 * rhs.x) + (self.m32 * rhs.y) + (self.m33 * rhs.z) + (self.m34 * rhs.w)
        )
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from([
            [
                (self.m11 * rhs.m11) + (self.m12 * rhs.m21) + (self.m13 * rhs.m31) + (self.m14 * rhs.m41), 
                (self.m11 * rhs.m12) + (self.m12 * rhs.m22) + (self.m13 * rhs.m32) + (self.m14 * rhs.m42),
                (self.m11 * rhs.m13) + (self.m12 * rhs.m23) + (self.m13 * rhs.m33) + (self.m14 * rhs.m43),
                (self.m11 * rhs.m14) + (self.m12 * rhs.m24) + (self.m13 * rhs.m34) + (self.m14 * rhs.m44)
            ],
            [
                (self.m21 * rhs.m11) + (self.m22 * rhs.m21) + (self.m23 * rhs.m31) + (self.m24 * rhs.m41), 
                (self.m21 * rhs.m12) + (self.m22 * rhs.m22) + (self.m23 * rhs.m32) + (self.m24 * rhs.m42),
                (self.m21 * rhs.m13) + (self.m22 * rhs.m23) + (self.m23 * rhs.m33) + (self.m24 * rhs.m43),
                (self.m21 * rhs.m14) + (self.m22 * rhs.m24) + (self.m23 * rhs.m34) + (self.m24 * rhs.m44)
            ],
            [
                (self.m31 * rhs.m11) + (self.m32 * rhs.m21) + (self.m33 * rhs.m31) + (self.m34 * rhs.m41), 
                (self.m31 * rhs.m12) + (self.m32 * rhs.m22) + (self.m33 * rhs.m32) + (self.m34 * rhs.m42),
                (self.m31 * rhs.m13) + (self.m32 * rhs.m23) + (self.m33 * rhs.m33) + (self.m34 * rhs.m43),
                (self.m31 * rhs.m14) + (self.m32 * rhs.m24) + (self.m33 * rhs.m34) + (self.m34 * rhs.m44)
            ],
            [
                (self.m41 * rhs.m11) + (self.m42 * rhs.m21) + (self.m43 * rhs.m31) + (self.m44 * rhs.m41), 
                (self.m41 * rhs.m12) + (self.m42 * rhs.m22) + (self.m43 * rhs.m32) + (self.m44 * rhs.m42),
                (self.m41 * rhs.m13) + (self.m42 * rhs.m23) + (self.m43 * rhs.m33) + (self.m44 * rhs.m43),
                (self.m41 * rhs.m14) + (self.m42 * rhs.m24) + (self.m43 * rhs.m34) + (self.m44 * rhs.m44)
            ]
        ])
    }
}

#[test]
fn from_f324x4_parses_in_row_major() {
    let from = [
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 2.0],
        [3.0, 3.0, 3.0, 3.0],
    ];

    let matrix = Matrix4x4::from(from);
   
    assert_eq!(matrix.m11, 0.0);
    assert_eq!(matrix.m12, 0.0);
    assert_eq!(matrix.m13, 0.0);
    assert_eq!(matrix.m14, 0.0);
    assert_eq!(matrix.m21, 1.0);
    assert_eq!(matrix.m22, 1.0);
    assert_eq!(matrix.m23, 1.0);
    assert_eq!(matrix.m24, 1.0);
    assert_eq!(matrix.m31, 2.0);
    assert_eq!(matrix.m32, 2.0);
    assert_eq!(matrix.m33, 2.0);
    assert_eq!(matrix.m34, 2.0);
    assert_eq!(matrix.m41, 3.0);
    assert_eq!(matrix.m42, 3.0);
    assert_eq!(matrix.m43, 3.0);
    assert_eq!(matrix.m44, 3.0);
}

#[test]
fn from_column_major_parses_to_row_major() {
    let from = [
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0],
    ];

    let matrix = Matrix4x4::from_column_major(from);
   
    assert_eq!(matrix.m11, 0.0);
    assert_eq!(matrix.m12, 0.0);
    assert_eq!(matrix.m13, 0.0);
    assert_eq!(matrix.m14, 0.0);
    assert_eq!(matrix.m21, 1.0);
    assert_eq!(matrix.m22, 1.0);
    assert_eq!(matrix.m23, 1.0);
    assert_eq!(matrix.m24, 1.0);
    assert_eq!(matrix.m31, 2.0);
    assert_eq!(matrix.m32, 2.0);
    assert_eq!(matrix.m33, 2.0);
    assert_eq!(matrix.m34, 2.0);
    assert_eq!(matrix.m41, 3.0);
    assert_eq!(matrix.m42, 3.0);
    assert_eq!(matrix.m43, 3.0);
    assert_eq!(matrix.m44, 3.0);
}

#[test]
fn into_column_major_goes_to_column_major() {
    let from =[
        [0.0, 0.0, 0.0, 0.0],
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 2.0],
        [3.0, 3.0, 3.0, 3.0],
    ];

    let matrix = Matrix4x4::from(from);

    let to: [[f32; 4]; 4] = matrix.into_column_major();
   
    let column_major = [
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 1.0, 2.0, 3.0],
    ];

    assert_eq!(to, column_major);
}

#[test]
fn translation_matrix4_translates_vector_correctly() {
    let position = vectors::Vector4::position(1.0, 1.0, 1.0);
    let translation = vectors::Vector4::position(1.0, 2.0, 3.0);
    
    let translated = Matrix4x4::translation(translation) * position;
    assert_eq!(translated, vectors::Vector4::position(2.0, 3.0, 4.0)); 
}