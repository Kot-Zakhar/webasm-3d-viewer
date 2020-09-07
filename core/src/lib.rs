mod utils;

extern crate nalgebra as na;
use na::{Vector4, Matrix4};
use wasm_bindgen::prelude::*;
use rand::Rng;

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub type Vertex = Vector4<f64>;
pub type TransformationMatrix = Matrix4<f64>;

#[wasm_bindgen]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    vertexes: Vec<Vertex>
}

impl Image {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl Image {
    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0, self.height);
        let col = rng.gen_range(0, self.width);

        let idx = self.get_index(row, col);
        self.pixels[idx].r = self.pixels[idx].r ^ 255;
        self.pixels[idx].g = self.pixels[idx].g ^ 255;
        self.pixels[idx].b = self.pixels[idx].b ^ 255;
    }

    pub fn new(width: u32, height: u32, vertexes_amount: u32) -> Image {
        let pixels = (0..width * height)
            .map(|_| {
                Pixel {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255
                }
            })
            .collect();
        let vertexes = (0..vertexes_amount)
            .map(|_| {
                Vertex::new(0.0, 0.0, 0.0, 0.0)
                // x, y, z, v
            })
            .collect();

        Image {
            width,
            height,
            pixels,
            vertexes
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    pub fn vertexes(&self) -> *const Vertex {
        self.vertexes.as_ptr()
    }

    pub fn move_object(&mut self) {
        let translation_matrix = TransformationMatrix::new(
            1.0, 0.0, 0.0, 1.0,
            0.0, 1.0, 0.0, 1.0,
            0.0, 0.0, 1.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        );

        for vertex in self.vertexes.iter_mut() {
            *vertex = translation_matrix * *vertex;
        }
    }
}