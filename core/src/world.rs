use crate::object::Object;
use crate::types::*;

pub struct World {
    pub objects: Vec<Object>,
    pub object_norms_and_light_angles_cos: Vec<Vec<f64>>,
    pub light_direction: Vector4<f64>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            object_norms_and_light_angles_cos: Vec::new(),
            light_direction: Vector4::new(-1., -1., -1., 0.).normalize()
        }
    }

    fn is_handle_exist(&self, object_handle: u32) -> bool {
        return object_handle <= self.objects.len() as u32
    }

    pub fn new_object(&mut self) -> u32 {
        let obj = Object {
            vertexes: Vec::new(),
            vertexes_viewvable: Vec::new(),
            faces: Vec::new(),
            normals: Vec::new(),

            rotation_matrix: _one(),
            scale_matrix: _one(),
            translation_matrix: _one()
        };

        self.objects.push(obj);
        self.object_norms_and_light_angles_cos.push(Vec::new());
        (self.objects.len() - 1) as u32
    }

    pub fn add_object_vertex(&mut self, object_handle: u32, x:f64, y:f64, z:f64, w:f64) {
        if self.is_handle_exist(object_handle) {
            self.objects[object_handle as usize].add_vertex(x, y, z, w);
        }
    }

    pub fn add_object_face(&mut self, object_handle: u32, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) {
        if self.is_handle_exist(object_handle) {
            let i = self.objects[object_handle as usize].add_face(v0, vt0, vn0, v1, vt1, vn1, v2, vt2, vn2);
            let cos = self.objects[object_handle as usize].normals[i].dot(&(-self.light_direction));
            self.object_norms_and_light_angles_cos[object_handle as usize].push(cos);
        }
    }

    pub fn set_object_rotation(&mut self, object_handle: u32, angle_x: f64, angle_y: f64, angle_z: f64) {
        if self.is_handle_exist(object_handle) {
            self.objects[object_handle as usize].set_rotation(angle_x, angle_y, angle_z);
        }
    }

    pub fn set_object_scale(&mut self, object_handle: u32, scale: f64) {
        if self.is_handle_exist(object_handle) {
            self.objects[object_handle as usize].set_scale(scale);
        }
    }

    pub fn set_object_translaiton(&mut self, object_handle: u32, x: f64, y: f64, z:f64) {
        if self.is_handle_exist(object_handle) {
            self.objects[object_handle as usize].set_translaiton(x, y, z);
        }
    }

    // pub fn set_light_direction()
    // pub fn set_light_position()

}