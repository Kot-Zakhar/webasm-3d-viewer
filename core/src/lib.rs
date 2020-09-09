mod utils;

extern crate nalgebra as na;
use na::*;
use std::mem;
use wasm_bindgen::prelude::*;

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
    faces: Vec<Matrix3<u32>>,
    rotation_matrix: Matrix4<f64>
}

impl Image {
    fn get_index(row: u32, column: u32, width: u32) -> usize {
        (row * width + column) as usize
    }

    fn draw_line(pixels: &mut Vec<Pixel>, width: u32, height: u32, mut x0: i32, mut y0: i32, mut x1:i32, mut y1: i32) {
        
        let steep = i32::abs(x0-x1) < i32::abs(y0-y1);
        if steep {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
        }

        if x0>x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = i32::abs(dy) * 2;
        let mut error2 = 0;

        let mut y = y0;
        let mut x = x0;
        while x <= x1 {
            let index;
            if steep {
                index = Image::get_index(x as u32, y as u32, width);
            } else {
                index = Image::get_index(y as u32, x as u32, width);
            }
            pixels[index].r = 0;
            pixels[index].g = 0;
            pixels[index].b = 0;
            
            error2 += derror2;

            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
            x += 1;
        }
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
            width: width,
            height: height,
            pixels: pixels,
            vertexes: vertexes,
            view_vertexes: view_vertexes,
            faces: Vec::new(),
            rotation_matrix: rotation_matrix
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

    pub fn add_face(&mut self, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) {
        self.faces.push(Matrix3::new(
            v0, v1, v2,
            vt0, vt1, vt2,
            vn0, vn1, vn2
        ));
    }

    pub fn upscale(&mut self) {
        let scale_matrix = Matrix4::new(
            100.0, 0.0, 0.0, 0.0,
            0.0, 100.0, 0.0, 0.0,
            0.0, 0.0, 100.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        for vertex in self.vertexes.iter_mut() {
            *vertex = scale_matrix * *vertex;
        }
    }

    pub fn rotate(&mut self) {
        for vertex in self.vertexes.iter_mut() {
            *vertex = self.rotation_matrix * *vertex;
        }
    }

    pub fn translate_to_camera(&mut self) {
        utils::set_panic_hook();

        let camera_position = RowVector3::new(0.3, 0.3, 1.0);

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

    pub fn clear_image(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = 255;
            pixel.g = 255;
            pixel.b = 255;
        }
    }

    pub fn draw_dots_on_image(&mut self) {
        for vertex in self.view_vertexes.iter() {
            // let x = (vertex[0] * self.width as f64 / 2.0 + self.width as f64 / 2.0).round() as i32;
            // let y = (-vertex[1] * self.height as f64 / 2.0 + self.height as f64 / 2.0).round() as i32;
            let x = (vertex[0] + self.width as f64 / 2.0).round() as i32;
            let y = (-vertex[1] + self.height as f64 / 2.0).round() as i32;
            // let x = (vertex[0]).round() as i32;
            // let y = (-vertex[1]).round() as i32;

            if x > self.width as i32 || y >= self.height as i32 || x < 0 || y < 0 {
                continue;
            }

            let index = Image::get_index(y as u32, x as u32, self.width);
            
            self.pixels[index].r = 0;
            self.pixels[index].g = 0;
            self.pixels[index].b = 0;
        }
    }

    pub fn draw_lines_on_image(&mut self) {
        for face in self.faces.iter() {
            for n in 0..3 {
                let v0 = self.view_vertexes[face[(0, n)] as usize];
                let v1 = self.view_vertexes[face[(0, (n + 1) % 3)] as usize];

                // let x0 = (v0[0] * self.width as f64 / 2.0 + self.width as f64 / 2.0).round() as i32;
                // let y0 = (-v0[1] * self.height as f64 / 2.0 + self.height as f64 / 2.0).round() as i32;
                // let x1 = (v1[0] * self.width as f64 / 2.0 + self.width as f64 / 2.0).round() as i32;
                // let y1 = (-v1[1] * self.height as f64 / 2.0 + self.height as f64 / 2.0).round() as i32;
                let x0 = (v0[0] + self.width as f64 / 2.0).round() as i32;
                let y0 = (-v0[1] + self.height as f64 / 2.0).round() as i32;
                let x1 = (v1[0] + self.width as f64 / 2.0).round() as i32;
                let y1 = (-v1[1] + self.height as f64 / 2.0).round() as i32;
                // let x0 = (v0[0]).round() as i32;
                // let y0 = (-v0[1]).round() as i32;
                // let x1 = (v1[0]).round() as i32;
                // let y1 = (-v1[1]).round() as i32;

                if x0 > self.width as i32 || y0 >= self.height as i32 || x0 < 0 || y0 < 0 ||
                x1 > self.width as i32 || y1 >= self.height as i32 || x1 < 0 || y1 < 0 {
                    continue;
                }

                Image::draw_line(&mut self.pixels, self.width, self.height, x0, y0, x1, y1);
            }
        }
    }
}