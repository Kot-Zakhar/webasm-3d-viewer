use wasm_bindgen::prelude::wasm_bindgen;

use crate::types::*;
use crate::raster;
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

    pub fn add_object_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64, w:f64) {
        self.world.add_object_vertex(object_handle, x, y, z, w);
    }

    pub fn add_object_face(&mut self, object_handle: usize, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) {
        self.world.add_object_face(object_handle, v0, vt0, vn0, v1, vt1, vn1, v2, vt2, vn2);
    }


    pub fn set_object_rotation(&mut self, object_handle: usize, angle_x: f64, angle_y: f64, angle_z: f64) {
        self.world.set_object_rotation(object_handle, angle_x, angle_y, angle_z);
    }

    pub fn set_object_scale(&mut self, object_handle: usize, scale: f64) {
        self.world.set_object_scale(object_handle, scale);
    }

    pub fn set_object_translaiton(&mut self, object_handle: usize, x: f64, y: f64, z:f64) {
        self.world.set_object_translaiton(object_handle, x, y, z);
    }

    pub fn set_camera_param(&mut self, param_id: u32, param_value: f64) {
        self.camera.set_param(param_id, param_value);
    }

    pub fn set_light_param(&mut self, param_id: u32, param_value: f64) {
        
    }

    fn clear_image(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = 255;
            pixel.g = 255;
            pixel.b = 255;
        }

        for z in self.z_buf.iter_mut() {
            *z = 0.;
        }
    }


    fn is_faced_towards_viewer(&v1: &Vector4<f64>, &v2: &Vector4<f64>, &v3: &Vector4<f64>) -> bool {
        (v2[0] - v1[0]) * (v3[1] - v1[1]) - (v3[0] - v1[0]) * (v2[1] - v1[1]) < 0.
    }

    pub fn compute(&mut self) {
        set_panic_hook();
        self.clear_image();

        self.camera.tick();

        let look_at = self.camera.look_at_matrix;

        let projection = self.camera.projection_matrix;

        let to_screen = self.to_screen_matrix;

        let object_independent_matrix = to_screen * projection * look_at;

        let view_box_1 = to_screen * Vertex::new(-1., 1., 0., 1.);
        let view_box_2 = to_screen * Vertex::new(1., -1., 1., 1.);

        for (object_index, obj) in self.world.objects.iter_mut().enumerate() {
            let to_world = obj.translation_matrix * obj.scale_matrix * obj.rotation_matrix;
            let final_matrix = object_independent_matrix * to_world;

            let view_vertexes: Vec<Vertex> = obj.vertexes.iter().map(|vertex| {
                let mut v = final_matrix * *vertex;
                v = v / v[3];
                v
            }).collect();

            for (i, v) in view_vertexes.iter().enumerate() {
                obj.vertexes_viewvable[i] = raster::check_vertex_in_view_box(v, &view_box_1, &view_box_2);
            }

            let color = Pixel{r:100, g:100, b:255, a:255};
            let line_color = Pixel{r:50, g:50, b:255, a:255};
            
            for (face_index, face) in obj.faces.iter().enumerate() {
                let i0 = face[(0, 0)] as usize;
                let i1 = face[(0, 1)] as usize;
                let i2 = face[(0, 2)] as usize;

                // face is completely out of the screen
                if !obj.vertexes_viewvable[i0] && !obj.vertexes_viewvable[i1] && !obj.vertexes_viewvable[i2] {
                    continue;
                }

                // not drawing faces wich are faced away from the viewer
                if !Self::is_faced_towards_viewer(&view_vertexes[i0], &view_vertexes[i1], &view_vertexes[i2]) {
                    continue;
                }

                let angle = self.world.object_norms_and_light_angles_cos[object_index][face_index];
                let face_color = Pixel{
                    r: (color.r as f64 * angle) as u8,
                    g: (color.g as f64 * angle) as u8,
                    b: (color.b as f64 * angle) as u8,
                    a: color.a
                };

                let is_partial = !(obj.vertexes_viewvable[i0] && obj.vertexes_viewvable[i1] && obj.vertexes_viewvable[i2]);

                raster::draw_face(&mut self.pixels, &mut self.z_buf, self.width as i32, self.height as i32, 
                    &view_vertexes[i0], &view_vertexes[i1], &view_vertexes[i2], is_partial, &face_color, &line_color);
            }
    
        }

    }
}