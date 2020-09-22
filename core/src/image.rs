use wasm_bindgen::prelude::wasm_bindgen;
use std::mem;

use crate::types::*;
use crate::world::World;
use crate::camera::Camera;
use crate::utils::set_panic_hook;
use crate::console::log;

#[wasm_bindgen]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pixels: Vec<Pixel>,
    world: World,
    camera: Camera,
    to_screen_matrix: Matrix4<f64>
}

#[wasm_bindgen]
impl Image {
    pub fn new(width: u32, height: u32) -> Image {
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
            
        let f_width = width as f64;
        let f_height = height as f64;

        let to_screen_matrix = Matrix4::new(
            f_width / 2., 0.,              0., f_width / 2.,
            0.,           - f_height / 2., 0., f_height / 2.,
            0.,           0.,              1., 0.,
            0.,           0.,              0., 1.,
        );

        let world = World::new();

        let camera = Camera::new(
            std::f64::consts::PI / 4.,
            f_width / f_height,
            0.1,
            100.,
            &Point3::new(1., 1., 1.),
            &Vector3::new(-1., -1., -1.).normalize(),
            &Vector3::new(0., 1., 0.)
        );
                        
        Image {
            width,
            height,
            pixels,
            world,
            camera,
            to_screen_matrix
        }
    }

    pub fn new_object(&mut self) -> u32 {
        self.world.new_object()
    }

    pub fn get_pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    pub fn add_object_vertex(&mut self, object_handle: u32, x:f64, y:f64, z:f64, w:f64) {
        self.world.add_object_vertex(object_handle, x, y, z, w);
    }

    pub fn add_object_face(&mut self, object_handle: u32, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) {
        self.world.add_object_face(object_handle, v0, vt0, vn0, v1, vt1, vn1, v2, vt2, vn2);
    }

    pub fn set_object_rotation(&mut self, object_handle: u32, angle_x: f64, angle_y: f64, angle_z: f64) {
        self.world.set_object_rotation(object_handle, angle_x, angle_y, angle_z);
    }

    pub fn set_object_scale(&mut self, object_handle: u32, scale: f64) {
        self.world.set_object_scale(object_handle, scale);
    }

    pub fn set_object_translaiton(&mut self, object_handle: u32, x: f64, y: f64, z:f64) {
        self.world.set_object_translaiton(object_handle, x, y, z);
    }

    // pub fn move_camera_on(&mut self, d_toward: f64, d_right: f64, d_up:f64) {
    //     self.camera.mov_on(d_toward, d_right, d_up);
    // }

    pub fn set_camera_param(&mut self, param_id: u32, param_value: f64) {
        self.camera.set_param(param_id, param_value);
    }

    fn clear_world(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = 255;
            pixel.g = 255;
            pixel.b = 255;
        }
    }

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
                index = Self::get_index(x as u32, y as u32, width);
            } else {
                index = Self::get_index(y as u32, x as u32, width);
            }
            pixels[index].r = 15;
            pixels[index].g = 100;
            pixels[index].b = 150;
            
            error2 += derror2;

            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
            x += 1;
        }
    }

    fn check_vertexes(x: f64, y: f64, z:f64) -> bool {
        x < 1. && x > -1. && y < 1. && y > -1. && z > 0. && z < 1.
    }

    pub fn compute(&mut self) {
        set_panic_hook();
        self.clear_world();

        self.camera.tick();

        let look_at = self.camera.look_at_matrix;

        let projection = self.camera.projection_matrix;

        let to_screen = self.to_screen_matrix;

        let object_independent_matrix = projection * look_at;
        
        for obj in self.world.objects.iter_mut() {
            let to_world = obj.translation_matrix * obj.scale_matrix * obj.rotation_matrix;
            let final_matrix = object_independent_matrix * to_world;

            let view_vertexes: Vec<Vertex> = obj.vertexes.iter().map(|vertex| {
                let v = final_matrix * *vertex;
                v / v[(3, 0)]
            }).collect();

            for (i, v) in view_vertexes.iter().enumerate() {
                let x = v[(0, 0)];
                let y = v[(1, 0)];
                let z = v[(2, 0)];
                obj.vertexes_viewvable[i] = Self::check_vertexes(x, y, z);
            }

            
            for face in obj.faces.iter() {
                let i0 = face[(0, 0)] as usize;
                let i1 = face[(0, 1)] as usize;
                let i2 = face[(0, 2)] as usize;

                if !obj.vertexes_viewvable[i0] || !obj.vertexes_viewvable[i1] || !obj.vertexes_viewvable[i2] {
                    continue;
                }

                let v0 = to_screen * view_vertexes[i0];
                let v1 = to_screen * view_vertexes[i1];
                let v2 = to_screen * view_vertexes[i2];

                let v0 = v0 / v0[(3, 0)];
                let x0 = v0[(0, 0)].floor() as i32;
                let y0 = v0[(1, 0)].floor() as i32;

                let v1 = v1 / v1[(3, 0)];
                let x1 = v1[(0, 0)].floor() as i32;
                let y1 = v1[(1, 0)].floor() as i32;
                
                let v2 = v2 / v2[(3, 0)];
                let x2 = v2[(0, 0)].floor() as i32;
                let y2 = v2[(1, 0)].floor() as i32;
    

                //TODO remove this check
                // if
                // x0 >= self.width as i32 || y0 >= self.height as i32 || x0 < 0 || y0 < 0 ||
                // x1 >= self.width as i32 || y1 >= self.height as i32 || x1 < 0 || y1 < 0 ||
                // x2 >= self.width as i32 || y2 >= self.height as i32 || x2 < 0 || y2 < 0 
                // {
                //     not_printed += 1;
                //     continue;
                // }

    
                Self::draw_line(&mut self.pixels, self.width, self.height, x0, y0, x1, y1);
                Self::draw_line(&mut self.pixels, self.width, self.height, x0, y0, x2, y2);
                Self::draw_line(&mut self.pixels, self.width, self.height, x1, y1, x2, y2);
            }
    
        }

    }
}