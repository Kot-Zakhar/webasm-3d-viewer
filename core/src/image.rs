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
    emission_buf: Vec<Pixel>,
    pingpong_emission_buf: Vec<Pixel>,
    z_buf: Vec<f64>,
    face_index_buffer: Vec<i32>,
    object_index_buffer: Vec<i32>,

    world: World,
    camera: Camera,
    to_screen_matrix: Matrix4<f64>
}

static blur_weights: &'static [f64] = &[0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216];

#[wasm_bindgen]
impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let pixels = (0..width * height)
            .map(|_| {
                Pixel {
                    color: Color {
                        r: 255,
                        g: 255,
                        b: 255,           
                    },
                    a: 255
                }
            })
            .collect();
        
        let emission_buf = (0..width * height)
        .map(|_| {
            Pixel {
                color: Color {
                    r: 0,
                    g: 0,
                    b: 0,           
                },
                a: 255
            }
        })
        .collect();

        let pingpong_emission_buf = (0..width * height)
        .map(|_| {
            Pixel {
                color: Color {
                    r: 0,
                    g: 0,
                    b: 0,           
                },
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
            emission_buf,
            pingpong_emission_buf,
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
        // self.emission_buf.as_ptr()
    }

    pub fn add_object_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        self.world.add_object_vertex(object_handle, x, y, z);
    }

    pub fn add_object_vertex_normal(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        self.world.add_object_vertex_normal(object_handle, x, y, z);
    }

    pub fn add_object_texture_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        self.world.add_object_texture_vertex(object_handle, x, y, z);
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
    
    pub fn set_object_color(&mut self, object_handle: usize, r: u8, g: u8, b: u8) {
        self.world.set_object_color(object_handle, r, g, b);
    }

    pub fn set_object_texture_size(&mut self, object_handle: usize, texture_index: usize, width: usize, height: usize) {
        self.world.set_object_texture_size(object_handle, texture_index, width, height);
    }

    pub fn set_object_use_texture(&mut self, object_handle: usize, texture_index: usize, value: bool) {
        self.world.set_object_use_texture(object_handle, texture_index, value);
    }

    pub fn get_object_texture_pixels(&mut self, object_handle: usize, texture_index: usize) -> *const Pixel {
        self.world.get_object_texture_pixels(object_handle, texture_index)
    }


    pub fn set_camera_param(&mut self, param_id: u32, param_value: f64) {
        self.camera.set_param(param_id, param_value);
    }

    pub fn set_light_param(&mut self, param_id: u32, param_value: f64) {

    }

    fn clear_image(&mut self) {
        // for pixel in self.pixels.iter_mut() {
        //     pixel.color.r = 0;
        //     pixel.color.g = 0;
        //     pixel.color.b = 0;
        //     pixel.a = 255;
        // }
        // for pixel in self.emission_buf.iter_mut() {
        //     pixel.color.r = 0;
        //     pixel.color.g = 0;
        //     pixel.color.b = 0;
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

        let mut world_to_object_translations = Vec::new();
        let mut cameras_in_object_space = Vec::new();
        let mut lights_in_object_space = Vec::new();

        // translating all the vertices into the final space (camera space)
        let mut vertices_linear_z: Vec<Vec<f64>> = Vec::new();
        let view_vertices: Vec<Vec<Vertex>> = self.world.objects.iter_mut().map(|obj|{
            let to_world = obj.translation_matrix * obj.scale_matrix * obj.rotation_matrix;
            world_to_object_translations.push(to_world.try_inverse().unwrap());

            let final_matrix = object_independent_matrix * to_world;
            let mut object_vertices_linear_z: Vec<f64> = Vec::new();
            let view_vertices: Vec<Vertex> = obj.vertices.iter().map(|vertex| {
                let v = final_matrix * *vertex;
                object_vertices_linear_z.push(v[2]);
                v / v[3]
            }).collect();

            for (i, v) in view_vertices.iter().enumerate() {
                obj.vertices_viewvable[i] = Self::check_vertex_in_view_box(v, &view_box_1, &view_box_2);
            }

            vertices_linear_z.push(object_vertices_linear_z);
            view_vertices
        }).collect();


        // pre-run (not calculating the light and colors)
        for (object_index, obj) in self.world.objects.iter_mut().enumerate() {

            cameras_in_object_space.push(world_to_object_translations[object_index] * self.camera.position.to_homogeneous());
            lights_in_object_space.push(world_to_object_translations[object_index] * self.world.direct_light_direction);
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

        let mut emission_buf_used = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_index = raster::get_index(y, x, self.width);

                if self.z_buf[pixel_index] == 1. {
                    self.pixels[pixel_index].color = white_color;
                    self.emission_buf[pixel_index].color = black_color;
                    continue;
                }

                let object_index = self.object_index_buffer[pixel_index] as usize;
                let face_index = self.face_index_buffer[pixel_index] as usize;
    
                let obj = &self.world.objects[object_index];
                let face = &obj.faces[face_index];

                let i1 = face.vertices_indexes[0];
                let i2 = face.vertices_indexes[1];
                let i3 = face.vertices_indexes[2];

                let view_v1 = &view_vertices[object_index][i1];
                let view_v2 = &view_vertices[object_index][i2];
                let view_v3 = &view_vertices[object_index][i3];

                let view_v1_linear_z = vertices_linear_z[object_index][i1];
                let view_v2_linear_z = vertices_linear_z[object_index][i2];
                let view_v3_linear_z = vertices_linear_z[object_index][i3];

                let model_v1 = &obj.vertices[i1];
                let model_v2 = &obj.vertices[i2];
                let model_v3 = &obj.vertices[i3];

                let in1 = face.vertices_normals_indexes[0];
                let in2 = face.vertices_normals_indexes[1];
                let in3 = face.vertices_normals_indexes[2];

                let vn1 = &obj.vertices_normals[in1];
                let vn2 = &obj.vertices_normals[in2];
                let vn3 = &obj.vertices_normals[in3];

                let it1 = face.texture_vertices_indexes[0];
                let it2 = face.texture_vertices_indexes[1];
                let it3 = face.texture_vertices_indexes[2];

                let vt1 = &obj.texture_vertices[it1];
                let vt2 = &obj.texture_vertices[it2];
                let vt3 = &obj.texture_vertices[it3];

                let direct_light_direction = &lights_in_object_space[object_index];
                let camera_position = &cameras_in_object_space[object_index];

                let view_point = Vertex::new(x as f64, y as f64, self.z_buf[pixel_index], 1.);

                let barycentric = raster::calc_barycentric(&view_point, view_v1, view_v2, view_v3);
                
                // let texture_pixel_vertex = vt1 * barycentric.x + vt2 * barycentric.y + vt3 * barycentric.z;
                let texture_pixel_vertex = raster::lerp(&barycentric, vt1, vt2, vt3, view_v1_linear_z, view_v2_linear_z, view_v3_linear_z);

                let diffuse_texture_color: Color<f64>;
                if obj.use_diffuse_texture {
                    let diffuse_texture_color_u8 = obj.diffuse_texture.get_pixel(texture_pixel_vertex[0], texture_pixel_vertex[1]).color;
                    diffuse_texture_color = Color {
                        r: diffuse_texture_color_u8.r as f64 / 255.,
                        g: diffuse_texture_color_u8.g as f64 / 255.,
                        b: diffuse_texture_color_u8.b as f64 / 255.,
                    };
                } else {
                    diffuse_texture_color = Color {
                        r: obj.model_color.r,
                        g: obj.model_color.g,
                        b: obj.model_color.b,
                    }
                }

                if obj.use_emission_texture {
                    self.emission_buf[pixel_index].color = obj.emission_texture.get_pixel(texture_pixel_vertex[0], texture_pixel_vertex[1]).color;
                    emission_buf_used = true;
                } else {
                    self.emission_buf[pixel_index].color = black_color;
                }

                let normal;
                if obj.use_normal_texture {
                    normal = *obj.normal_texture_normals.get_pixel(texture_pixel_vertex[0], texture_pixel_vertex[1]);
                } else {
                    // normal = vn1 * barycentric.x + vn2 * barycentric.y + vn3 * barycentric.z;
                    normal = raster::lerp(&barycentric, vn1, vn2, vn3, view_v1_linear_z, view_v2_linear_z, view_v3_linear_z);
                }

                let cos = normal.normalize().dot(&(-direct_light_direction.normalize()));

                // let model_point = model_v1 * barycentric.x + model_v2 * barycentric.y + model_v3 * barycentric.z;
                let model_point = raster::lerp(&barycentric, model_v1, model_v2, model_v3, view_v1_linear_z, view_v2_linear_z, view_v3_linear_z);

                let model_camera_direction = camera_position - model_point;

                let reflection_direction = direct_light_direction - 2. * (direct_light_direction).dot(&normal) * normal;

                let mut gloss_not_powered = reflection_direction.normalize().dot(&model_camera_direction.normalize());
                if gloss_not_powered < 0. {
                    gloss_not_powered = 0.;
                }
                let gloss = gloss_not_powered.powf(obj.shininess);

                let specular: &Color<f64>;
                if obj.use_specular_texture {
                    specular = obj.specular_texture_coeff.get_pixel(texture_pixel_vertex[0], texture_pixel_vertex[1])
                } else {
                    specular = &obj.specular
                }
                

                let obj = &self.world.objects[object_index];
                let bg_color = &self.world.background_light_color;
                let dl_color = &self.world.direct_light_color;

                
                self.pixels[pixel_index].color.r = ((
                    bg_color.r * obj.ambient.r +
                    dl_color.r * specular.r * obj.specular_intensity.r * gloss +
                    diffuse_texture_color.r * obj.diffuse_intensity.r * cos
                ) * 255.) as u8;

                self.pixels[pixel_index].color.g = ((
                    bg_color.g * obj.ambient.g +
                    dl_color.g * specular.g * obj.specular_intensity.g * gloss +
                    diffuse_texture_color.g * obj.diffuse_intensity.g * cos
                ) * 255.) as u8;

                self.pixels[pixel_index].color.b = ((
                    bg_color.b * obj.ambient.b +
                    dl_color.b * specular.b * obj.specular_intensity.b * gloss +
                    diffuse_texture_color.b * obj.diffuse_intensity.b * cos
                ) * 255.) as u8;

            }
        }

        if emission_buf_used {
            // bluring emission buf (gaussian blur)
            for i in 0..5 {
                //horizontal
                for x in 0..self.width {
                    for y in 0..self.height {
                        let pixel_index = raster::get_index(y, x, self.width);
                        let mut pixel_color = self.emission_buf[pixel_index].color.to_f64() * blur_weights[0];
                        for offset in 1..5 as u32 {
                            let coeff = blur_weights[offset as usize];
                            if x as i32 - offset as i32 >= 0 {
                                let index_l = raster::get_index(y, x - offset, self.width);
                                pixel_color = pixel_color + (self.emission_buf[index_l].color.to_f64() * coeff);
                            }
                            if x as i32 + (offset as i32) < self.width as i32 {
                                let index_r = raster::get_index(y, x + offset, self.width);
                                pixel_color = pixel_color + (self.emission_buf[index_r].color.to_f64() * coeff);
                            }
                        }
                        self.pingpong_emission_buf[pixel_index].color = pixel_color.to_u8();
                    }
                }

                //vertical
                for x in 0..self.width {
                    for y in 0..self.height {
                        let pixel_index = raster::get_index(y, x, self.width);
                        let mut pixel_color = self.pingpong_emission_buf[pixel_index].color.to_f64() * blur_weights[0];
                        for offset in 1..5 as u32 {
                            let coeff = blur_weights[offset as usize];
                            if y as i32 - offset as i32 >= 0 {
                                let index_l = raster::get_index(y - offset, x, self.width);
                                pixel_color = pixel_color + (self.pingpong_emission_buf[index_l].color.to_f64() * coeff);
                            }
                            if y as i32 + (offset as i32) < self.height as i32 {
                                let index_r = raster::get_index(y + offset, x, self.width);
                                pixel_color = pixel_color + (self.pingpong_emission_buf[index_r].color.to_f64() * coeff);
                            }
                        }
                        self.emission_buf[pixel_index].color = pixel_color.to_u8();
                    }
                }
            }
            let gamma = 2.2;
            let exposure = 1.;
            // combining emission with image
            for i in 0..(self.width * self.height) as usize {
                let pixel_color = self.pixels[i].color.to_f64();
                let emission_color = self.emission_buf[i].color.to_f64();
                let color = pixel_color + emission_color * 1.5;
                let final_color = Color{
                    r: (1. - ((-color.r / 255.) * exposure).exp()).powf(1./ gamma) * 255.,
                    g: (1. - ((-color.g / 255.) * exposure).exp()).powf(1./ gamma) * 255.,
                    b: (1. - ((-color.b / 255.) * exposure).exp()).powf(1./ gamma) * 255.,
                };
                self.pixels[i].color = final_color.to_u8();
            }

        }
    }
}