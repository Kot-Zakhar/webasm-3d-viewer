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
    z_buf: Vec<f64>,
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
        
        let z_buf = vec![0.; (width * height) as usize];
            
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
            z_buf,
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

    pub fn set_camera_param(&mut self, param_id: u32, param_value: f64) {
        self.camera.set_param(param_id, param_value);
    }

    fn clear_world(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = 255;
            pixel.g = 255;
            pixel.b = 255;
        }

        for z in self.z_buf.iter_mut() {
            *z = 0.;
        }
    }

    fn get_index(row: u32, column: u32, width: u32) -> usize {
        (row * width + column) as usize
    }

    fn draw_line<T>(pixels: &mut Vec<T>, width: u32, height: u32, mut x0: i32, mut y0: i32, mut x1:i32, mut y1: i32, &color: &T) where T: Copy {
        
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
        let direction = if y1 > y0 { 1 } else { -1 };
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
            pixels[index] = color;
            
            error2 += derror2;

            if error2 > dx {
                y += direction;
                error2 -= dx * 2;
            }
            x += 1;
        }
    }

    // fn draw_face(pixels: &mut Vec<Pixel>, width: u32, height: u32,
    fn draw_face(pixels: &mut Vec<Pixel>, z_buf: &mut Vec<f64>, width: u32, height: u32,
        v1: &Vertex, v2: &Vertex, v3: &Vertex, &color: &Pixel, &lineColor: &Pixel) {

        let mut v1 = v1;
        let mut v2 = v2;
        let mut v3 = v3;

        // sort on y
        if v2[1] < v1[1] {
            mem::swap(&mut v2,&mut v1);
        }
        if v3[1] < v1[1] {
            mem::swap(&mut v3,&mut v1);
        }
        if v3[1] < v2[1] {
            mem::swap(&mut v3,&mut v2);
        }

        let x1 = v1[0].round();
        let y1 = v1[1].round();

        let x2 = v2[0].round();
        let y2 = v2[1].round();

        let x3 = v3[0].round();
        let y3 = v3[1].round();

        let z = (v1[2] + v2[2] + v3[2]) / 3.;
        
        let mut dx13 = if y1 != y3 { (x3 - x1) / (y3 - y1) } else { 0. };
        let mut _dx13 = dx13;
        let mut dx12 = if y1 != y2 { (x2 - x1) / (y2 - y1) } else { 0. };
        let mut dx23 = if y2 != y3 { (x3 - x2) / (y3 - y2) } else { 0. };

        let mut wx1 = x1;
        let mut wx2 = x1;

        if dx13 > dx12 {
            mem::swap(&mut dx13, &mut dx12);
        }
        
        let y1 = y1 as i32;
        let x1 = x1 as i32;
        let y2 = y2 as i32;
        let x2 = x2 as i32;
        let y3 = y3 as i32;
        let x3 = x3 as i32;

        for i in y1..y2 {
            
            for j in wx1.floor() as i32 .. wx2.floor() as i32 {
                let index = Self::get_index(i as u32, j as u32, width);
                if z_buf[index] == 0. || z < z_buf[index] {
                    z_buf[index] = z;
                    pixels[index] = color;
                }
            }

            let border1_index = Self::get_index(i as u32, wx1.floor() as u32, width);
            if z_buf[border1_index] == 0. || z < z_buf[border1_index] {
                z_buf[border1_index] = z - 0.0001;
                pixels[border1_index] = lineColor;
            }

            let border2_index = Self::get_index(i as u32, wx2.floor() as u32, width);
            if z_buf[border2_index] == 0. || z < z_buf[border2_index] {
                z_buf[border2_index] = z - 0.0001;
                pixels[border2_index] = lineColor;
            }
            
            wx1 += dx13;
            wx2 += dx12;
        }

        if y1 == y2 {
            if x1 < x2 {
                wx1 = x1 as f64;
                wx2 = x2 as f64;
            } else {
                wx1 = x2 as f64;
                wx2 = x1 as f64;
            }
        }

        if _dx13 < dx23 {
            mem::swap(&mut _dx13, &mut dx23);
        }
        
        for i in y2..y3 {
            for j in wx1.floor() as i32 .. wx2.floor() as i32 {
                let index = Self::get_index(i as u32, j as u32, width);
                if z_buf[index] == 0. || z < z_buf[index] {
                    z_buf[index] = z;
                    pixels[index] = color;
                }
            }
            
            let border1_index = Self::get_index(i as u32, wx1.floor() as u32, width);
            if z_buf[border1_index] == 0. || z < z_buf[border1_index] {
                z_buf[border1_index] = z - 0.0001;
                pixels[border1_index] = lineColor;
            }

            let border2_index = Self::get_index(i as u32, wx2.floor() as u32, width);
            if z_buf[border2_index] == 0. || z < z_buf[border2_index] {
                z_buf[border2_index] = z - 0.0001;
                pixels[border2_index] = lineColor;
            }

            wx1 += _dx13;
            wx2 += dx23;
        }

        // Self::draw_line(pixels, width, height, x1, y1, x2, y2, &lineColor);
        // Self::draw_line(pixels, width, height, x1, y1, x3, y3, &lineColor);
        // Self::draw_line(pixels, width, height, x2, y2, x3, y3, &lineColor);
    }

    fn check_vertexes(x: f64, y: f64, z:f64) -> bool {
        x < 1. && x > -1. && y < 1. && y > -1. && z > 0. && z < 1.
    }

    fn check_vertex_in_view_box(&v: &Vertex, &cube_min: &Vertex, &cube_max: &Vertex) -> bool {
        v[0] > cube_min[0] && v[0] < cube_max[0] &&
        v[1] > cube_min[1] && v[1] < cube_max[1] &&
        v[2] > cube_min[2] && v[2] < cube_max[2]
    }

    fn is_faced_towards_viewer(&v1: &Vector4<f64>, &v2: &Vector4<f64>, &v3: &Vector4<f64>) -> bool {
        (v2[0] - v1[0]) * (v3[1] - v1[1]) - (v3[0] - v1[0]) * (v2[1] - v1[1]) >= 0.
    }

    pub fn compute(&mut self) {
        set_panic_hook();
        self.clear_world();

        self.camera.tick();

        let look_at = self.camera.look_at_matrix;

        let projection = self.camera.projection_matrix;

        let to_screen = self.to_screen_matrix;

        let object_independent_matrix = to_screen * projection * look_at;

        let view_box_1 = to_screen * Vertex::new(-1., 1., 0., 1.);
        let view_box_2 = to_screen * Vertex::new(1., -1., 1., 1.);
        // unsafe { log(&format!("front from tick:\n{} {} {}\n{} {} {}", view_box_1[0], view_box_1[1], view_box_1[2], view_box_2[0], view_box_2[1], view_box_2[2])) };

        for obj in self.world.objects.iter_mut() {
            let to_world = obj.translation_matrix * obj.scale_matrix * obj.rotation_matrix;
            let final_matrix = object_independent_matrix * to_world;


            let mut view_vertexes: Vec<Vertex> = obj.vertexes.iter().map(|vertex| {
                let mut v = final_matrix * *vertex;
                v = v / v[3];
                v
            }).collect();

            for (i, v) in view_vertexes.iter().enumerate() {
                obj.vertexes_viewvable[i] = Self::check_vertex_in_view_box(v, &view_box_1, &view_box_2);
            }

            let color = Pixel{r:100, g:100, b:100, a:255};
            let line_color = Pixel{r:50, g:50, b:255, a:255};
            
            for face in obj.faces.iter() {
                let i0 = face[(0, 0)] as usize;
                let i1 = face[(0, 1)] as usize;
                let i2 = face[(0, 2)] as usize;

                if !obj.vertexes_viewvable[i0] || !obj.vertexes_viewvable[i1] || !obj.vertexes_viewvable[i2] {
                    continue;
                }

                if Self::is_faced_towards_viewer(&view_vertexes[i0], &view_vertexes[i1], &view_vertexes[i2]) {
                    continue;
                }

                Self::draw_face(&mut self.pixels, &mut self.z_buf, self.width, self.height, &view_vertexes[i0], &view_vertexes[i1], &view_vertexes[i2], &color, &line_color);
            }
    
        }

    }
}