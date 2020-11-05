use crate::types::*;

pub struct Face {
    pub vertices_indexes: Vector3<usize>,
    pub texture_vertices_indexes: Vector3<usize>,
    pub vertices_normals_indexes: Vector3<usize>,
    pub normal: Vector4<f64>
}

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub vertices_normals: Vec<Vector4<f64>>,
    pub vertices_viewvable: Vec<bool>,
    pub faces: Vec<Face>,
    
    // world_position stuff
    pub rotation_matrix: Matrix4<f64>,
    pub scale_matrix: Matrix4<f64>,
    pub translation_matrix: Matrix4<f64>
}

impl Object {
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertices.push(Vertex::new(x, y, z, 1.));
        self.vertices_viewvable.push(true);
    }

    pub fn add_vertex_normal(&mut self, x: f64, y: f64, z: f64) {
        self.vertices_normals.push(Vertex::new(x, y, z, 0.));
    }

    pub fn add_face(&mut self, v0: usize, vt0: usize, vn0: usize, v1: usize, vt1: usize, vn1: usize, v2: usize, vt2: usize, vn2: usize) -> usize {
        let a = Vector3::from_homogeneous(self.vertices[v1] - self.vertices[v0]).unwrap();
        let b = Vector3::from_homogeneous(self.vertices[v2] - self.vertices[v0]).unwrap();
        self.faces.push(
            Face{
                vertices_indexes: Vector3::new(v0, v1,v2),
                texture_vertices_indexes: Vector3::new(vt0, vt1, vt2),
                vertices_normals_indexes: Vector3::new(vn0, vn1, vn2),
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