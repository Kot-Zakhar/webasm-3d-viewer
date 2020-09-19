mod utils;

extern crate nalgebra as na;
use na::*;
use std::mem;
use wasm_bindgen::prelude::*;

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub type Vertex = Vector4<f64>;
pub type TransformationMatrix = Matrix4<f64>;

struct Object {
    vertexes: Vec<Vertex>,
    faces: Vec<Matrix3<u32>>,
    
    // world_position stuff
    rotation_matrix: Matrix4<f64>,
    scale_matrix: Matrix4<f64>,
    translation_matrix: Matrix4<f64>
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Camera {
    position: RowVector3<f64>,
    target: RowVector3<f64>,
    world_up: RowVector3<f64>,
    // front: RowVector3<f64>,
    look_at_matrix: Matrix4<f64>,
    projection_matrix: Matrix4<f64>
}// TODO: try to make camera using FOV and aspect (угол обзора камеры)

#[wasm_bindgen]
impl Camera {
    fn compute_look_at(position: RowVector3<f64>, target: RowVector3<f64>, up: RowVector3<f64>) -> Matrix4<f64>{

        let direction = (position - target).normalize();
        let right = up.cross(&direction).normalize();
        let up = direction.cross(&right).normalize();

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
    }

    pub fn new(fov: f64, aspect_ration: f64) -> Camera {
        let position = RowVector3::new(0., 0., 2.);
        let target = RowVector3::new(0., 0., 0.);
        let up = RowVector3::new(0., 0., 1.).normalize();
        // let front = RowVector3::new(0., 0., -1.).normalize();

        let z_near = 0.1;
        let z_far = 10.;
        Camera {
            position,
            target,
            world_up: up,
            // front,
            look_at_matrix: Camera::compute_look_at(position, target, up),
            // look_at_matrix: Camera::compute_look_at(position, position + front, up),
            projection_matrix: Camera::compute_projection_matrix(fov, aspect_ration, z_near, z_far),
        }
    }

    fn update_look_at(&mut self) {
        self.look_at_matrix = Camera::compute_look_at(self.position, self.target, self.world_up)
        // self.look_at_matrix = Camera::compute_look_at(self.position, self.position + self.front, self.world_up)
    }

    pub fn move_on(&mut self, d_toward: f64, d_right: f64, d_up: f64) {
        let front = (self.position - self.target).normalize();
        self.position += front * d_toward;
        self.position += front.cross(&self.world_up).normalize() * d_right;
        self.position += front.cross(&self.world_up).normalize().cross(&front).normalize() * d_up;

        self.update_look_at();
        console_log!("x: {}, y: {}, z: {}", self.position[0], self.position[1], self.position[2]);
    }

    pub fn fly_around_center(&mut self, xz_angle: f64) {
        let radius = (self.position[2].powi(2) + self.position[0].powi(2)).sqrt() ;
        let cam_x = xz_angle.sin() * radius;
        let cam_z = xz_angle.cos() * radius;
        self.position = RowVector3::new(cam_x, self.position[1], cam_z);
        self.update_look_at();
    }

    pub fn rotate(&mut self, yaw: f64, pitch: f64, scroll: f64) {
        // yaw - rotate around up vector (y)
        // pitch - rotate around right vector (x)
        // scroll - rotate around neg front vector (z)
        
    }

    fn compute_projection_matrix(fov: f64, aspect_ration: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {

        // ortographic projection
        let width = 10.;
        let height = width / aspect_ration;
        Matrix4::new(
            2. / width, 0., 0., 0.,
            0., 2. / height, 0., 0.,
            0., 0., 1. / (z_near - z_far), z_near / (z_near - z_far),
            0., 0., 0., 1.
        )
        // // perspective projection
        // Matrix4::new(
        //     2. * z_near / width, 0., 0., 0.,
        //     0., 2. * z_near / height, 0., 0.,
        //     0., 0., z_far / (z_near - z_far),  z_near * z_far / (z_near - z_far),
        //     0., 0., -1., 0.
        // )
        // Matrix4::new(
        //     1. / (aspect_ration * (fov/2.).tan()), 0., 0., 0.,
        //     0., 1. / (fov / 2.).tan(), 0., 0.,
        //     0., 0., z_far / (z_near - z_far),  z_near * z_far / (z_near - z_far),
        //     0., 0., -1., 0.
        // )
    }
}

#[wasm_bindgen]
pub struct World {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    objects: Vec<Object>,
    camera: Camera,
    to_screen_matrix: Matrix4<f64>
}

fn _diagonal(val: f64) -> Matrix4<f64> {
    Matrix4::new(
        val, 0., 0., 0.,
        0., val, 0., 0.,
        0., 0., val, 0.,
        0., 0., 0., val
    )
}

fn _one() -> Matrix4<f64> {
    _diagonal(1.)
}

fn _zero() -> Matrix4<f64> {
    _diagonal(0.)
}

impl World {
    fn get_index(row: u32, column: u32, width: u32) -> usize {
        (row * width + column) as usize
    }

