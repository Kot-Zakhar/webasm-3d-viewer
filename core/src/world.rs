use crate::object::Object;
use crate::types::*;
use crate::console::log;

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
        let obj = Object {
            vertices: Vec::new(),
            vertices_normals: Vec::new(),
            vertices_viewvable: Vec::new(),
            faces: Vec::new(),
            model_color: Color{ r: 1., g: 1., b: 1. },

            // emerald
            ambient: Color{ r: 0.0215, g: 0.1745, b: 0.0215 },
            diffuse: Color{ r: 0.07568, g: 0.61424, b: 0.07568 },
            specular: Color{ r: 0.633, g: 0.727811, b: 0.633 },
            shininess: 0.6,

            // obsidian
            // ambient: Color{ r: 0.05375, g: 0.05, b: 0.06625 },
            // diffuse: Color{ r: 0.18275, g: 0.17, b: 0.22525 },
            // specular: Color{ r: 0.332741, g: 0.328634, b: 0.346435 },
            // shininess: 0.3,

            // Gold
            // ambient: Color{ r: 0.24725, g: 0.2245, b: 0.0645 },
            // diffuse: Color{ r: 0.34615, g: 0.3143, b: 0.0903 },
            // specular: Color{ r: 0.797357, g: 0.723991, b: 0.208006 },
            // shininess: 83.2,

            rotation_matrix: _one(),
            scale_matrix: _one(),
            translation_matrix: _one()
        };

        self.objects.push(obj);
        (self.objects.len() - 1) as u32
    }

    pub fn add_object_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].add_vertex(x, y, z);
    }

    pub fn add_object_vertex_normal(&mut self, object_handle: usize, x:f64, y:f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].add_vertex_normal(x, y, z);
    }

    pub fn add_object_face(&mut self, object_handle: usize, v0: usize, vt0: usize, vn0: usize, v1: usize, vt1: usize, vn1: usize, v2: usize, vt2: usize, vn2: usize) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].add_face(v0, vt0, vn0, v1, vt1, vn1, v2, vt2, vn2);
    }

    pub fn set_object_rotation(&mut self, object_handle: usize, angle_x: f64, angle_y: f64, angle_z: f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_rotation(angle_x, angle_y, angle_z);
    }

    pub fn set_object_scale(&mut self, object_handle: usize, scale: f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].set_scale(scale);
    }

    pub fn set_object_translaiton(&mut self, object_handle: usize, x: f64, y: f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].set_translaiton(x, y, z);
    }

    pub fn set_object_color(&mut self, object_handle: usize, r: u8, g: u8, b: u8) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].set_color(r, g, b);
    }
}