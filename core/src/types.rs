pub use nalgebra::{
    Vector3, Vector4,
    Matrix3, Matrix4,
    RowVector3, RowVector4,
    Point3, Point4,
    Perspective3
};

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