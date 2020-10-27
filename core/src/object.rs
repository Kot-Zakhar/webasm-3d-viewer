use crate::types::*;

pub struct Face {
    pub vertexes_indexes: Vector3<u32>,
    pub texture_vertexes_indexes: Vector3<u32>,
    pub vertexes_normals_indexes: Vector3<u32>,
    pub normal: Vector4<f64>
}

pub struct Object {
    pub vertexes: Vec<Vertex>,
    pub vertexes_normals: Vec<Vector4<f64>>,
    pub vertexes_viewvable: Vec<bool>,
    pub faces: Vec<Face>,
    
    // world_position stuff
    pub rotation_matrix: Matrix4<f64>,
    pub scale_matrix: Matrix4<f64>,
    pub translation_matrix: Matrix4<f64>
}

impl Object {
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertexes.push(Vertex::new(x, y, z, 1.));
        self.vertexes_viewvable.push(true);
    }

    pub fn add_vertex_normal(&mut self, x: f64, y: f64, z: f64) {
        self.vertexes_normals.push(Vertex::new(x, y, z, 0.));
    }

    pub fn add_face(&mut self, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) -> usize {
        let a = Vector3::from_homogeneous(self.vertexes[v1 as usize] - self.vertexes[v0 as usize]).unwrap();
        let b = Vector3::from_homogeneous(self.vertexes[v2 as usize] - self.vertexes[v0 as usize]).unwrap();
        self.faces.push(
            Face{
                vertexes_indexes: Vector3::new(v0, v1,v2),
                texture_vertexes_indexes: Vector3::new(vt0, vt1, vt2),
                vertexes_normals_indexes: Vector3::new(vn0, vn1, vn2),
                normal: a.cross(&b).normalize().to_homogeneous()
            }
        );
        self.faces.len() - 1
    }


    pub fn set_rotation(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) {
        let sinx = angle_x.sin();
        let cosx = angle_x.cos();
        let siny = angle_y.sin();
        let cosy = angle_y.cos();
        let sinz = angle_z.sin();
        let cosz = angle_z.cos();


        let rotation_x_matrix = Matrix4::new(
            1., 0.,   0.,    0.,
            0., cosx, -sinx, 0.,
            0., sinx, cosx,  0.,
            0., 0.,   0.,    1.
        );

        let rotation_y_matrix = Matrix4::new(
            cosy,  0., siny, 0.,
            0.,    1., 0.,   0.,
            -siny, 0., cosy, 0.,
            0.,    0., 0.,   1.
        );

        let rotation_z_matrix = Matrix4::new(
            cosz, -sinz, 0., 0.,
            sinz, cosz,  0., 0.,
            0.,   0.,    1., 0.,
            0.,   0.,    0., 1.
        );

        self.rotation_matrix = rotation_x_matrix * rotation_y_matrix * rotation_z_matrix;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale_matrix = Matrix4::new(
            scale, 0.,    0.,    0.,
            0.,    scale, 0.,    0.,
            0.,    0.,    scale, 0.,
            0.,    0.,    0.,    1.
        );
    }

    pub fn set_translaiton(&mut self, x: f64, y: f64, z:f64) {
        self.translation_matrix = Matrix4::new(
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.
        )
    }

}