    fn draw_line(pixels: &mut Vec<Pixel>, width: u32, height: u32, mut x0: i32, mut y0: i32, mut x1:i32, mut y1: i32) {
        
        let steep = i32::abs(x0-x1) < i32::abs(y0-y1);
        if steep {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
        }

        if x0>x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = i32::abs(dy) * 2;
        let mut error2 = 0;

        let mut y = y0;
        let mut x = x0;
        while x <= x1 {
            let index;
            if steep {
                index = World::get_index(x as u32, y as u32, width);
            } else {
                index = World::get_index(y as u32, x as u32, width);
            }
            pixels[index].r = 0;
            pixels[index].g = 0;
            pixels[index].b = 0;
            
            error2 += derror2;

            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
            x += 1;
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(width: u32, height: u32) -> World {
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

        let width_2 = width as f64 / 2.;
        let height_2 = height as f64 / 2.;
        let to_screen_matrix = Matrix4::new(
            width_2, 0., 0., width_2,
            0., -height_2, 0., height_2,
            0., 0., 1., 0.,
            0., 0., 0., 1.,
        );
                
        World {
            width,
            height,
            pixels,
            objects: Vec::new(),
            camera: Camera::new(std::f64::consts::PI / 4., width as f64 / height as f64),
            to_screen_matrix
        }
    }

    pub fn new_object(&mut self) -> u32 {

        let obj = Object {
            vertexes: Vec::new(),
            faces: Vec::new(),

            rotation_matrix: _one(),
            scale_matrix: _one(),
            translation_matrix: _one()
        };

        self.objects.push(obj);
        (self.objects.len() - 1) as u32
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }

    pub fn add_vertex(&mut self, object_handle: u32, x:f64, y:f64, z:f64, w:f64) {
        self.objects[object_handle as usize].vertexes.push(Vertex::new(x, y, z, w));
    }

    pub fn add_face(&mut self, object_handle: u32, v0: u32, vt0: u32, vn0: u32, v1: u32, vt1: u32, vn1: u32, v2: u32, vt2: u32, vn2: u32) {
        self.objects[object_handle as usize].faces.push(Matrix3::new(
            v0, v1, v2,
            vt0, vt1, vt2,
            vn0, vn1, vn2
        ));
    }

    pub fn set_object_rotation(&mut self, object_handle: u32, angle_x: f64, angle_y: f64, angle_z: f64) {
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
            -sinx, 0., cosy, 0.,
            0.,    0., 0.,   1.
        );

        let rotation_z_matrix = Matrix4::new(
            cosz, -sinz, 0., 0.,
            sinz, cosz,  0., 0.,
            0.,   0.,    1., 0.,
            0.,   0.,    0., 1.
        );

        self.objects[object_handle as usize].rotation_matrix = rotation_x_matrix * rotation_y_matrix * rotation_z_matrix;
    }

    pub fn set_object_scale(&mut self, object_handle: u32, scale: f64) {
        self.objects[object_handle as usize].scale_matrix = Matrix4::new(
            scale, 0.,   0.,   0.,
            0.,   scale, 0.,   0.,
            0.,   0.,   scale, 0.,
            0.,   0.,   0.,   1.
        );
    }

    pub fn set_object_translaiton(&mut self, object_handle: u32, x: f64, y: f64, z:f64) {
        self.objects[object_handle as usize].translation_matrix = Matrix4::new(
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.
        )
    }

    pub fn move_camera_on(&mut self, d_toward: f64, d_right: f64, d_up: f64) {
        self.camera.move_on(d_toward, d_right, d_up);
    }

    fn clear_world(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = 255;
            pixel.g = 255;
            pixel.b = 255;
        }
    }

    pub fn rotate(&mut self, xz_angle: f64) {
        self.camera.fly_around_center(xz_angle)
    }

    pub fn compute(&mut self) -> u32 {
        utils::set_panic_hook();
        self.clear_world();
    
        // let current_obj = &self.objects[0];
        // let to_world = current_obj.translation_matrix * current_obj.rotation_matrix * current_obj.scale_matrix;

        let look_at = self.camera.look_at_matrix;

        let projection = self.camera.projection_matrix;

        let to_screen = self.to_screen_matrix;

        // let final_matrix = to_screen * projection * look_at * to_world;
        let object_independent_matrix = to_screen * projection * look_at;

        let mut not_printed = 0;

        for obj in self.objects.iter() {
            let to_world = obj.translation_matrix * obj.scale_matrix * obj.rotation_matrix;
            let final_matrix = object_independent_matrix * to_world;
            let view_vertexes: Vec<Vertex> = obj.vertexes.iter().map(|vertex| {
                final_matrix * vertex
            }).collect();

            for face in obj.faces.iter() {
                let v0 = view_vertexes[face.column(0)[0] as usize];
                let x0 = v0[0].round() as i32;
                let y0 = v0[1].round() as i32;
                let v1 = view_vertexes[face.column(1)[0] as usize];
                let x1 = v1[0].round() as i32;
                let y1 = v1[1].round() as i32;
                let v2 = view_vertexes[face.column(2)[0] as usize];
                let x2 = v2[0].round() as i32;
                let y2 = v2[1].round() as i32;
    
                if
                x0 >= self.width as i32 || y0 >= self.height as i32 || x0 < 0 || y0 < 0 ||
                x1 >= self.width as i32 || y1 >= self.height as i32 || x1 < 0 || y1 < 0 ||
                x2 >= self.width as i32 || y2 >= self.height as i32 || x2 < 0 || y2 < 0 
                {
                    not_printed += 1;
                    continue;
                }
    
                World::draw_line(&mut self.pixels, self.width, self.height, x0, y0, x1, y1);
                World::draw_line(&mut self.pixels, self.width, self.height, x0, y0, x2, y2);
                World::draw_line(&mut self.pixels, self.width, self.height, x1, y1, x2, y2);
            }
    
        }

        not_printed
    }
}