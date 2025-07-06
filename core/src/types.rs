pub use nalgebra::{
    Vector3, Vector4, Matrix4,
    Point3
};

use std::ops;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub color: Color<u8>,
    pub a: u8
}


pub type Vertex = Vector4<f64>;

pub fn _diagonal(val: f64) -> Matrix4<f64> {
    Matrix4::new(
        val, 0., 0., 0.,
        0., val, 0., 0.,
        0., 0., val, 0.,
        0., 0., 0., val
    )
}

pub fn _one() -> Matrix4<f64> {
    _diagonal(1.)
}

pub fn _zero() -> Matrix4<f64> {
    _diagonal(0.)
}

pub struct Face {
    pub vertices_indexes: Vector3<usize>,
    pub texture_vertices_indexes: Vector3<usize>,
    pub vertices_normals_indexes: Vector3<usize>,
    pub normal: Vector4<f64>
}

#[derive(Copy, Clone)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T
}

impl Color<u8> {
    pub fn to_f64(self) -> Color<f64> {
        Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
        }
    }
}

impl Color<f64> {
    pub fn to_u8(self) -> Color<u8> {
        Color {
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
        }
    }
}

impl ops::Add<Color<f64>> for Color<f64> {
    type Output = Color<f64>;

    fn add(self, add: Color<f64>) -> Color<f64> {
        Color {
            r: self.r + add.r,
            g: self.g + add.g,
            b: self.b + add.b,
        }
    }
}

impl ops::Sub<Color<f64>> for Color<f64> {
    type Output = Color<f64>;

    fn sub(self, sub: Color<f64>) -> Color<f64> {
        Color {
            r: self.r - sub.r,
            g: self.g - sub.g,
            b: self.b - sub.b,
        }
    }
}

impl ops::Neg for Color<f64> {
    type Output = Color<f64>;

    fn neg(self) -> Color<f64> {
        Color {
            r: - self.r,
            g: - self.g,
            b: - self.b,
        }
    }
}

impl ops::Mul<f64> for Color<f64> {
    type Output = Color<f64>;

    fn mul(self, mul: f64) -> Color<f64> {
        Color {
            r: self.r * mul,
            g: self.g * mul,
            b: self.b * mul,
        }
    }
}

pub const white_color: Color<u8> = Color {
    r: 255,
    g: 255,
    b: 255
};

pub const black_color: Color<u8> = Color {
    r: 0,
    g: 0,
    b: 0
};