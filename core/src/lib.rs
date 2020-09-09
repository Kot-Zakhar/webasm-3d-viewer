mod utils;

extern crate nalgebra as na;
use na::*;
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
    vertexes: Vec<Vertex>,
    view_vertexes: Vec<Vertex>,
    rotation_matrix: Matrix4<f64>
}

impl Image {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl Image {
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
        
        let view_vertexes = (0..vertexes_amount)
        .map(|_| {
            Vertex::new(0.0, 0.0, 0.0, 0.0)
            // x, y, z, v
        })
        .collect();
        
        let sin = 0.1.sin();
        let cos = 0.1.cos();
        let rotation_matrix = Matrix4::new(
            cos, 0.0, sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -sin, 0.0, cos, 0.0,
            0.0, 0.0, 0.0, 1.0
        );

        Image {
            width,
            height,
            pixels,
            vertexes,
            view_vertexes,
            rotation_matrix
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
    pub fn view_vertexes(&self) -> *const Vertex {
        self.view_vertexes.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0, self.height);
        let col = rng.gen_range(0, self.width);

        let idx = self.get_index(row, col);
        self.pixels[idx].r = self.pixels[idx].r ^ 255;
        self.pixels[idx].g = self.pixels[idx].g ^ 255;
        self.pixels[idx].b = self.pixels[idx].b ^ 255;
    }

    pub fn rotate(&mut self) {
        for vertex in self.vertexes.iter_mut() {
            *vertex = self.rotation_matrix * *vertex;
        }
    }

    pub fn translate_to_camera(&mut self) {
        utils::set_panic_hook();

        let camera_position = RowVector3::new(0.0, 0.0, 1.0);

        let camera_target = RowVector3::new(0.0, 0.0, 0.0);

        let camera_direction = camera_position - camera_target;

        let up = RowVector3::new(0.0, 1.0, 0.0);

        let camera_right = up.cross(&camera_direction);
        let camera_right = camera_right.normalize();

        let camera_up = camera_direction.cross(&camera_right);

        let camera_rud_matrix = Matrix4::from_rows(&[
            camera_right    .insert_column(3, 0.0),
            camera_up       .insert_column(3, 0.0),
            camera_direction.insert_column(3, 0.0),
            RowVector4::new(0.0, 0.0, 0.0, 1.0)
        ]);

        let camera_position_matrix = Matrix4::new(
            1.0, 0.0, 0.0, -camera_position.data[0],
            0.0, 1.0, 0.0, -camera_position.data[1],
            0.0, 0.0, 1.0, -camera_position.data[2],
            0.0, 0.0, 0.0, 1.0
        );

        let look_at_matrix = camera_rud_matrix * camera_position_matrix;


        for (i, vertex) in self.vertexes.iter().enumerate() {
            self.view_vertexes[i] = look_at_matrix * *vertex;
        }
    }

    pub fn map_view_on_image(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = 255;
            pixel.g = 255;
            pixel.b = 255;
        }
        for vertex in self.view_vertexes.iter() {
            let x = (vertex.data[0] * self.width as f64 / 2.0 + self.width as f64 / 2.0).round() as u32;
            let y = (-vertex.data[1] * self.height as f64 / 2.0 + self.height as f64 / 2.0).round() as u32;

            let index = (y * self.width + x) as usize;
            if index >= self.pixels.len() {
                continue;
            }
            self.pixels[index].r = 0;
            self.pixels[index].g = 0;
            self.pixels[index].b = 0;
        }
    }

}