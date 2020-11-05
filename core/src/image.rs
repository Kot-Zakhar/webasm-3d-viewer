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
    // buffers
    z_buf: Vec<f64>,
    face_index_buffer: Vec<i32>,
    object_index_buffer: Vec<i32>,

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
        
        let z_buf = vec![1.; (width * height) as usize];
        let face_buffer = vec![-1; (width * height) as usize];
        let object_index_buffer = vec![-1; (width * height) as usize];
            
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
            10.,
            &Point3::new(1.5, 1.5, 1.5),
            &Vector3::new(-1., -1., -1.).normalize(),
            &Vector3::new(0., 1., 0.)
        );
                        
        Image {
            width,
            height,
            pixels,
            z_buf,
            face_index_buffer: face_buffer,
            object_index_buffer,
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

    pub fn add_object_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        self.world.add_object_vertex(object_handle, x, y, z);
    }

    pub fn add_object_vertex_normal(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        self.world.add_object_vertex_normal(object_handle, x, y, z);
    }

    pub fn add_object_face(&mut self, object_handle: usize, v0: usize, vt0: usize, vn0: usize, v1: usize, vt1: usize, vn1: usize, v2: usize, vt2: usize, vn2: usize) {
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
        // for pixel in self.pixels.iter_mut() {
        //     pixel.r = 255;
        //     pixel.g = 255;
        //     pixel.b = 255;
        //     pixel.a = 255;
        // }

        for z in self.z_buf.iter_mut() {
            *z = 1.;
        }
        // for i in self.face_index_buffer.iter_mut() {
        //     *i = -1;
        // }
        // for i in self.object_index_buffer.iter_mut() {
        //     *i = -1;
        // }
    }


    fn is_faced_towards_viewer(&v1: &Vector4<f64>, &v2: &Vector4<f64>, &v3: &Vector4<f64>) -> bool {
        (v2[0] - v1[0]) * (v3[1] - v1[1]) - (v3[0] - v1[0]) * (v2[1] - v1[1]) < 0.
    }

    fn check_vertex_in_view_box(&v: &Vertex, &cube_min: &Vertex, &cube_max: &Vertex) -> bool {
        v[0] > cube_min[0] && v[0] < cube_max[0] &&
        v[1] > cube_min[1] && v[1] < cube_max[1] &&
        v[2] > cube_min[2] && v[2] < cube_max[2]
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

        // let translated_to_object_direct_light_direction = object_independent_matrix * self.world.direct_light_direction;

        // translating all the vertices into the final space (camera space)
        let view_vertices: Vec<Vec<Vertex>> = self.world.objects.iter_mut().enumerate().map(|(object_index, obj)| {
            let to_world = obj.translation_matrix * obj.scale_matrix * obj.rotation_matrix;
            let final_matrix = object_independent_matrix * to_world;

            // let translated_to_object_direct_light_direction = obj.rotation_matrix.transpose() * self.world.direct_light_direction;

            let view_vertices: Vec<Vertex> = obj.vertices.iter().map(|vertex| {
                let mut v = final_matrix * *vertex;
                v = v / v[3];
                v
            }).collect();

            for (i, v) in view_vertices.iter().enumerate() {
                obj.vertices_viewvable[i] = Self::check_vertex_in_view_box(v, &view_box_1, &view_box_2);
            }

            view_vertices
        }).collect();

        let color = Pixel{r:255, g:255, b:255, a:255};
        // let line_color = Pixel{r:50, g:50, b:255, a:255};

        let mut translated_to_objects_direct_light_directions = Vec::new();
        // pre-run (not calculating the light and colors)
        for (object_index, obj) in self.world.objects.iter_mut().enumerate() {

            let translated_to_object_direct_light_direction = obj.rotation_matrix.transpose() * self.world.direct_light_direction;
            translated_to_objects_direct_light_directions.push(translated_to_object_direct_light_direction);

            for (face_index, face) in obj.faces.iter().enumerate() {
                let i0 = face.vertices_indexes[0] as usize;
                let i1 = face.vertices_indexes[1] as usize;
                let i2 = face.vertices_indexes[2] as usize;

                // face is completely out of the screen
                if !obj.vertices_viewvable[i0] && !obj.vertices_viewvable[i1] && !obj.vertices_viewvable[i2] {
                    continue;
                }

                // not drawing faces wich are faced away from the viewer
                if !Self::is_faced_towards_viewer(&view_vertices[object_index][i0], &view_vertices[object_index][i1], &view_vertices[object_index][i2]) {
                    continue;
                }

                let is_partial = !(obj.vertices_viewvable[i0] && obj.vertices_viewvable[i1] && obj.vertices_viewvable[i2]);

                raster::draw_face_on_buffer(
                    self.width as i32, self.height as i32,
                    &mut self.z_buf,
                    &mut self.face_index_buffer, face_index,
                    &mut self.object_index_buffer, object_index,
                    &view_vertices,
                    &face,
                    is_partial
                );

            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_index = raster::get_index(y, x, self.width);

                if self.z_buf[pixel_index] == 1. {
                    self.pixels[pixel_index] = Pixel {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255
                    };
                    continue;
                }

                let object_index = self.object_index_buffer[pixel_index] as usize;
                let face_index = self.face_index_buffer[pixel_index] as usize;
    
                let face = &self.world.objects[object_index].faces[face_index];

                let v1 = &view_vertices[object_index][face.vertices_indexes[0]];
                let v2 = &view_vertices[object_index][face.vertices_indexes[1]];
                let v3 = &view_vertices[object_index][face.vertices_indexes[2]];

                let vn1 = &self.world.objects[object_index].vertices_normals[face.vertices_normals_indexes[0]];
                let vn2 = &self.world.objects[object_index].vertices_normals[face.vertices_normals_indexes[1]];
                let vn3 = &self.world.objects[object_index].vertices_normals[face.vertices_normals_indexes[2]];

                let direct_light_direction = &translated_to_objects_direct_light_directions[object_index];

                let point = Vertex::new(x as f64, y as f64, self.z_buf[pixel_index], 1.);

                self.pixels[pixel_index] = raster::find_color_in_point(&point, v1, v2, v3, vn1, vn2, vn3, direct_light_direction, &color);
            }
        }

        // for (object_index, obj) in self.world.objects.iter_mut().enumerate() {
            
        //     for (face_index, face) in obj.faces.iter().enumerate() {
        //         let i0 = face.vertices_indexes[0] as usize;
        //         let i1 = face.vertices_indexes[1] as usize;
        //         let i2 = face.vertices_indexes[2] as usize;

        //         // face is completely out of the screen
        //         if !obj.vertices_viewvable[i0] && !obj.vertices_viewvable[i1] && !obj.vertices_viewvable[i2] {
        //             continue;
        //         }

        //         // not drawing faces wich are faced away from the viewer
        //         if !Self::is_faced_towards_viewer(&view_vertices[i0], &view_vertices[i1], &view_vertices[i2]) {
        //             continue;
        //         }

        //         let is_partial = !(obj.vertices_viewvable[i0] && obj.vertices_viewvable[i1] && obj.vertices_viewvable[i2]);



        //         //   Lambert shading (each polygon is colored with a single color)
        //         let angle = self.world.objects_norms_and_light_angles_cos[object_index][face_index];
        //         let face_color = Pixel{
        //             r: (color.r as f64 * angle) as u8,
        //             g: (color.g as f64 * angle) as u8,
        //             b: (color.b as f64 * angle) as u8,
        //             a: color.a
        //         };
        //         raster::draw_face(&mut self.pixels, &mut self.z_buf, self.width as i32, self.height as i32,
        //             &view_vertices,
        //             &face,
        //             is_partial, &face_color
        //         );

        //         //   Phong shading ( interpolating normals of vertices to shade the triangle )
        //         // raster::draw_face_phong(
        //         //     &mut self.pixels, &mut self.z_buf, self.width as i32, self.height as i32,
        //         //     &view_vertices, &obj.vertices_normals,
        //         //     &face,
        //         //     &translated_to_object_direct_light_direction,
        //         //     &color,
        //         //     is_partial
        //         // );


        //         // raster::draw_sides_of_face(&mut self.pixels, &mut self.z_buf, self.width, self.height, 
        //         //     &view_vertices, &face, is_partial, &line_color);
        //     }
    
        // }

    }
}