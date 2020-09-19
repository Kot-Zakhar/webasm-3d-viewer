use crate::types::*;

pub struct Camera {
    pub fov: f64,
    pub aspect: f64,
    pub position: RowVector3<f64>,
    // pub target: RowVector3<f64>,
    pub world_up: RowVector3<f64>,
    pub front: RowVector3<f64>,
    pub look_at_matrix: Matrix4<f64>,
    pub projection_matrix: Matrix4<f64>
}// TODO: try to make camera using FOV and aspect (угол обзора камеры)

impl Camera {
    fn compute_look_at(position: RowVector3<f64>, target: RowVector3<f64>, up: RowVector3<f64>) -> Matrix4<f64>{
        let direction = (position - target).normalize();
        let right = up.cross(&direction).normalize();
        let up = direction.cross(&right);

        let camera_rud_matrix = Matrix4::from_rows(&[
            right    .insert_column(3, 0.),
            up       .insert_column(3, 0.),
            direction.insert_column(3, 0.),
            RowVector4::new(0., 0., 0., 1.)
        ]);

        let camera_negative_position_matrix = Matrix4::new(
            1., 0., 0., -(position[0]),
            0., 1., 0., -(position[1]),
            0., 0., 1., -(position[2]),
            0., 0., 0., 1.
        );

        camera_rud_matrix * camera_negative_position_matrix

        // let position = Point3::new(position[0], position[1], position[2]);
        // let target = Point3::new(target[0], target[1], target[2]);
        // let up = Vector3::new(up[0], up[1], up[2]);

        // Matrix4::look_at_rh(&position, &target, &up)
    }

    fn compute_projection_matrix(fov: f64, aspect_ration: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
        let w = 0.5;
        let h = w / aspect_ration;

        let n = z_near;
        let f = z_far;
        let a = aspect_ration;

        // // ortographic projection
        // Matrix4::new(
        //     2. / w, 0.,          0.,                    0.,
        //     0.,         2. / h, 0.,                    0.,
        //     0.,         0.,          1. / (n - f), n / (n - f),
        //     0.,         0.,          0.,                    1.
        // )
        // perspective projection
        // Matrix4::new(
        //     2. * n / w, 0.,          0.,          0.,
        //     0.,         2. * n / h,  0.,          0.,
        //     0.,         0.,          f / (n - f), (n * f) / (n - f),
        //     0.,         0.,          -1.,         0.
        // )
        // // perspective projection from fov
        Matrix4::new(
            1. / (a * f64::tan(fov / 2.)), 0., 0., 0.,
            0., 1. / f64::tan(fov / 2.), 0., 0.,
            0., 0., f / (n - f),  n * f / (n - f),
            0., 0., -1., 0.
        )
        // Perspective3::new(a, fov, n, f).into_inner()
    }

    fn update_look_at(&mut self) {
        // self.look_at_matrix = Camera::compute_look_at(self.position, self.target, self.world_up)
        self.look_at_matrix = Camera::compute_look_at(self.position, self.position + self.front, self.world_up)
    }

    pub fn mov_on(&mut self, d_toward:f64, d_right:f64, d_up:f64) {
        // let direction = (self.position - self.target).normalize();
        let direction = - self.front;
        
        let right = self.world_up.cross(&direction).normalize();
        let up = direction.cross(&right).normalize();

        self.position += self.front * d_toward + right * d_right + up * d_up;
        self.update_look_at();
    }

    pub fn new(fov: f64, aspect: f64, z_near: f64, z_far: f64) -> Camera {
        let position = RowVector3::new(1., 1., 1.);
        // let target = RowVector3::new(0., 0., 0.);
        let front = RowVector3::new(-1., -1., -1.).normalize();
        let up = RowVector3::new(0., 1., 0.);

        Camera {
            fov,
            aspect,
            position,
            // target,
            world_up: up,
            front,
            // look_at_matrix: Camera::compute_look_at(position, target, up),
            look_at_matrix: Camera::compute_look_at(position, position + front, up),
            projection_matrix: Camera::compute_projection_matrix(fov, aspect, z_near, z_far)
        }
    }
}
