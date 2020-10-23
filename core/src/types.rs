pub use nalgebra::{
    Vector3, Vector4,
    Matrix3, Matrix4,
    RowVector3, RowVector4,
    Point3, Point4,
    Perspective3
};

#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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