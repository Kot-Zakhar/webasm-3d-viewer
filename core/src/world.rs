use crate::object::Object;
use crate::types::*;

pub struct World {
    pub objects: Vec<Object>,
    pub object_norms_and_light_angles_cos: Vec<Vec<f64>>,
    pub directional_light_direction: Vector4<f64>
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            object_norms_and_light_angles_cos: Vec::new(),
            directional_light_direction: Vector4::new(-1., -1., -1., 0.).normalize()
        }
    }

    fn is_handle_exist(&self, object_handle: usize) -> bool {
        return object_handle <= self.objects.len()
    }

    pub fn new_object(&mut self) -> u32 {
        let obj = Object {
            vertexes: Vec::new(),
            vertexes_viewvable: Vec::new(),
            faces: Vec::new(),
            face_normals: Vec::new(),

            rotation_matrix: _one(),
            scale_matrix: _one(),
            translation_matrix: _one()
        };

        self.objects.push(obj);
        self.object_norms_and_light_angles_cos.push(Vec::new());
        (self.objects.len() - 1) as u32
    }

    pub fn add_object_vertex(&mut self, object_handle: usize, x:f64, y:f64, z:f64, w:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].add_vertex(x, y, z, w);
    }

    pub fn add_object_face(&mut self, object_handle: usize, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) {
        if !self.is_handle_exist(object_handle) { return }

        let i = self.objects[object_handle as usize].add_face(v0, vt0, vn0, v1, vt1, vn1, v2, vt2, vn2);
        let cos = self.objects[object_handle as usize].face_normals[i].dot(&(-self.directional_light_direction));
        self.object_norms_and_light_angles_cos[object_handle as usize].push(cos);
    }

    pub fn set_object_rotation(&mut self, object_handle: usize, angle_x: f64, angle_y: f64, angle_z: f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle].set_rotation(angle_x, angle_y, angle_z);

        self.update_object_normals(object_handle);
    }

    pub fn set_object_scale(&mut self, object_handle: usize, scale: f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].set_scale(scale);
    }

    pub fn set_object_translaiton(&mut self, object_handle: usize, x: f64, y: f64, z:f64) {
        if !self.is_handle_exist(object_handle) { return }

        self.objects[object_handle as usize].set_translaiton(x, y, z);
    }

    fn update_object_normals(&mut self, object_handle: usize) {
        let obj = &mut self.objects[object_handle];
        for (face_index, face) in obj.faces.iter_mut().enumerate() {
            let rotated_normal = obj.rotation_matrix * obj.face_normals[face_index];
            let cos = rotated_normal.dot(&(-self.directional_light_direction));
            self.object_norms_and_light_angles_cos[object_handle][face_index] = cos;
        }
    }

    fn update_objects_normals(&mut self) {
        for i in 0..self.objects.len() {
            self.update_object_normals(i);
        }
    }

    pub fn set_directional_light_direction(&mut self, light_direction: Vector3<f64>) {
        self.directional_light_direction = light_direction.to_homogeneous();
        self.update_objects_normals();
    }

}