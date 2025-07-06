use crate::object::Object;
use crate::types::*;

pub struct World {
    pub objects: Vec<Object>,
    pub direct_light_direction: Vector4<f64>,
    pub direct_light_color: Color<f64>,
    pub background_light_color: Color<f64>,
    // pub ambient_coeff: f64,
    // pub diffuse_coeff: f64,
    // pub specular_coeff: f64
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            direct_light_direction: Vector4::new(-1., -1., -1., 0.).normalize(),
            direct_light_color: Color{ r: 1., g: 1., b: 1.},
            background_light_color: Color{ r: 1., g: 1., b: 1.},
            // ambient_coeff: 0.1,
            // diffuse_coeff: 0.5,
            // specular_coeff: 1.
        }
    }

    fn is_handle_exist(&self, object_handle: usize) -> bool {
        return object_handle <= self.objects.len()
    }

    pub fn new_object(&mut self) -> u32 {
        let obj = Object::new();

        self.objects.push(obj);
        (self.objects.len() - 1) as u32
    }

    pub fn add_object_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].add_vertex(x, y, z);
    }

    pub fn add_object_vertex_normal(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].add_vertex_normal(x, y, z);
    }

    pub fn add_object_texture_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].add_texture_vertex(x, y, z);
    }

    pub fn add_object_face(&mut self, object_handle: usize, v0: usize, vt0: usize, vn0: usize, v1: usize, vt1: usize, vn1: usize, v2: usize, vt2: usize, vn2: usize) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].add_face(v0, vt0, vn0, v1, vt1, vn1, v2, vt2, vn2);
    }

    pub fn set_object_rotation(&mut self, object_handle: usize, angle_x: f64, angle_y: f64, angle_z: f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_rotation(angle_x, angle_y, angle_z);
    }

    pub fn set_object_scale(&mut self, object_handle: usize, scale: f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_scale(scale);
    }

    pub fn set_object_translaiton(&mut self, object_handle: usize, x: f64, y: f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_translaiton(x, y, z);
    }

    pub fn set_object_color(&mut self, object_handle: usize, r: u8, g: u8, b: u8) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_color(r, g, b);
    }

    pub fn set_object_texture_size(&mut self, object_handle: usize, texture_index: usize, width: usize, height: usize) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_texture_size(texture_index, width, height);
    }

    pub fn set_object_use_texture(&mut self, object_handle: usize, texture_index: usize, value: bool) {
        if !self.is_handle_exist(object_handle) { return }

        match texture_index {
            1 => self.objects[object_handle].use_diffuse_texture = value,
            2 => {
                self.objects[object_handle].use_normal_texture = value;
                if value {
                    self.objects[object_handle].normalize_normal_texture();
                }
            },
            3 => self.objects[object_handle].use_specular_texture = value,
            4 => self.objects[object_handle].use_emission_texture = value,
            _ => {}
        }
    }

    pub fn get_object_texture_pixels(&mut self, object_handle: usize, texture_index: usize) -> *const Pixel {
        if !self.is_handle_exist(object_handle) { return std::ptr::null() }

        self.objects[object_handle].get_texture_pixels(texture_index)
    }
}