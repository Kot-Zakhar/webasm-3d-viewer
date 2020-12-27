use crate::types::*;
use crate::texture::Texture;

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub vertices_normals: Vec<Vector4<f64>>,
    pub texture_vertices: Vec<Vertex>,
    pub vertices_viewvable: Vec<bool>,
    pub faces: Vec<Face>,
    pub model_color: Color<f64>,
    pub diffuse_texture: Texture<Pixel>,
    pub normal_texture_data: Texture<Pixel>,
    pub normal_texture_normals: Texture<Vector4<f64>>,
    pub specular_texture_data: Texture<Pixel>,
    pub specular_texture_coeff: Texture<Color<f64>>,
    
    pub use_diffuse_texture: bool,
    pub use_normal_texture: bool,
    pub use_specular_texture: bool,

    pub ambient: Color<f64>,
    pub diffuse_intensity: Color<f64>,
    pub specular: Color<f64>,
    pub specular_intensity: Color<f64>,
    pub shininess: f64,
    
    // world_position stuff
    pub rotation_matrix: Matrix4<f64>,
    pub scale_matrix: Matrix4<f64>,
    pub translation_matrix: Matrix4<f64>
}

impl Object {
    pub fn new() -> Object {
        Object {
            vertices: Vec::new(),
            vertices_normals: Vec::new(),
            vertices_viewvable: Vec::new(),
            texture_vertices: Vec::new(),
            faces: Vec::new(),
            model_color: Color{ r: 1., g: 1., b: 1. },
            diffuse_texture: Texture::new(),
            normal_texture_data: Texture::new(),
            normal_texture_normals: Texture::new(),
            specular_texture_data: Texture::new(),
            specular_texture_coeff: Texture::new(),
            
            use_diffuse_texture: false,
            use_normal_texture: false,
            use_specular_texture: false,
            
            ambient: Color{ r: 0.1, g: 0.1, b: 0.1 },
            diffuse_intensity: Color{ r: 0.8, g: 0.8, b: 0.8 },
            specular: Color{ r: 1., g: 1., b: 1. },
            specular_intensity: Color{ r: 0.1, g: 0.1, b: 0.1 },
            shininess: 1.,

            // emerald
            // ambient: Color{ r: 0.0215, g: 0.1745, b: 0.0215 },
            // diffuse: Color{ r: 0.07568, g: 0.61424, b: 0.07568 },
            // specular: Color{ r: 0.633, g: 0.727811, b: 0.633 },
            // shininess: 0.6,

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
        }
    }

    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertices.push(Vertex::new(x, y, z, 1.));
        self.vertices_viewvable.push(true);
    }

    pub fn add_vertex_normal(&mut self, x: f64, y: f64, z: f64) {
        self.vertices_normals.push(Vertex::new(x, y, z, 0.));
    }

    pub fn add_texture_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.texture_vertices.push(Vertex::new(x, y, z, 0.));
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

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.model_color.r = r as f64 / 255.;
        self.model_color.g = g as f64 / 255.;
        self.model_color.b = b as f64 / 255.;
    }

    pub fn set_ambient(&mut self, r: f64, g: f64, b: f64) {
        self.ambient.r = r;
        self.ambient.g = g;
        self.ambient.b = b;
    }

    pub fn set_diffuse(&mut self, r: f64, g: f64, b: f64) {
        self.diffuse_intensity.r = r;
        self.diffuse_intensity.g = g;
        self.diffuse_intensity.b = b;
    }

    pub fn set_specular(&mut self, r: f64, g: f64, b: f64) {
        self.specular.r = r;
        self.specular.g = g;
        self.specular.b = b;
    }


    pub fn set_texture_size(&mut self, texture_index: usize, width: usize, height: usize) {
        match texture_index {
            1 => self.diffuse_texture.set_size(width, height, Pixel{ color: white_color, a: 0}),
            2 => {
                self.normal_texture_data.set_size(width, height, Pixel{ color: white_color, a: 0});
                self.normal_texture_normals.set_size(width, height, Vector4::new(1., 1., 1., 1.));
            },
            3 => {
                self.specular_texture_data.set_size(width, height, Pixel{ color: white_color, a: 0});
                self.specular_texture_coeff.set_size(width, height, Color{ r: 1., g: 1., b: 1.});
            },
            _ => {}
        }
    }

    pub fn get_texture_pixels(&mut self, texture_index: usize) -> *const Pixel {
        match texture_index {
            1 => self.diffuse_texture.get_data_pointer(),
            2 => self.normal_texture_data.get_data_pointer(),
            3 => self.specular_texture_data.get_data_pointer(),
            _ => std::ptr::null()
        }
    }

    pub fn normalize_normal_texture(&mut self) {
        for (normal_index, normal_color) in self.normal_texture_data.data.iter_mut().enumerate() {
            self.normal_texture_normals.data[normal_index] = Vector4::new(
                normal_color.color.r as f64 / 255. * 2. - 1.,
                normal_color.color.g as f64 / 255. * 2. - 1.,
                normal_color.color.b as f64 / 255. * 2. - 1.,
                0.
            )
        }
    }

    pub fn normalize_specular_texture(&mut self) {
        for (specular_index, specular_color) in self.specular_texture_data.data.iter_mut().enumerate() {
            self.specular_texture_coeff.data[specular_index] = Color {
                r: specular_color.color.r as f64 / 255.,
                g: specular_color.color.g as f64 / 255.,
                b: specular_color.color.b as f64 / 255.
            }
        }
    }
}