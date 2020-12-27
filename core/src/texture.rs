use crate::types::*;

pub struct Texture<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>
}

impl<T> Texture<T>
where T : Clone
{
    pub fn new() -> Texture<T> {
        Texture {
            width: 0,
            height: 0,
            data: Vec::new()
        }
    }

    pub fn set_size(&mut self, width: usize, height: usize, init_value: T) {
        self.width = width;
        self.height = height;
        self.data.resize(width * height, init_value);
    }

    pub fn get_data_pointer(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn get_pixel(&self, mut u: f64, mut v: f64) -> &T {
    // TODO: this function sucks because of these ifs
    // u [0:1] stands for x; v [0:1] stands for y
        if u <= 0. { u = 0.001 }
        if v <= 0. { v = 0.001 }
        if u >= 1. { u = 0.999 }
        if v >= 1. { v = 0.999 }
        let x = (u * (self.width as f64)) as usize;
        let y = ((1. - v) * (self.height as f64)) as usize;
        &self.data[y * self.width + x]
    }
}