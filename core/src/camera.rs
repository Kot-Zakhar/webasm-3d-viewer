use crate::types::*;
use crate::console::log;

pub struct Camera {
    pub fov: f64,
    pub aspect: f64,
    pub position: Point3<f64>,
    pub world_up: Vector3<f64>,
    pub front: Vector3<f64>,
    pub look_at_matrix: Matrix4<f64>,
    pub projection_matrix: Matrix4<f64>,
    speed: Vector3<f64>,
    rotation: Vector3<f64> // pitch, yaw, scroll
}

impl Camera {
    fn compute_look_at(&position: &Point3<f64>, &target: &Point3<f64>, &world_up: &Vector3<f64>) -> Matrix4<f64>{
        let direction = (position - target).normalize();
        let right = world_up.cross(&direction).normalize();
        let up = direction.cross(&right);

        let camera_rud_matrix = Matrix4::from_columns(&[
            right.to_homogeneous(),
            up.to_homogeneous(),
            direction.to_homogeneous(),
            Point3::<f64>::origin().to_homogeneous()
        ]).transpose();

        // let camera_negative_position_matrix = Matrix4::new(
        //     1., 0., 0., -(position[0]),
        //     0., 1., 0., -(position[1]),
        //     0., 0., 1., -(position[2]),
        //     0., 0., 0., 1.
        // );
        let negative_position = -1. * position.coords;
        let camera_translation_matrix = Matrix4::new_translation(&negative_position);

        camera_rud_matrix * camera_translation_matrix
    }

    fn compute_projection_matrix(fov: f64, aspect_ration: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
        // let w = 0.5;
        // let h = w / aspect_ration;

        let n = z_near;
        let f = z_far;
        let a = aspect_ration;

        // ortographic projection
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

        // perspective projection from fov
        Matrix4::new(
            1. / (a * f64::tan(fov / 2.)), 0., 0., 0.,
            0., 1. / f64::tan(fov / 2.), 0., 0.,
            0., 0., f / (n - f),  n * f / (n - f),
            0., 0., -1., 0.
        )
    }

    fn update_look_at(&mut self) {
        self.look_at_matrix = Camera::compute_look_at(&self.position, &(self.position + self.front), &self.world_up)
    }

    fn move_on(&mut self, on: Vector3<f64>) {
        let direction = -self.front;
        let right = self.world_up.cross(&direction).normalize();
        let up = direction.cross(&right);

        let camera_rud_matrix = Matrix4::from_columns(&[
            right.to_homogeneous(),
            up.to_homogeneous(),
            direction.to_homogeneous(),
            Point3::<f64>::origin().to_homogeneous()
        ]);

        let transformed_on_homogenized = camera_rud_matrix * on.to_homogeneous();
        let transformed_on = Vector3::<f64>::from_homogeneous(transformed_on_homogenized).unwrap();
        self.position = self.position + transformed_on;
        self.update_look_at();
    }

    fn rotate_on(&mut self, pitch: f64, yaw: f64, scroll: f64) {

        let direction = -self.front;
        let right = self.world_up.cross(&direction).normalize();
        let up = direction.cross(&right);

        let camera_rud_matrix = Matrix4::from_columns(&[
            right.to_homogeneous(),
            up.to_homogeneous(),
            direction.to_homogeneous(),
            Point3::<f64>::origin().to_homogeneous()
        ]);

        let desired_front = Vector3::new(
            yaw.sin(),
            -pitch.sin(),
            yaw.cos() * -pitch.cos()
        ).normalize();

        let new_front_homogenized = camera_rud_matrix * desired_front.to_homogeneous();
        self.front = Vector3::<f64>::from_homogeneous(new_front_homogenized).unwrap();
        self.update_look_at();
    }

    pub fn set_param(&mut self, action_id: u32, action_value: f64) {
        match action_id {
            1 => self.speed[0] = action_value, // speed of camera on x axes
            2 => self.speed[1] = action_value, // speed of camera on y axes
            3 => self.speed[2] = -action_value, // speed of camera on z axes
            11 => self.rotation[0] = action_value,
            12 => self.rotation[1] = action_value,
            13 => self.rotation[2] = action_value,
            _ => unsafe { log(&"Not recognized action") }
        }
    }

    pub fn tick(&mut self) {
        self.rotate_on(self.rotation[0], self.rotation[1], self.rotation[2]);
        self.move_on(self.speed);
    }

    pub fn new(
        fov: f64, aspect: f64,
        z_near: f64, z_far: f64,
        &position: &Point3<f64>,
        &front: &Vector3<f64>,
        &world_up: &Vector3<f64>
    ) -> Camera {
        // let position = RowVector3::new(1., 1., 1.);
        // let front = RowVector3::new(-1., -1., -1.).normalize();
        // let up = RowVector3::new(0., 1., 0.);

        Camera {
            fov,
            aspect,
            position,
            world_up,
            front,
            look_at_matrix: Camera::compute_look_at(&position, &(position + front), &world_up),
            projection_matrix: Camera::compute_projection_matrix(fov, aspect, z_near, z_far),
            speed: Vector3::new(0., 0., 0.),
            rotation: Vector3::new(0., 0., 0.)
        }
    }
}
