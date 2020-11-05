use std::mem;
use crate::types::*;
use crate::object::*;
use crate::console::log;

pub fn get_index(y: u32, x: u32, width: u32) -> usize {
    (y * width + x) as usize
}

pub fn draw_line<T>(pixels: &mut Vec<T>, mut width: u32, mut height: u32, mut x0: i32, mut y0: i32, mut x1:i32, mut y1: i32, &color: &T) where T: Copy {
    
    let steep = i32::abs(x0-x1) < i32::abs(y0-y1);
    if steep {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        mem::swap(&mut width, &mut height);
    }

    if x0>x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let direction = if y1 > y0 { 1 } else { -1 };
    let derror2 = i32::abs(dy) * 2;
    let mut error2 = 0;

    let mut y = y0;
    let mut x = x0;
    while x <= x1 {
        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
            let index;
            if steep {
                index = get_index(x as u32, y as u32, height);
            } else {
                index = get_index(y as u32, x as u32, width);
            }
            pixels[index] = color;
        }
        
        error2 += derror2;

        if error2 > dx {
            y += direction;
            error2 -= dx * 2;
        }
        x += 1;
    }
}


pub fn draw_sides_of_face(pixels: &mut Vec<Pixel>, z_buf: &mut Vec<f64>, width: u32, height: u32,
    vertices: &Vec<Vector4<f64>>, face: &Face,
    is_partial: bool, lineColor: &Pixel
) {
    if is_partial { return }
    let v1 = vertices[face.vertices_indexes[0] as usize];
    let v2 = vertices[face.vertices_indexes[1] as usize];
    let v3 = vertices[face.vertices_indexes[2] as usize];
    draw_line(
        pixels, width, height,
        v1[0].round() as i32, v1[1].round() as i32, v2[0].round() as i32, v2[1].round() as i32,
        &lineColor
    );
    draw_line(
        pixels, width, height,
        v1[0].round() as i32, v1[1].round() as i32, v3[0].round() as i32, v3[1].round() as i32,
        &lineColor
    );
    draw_line(
        pixels, width, height,
        v2[0].round() as i32, v2[1].round() as i32, v3[0].round() as i32, v3[1].round() as i32,
        &lineColor
    );
}

pub fn draw_face_on_buffer(width: i32, height: i32,
    z_buf: &mut [f64],
    face_index_buffer: &mut [i32], face_index: usize,
    object_index_buffer: &mut [i32], object_index: usize,
    vertices: &Vec<Vec<Vector4<f64>>>,
    face: &Face,
    is_partial: bool
) {

    let mut v1 = &vertices[object_index][face.vertices_indexes[0] as usize];
    let mut v2 = &vertices[object_index][face.vertices_indexes[1] as usize];
    let mut v3 = &vertices[object_index][face.vertices_indexes[2] as usize];

    // sort on y
    if v2[1] < v1[1] {
        mem::swap(&mut v2,&mut v1);
    }
    if v3[1] < v1[1] {
        mem::swap(&mut v3,&mut v1);
    }
    if v3[1] < v2[1] {
        mem::swap(&mut v3,&mut v2);
    }

    let x1 = v1[0].round();
    let y1 = v1[1].round();
    let z1 = v1[2];

    let x2 = v2[0].round();
    let y2 = v2[1].round();
    let z2 = v2[2];

    let x3 = v3[0].round();
    let y3 = v3[1].round();
    let z3 = v3[2];

    // let z = (v1[2] + v2[2] + v3[2]) / 3.;

    // dx
    let mut dx13 = if y1 != y3 { (x3 - x1) / (y3 - y1) } else { 0. };
    let mut _dx13 = dx13;
    let mut dx12 = if y1 != y2 { (x2 - x1) / (y2 - y1) } else { 0. };
    let mut dx23 = if y2 != y3 { (x3 - x2) / (y3 - y2) } else { 0. };

    // dz
    let mut dz13 = if y1 != y3 { (z3 - z1) / (y3 - y1) } else { 0. };
    let mut _dz13 = dz13;
    let mut dz12 = if y1 != y2 { (z2 - z1) / (y2 - y1) } else { 0. };
    let mut dz23 = if y2 != y3 { (z3 - z2) / (y3 - y2) } else { 0. };

    let mut wx1 = x1;
    let mut wx2 = x1;

    let mut wz1 = z1;
    let mut wz2 = z1;

    if dx13 > dx12 {
        mem::swap(&mut dx13, &mut dx12);
        mem::swap(&mut dz13, &mut dz12);
    }
    
    let y1 = y1 as i32;
    let y2 = y2 as i32;
    let y3 = y3 as i32;

    for i in y1..y2 {
        if !is_partial || !(i < 0 || i >= height) {
            let dz_k = (wz2 - wz1) / (wx2.round() - wx1.round() + 1.);
            let mut z_j = wz1;
            for j in wx1.round() as i32 ..= wx2.round() as i32 {
                if !is_partial || !(j < 0 || j >= width) {
                    let index = get_index(i as u32, j as u32, width as u32);
                    if z_j < z_buf[index] {
                        z_buf[index] = z_j;
                        face_index_buffer[index] = face_index as i32;
                        object_index_buffer[index] = object_index as i32;
                    }
                }
                z_j += dz_k;
            }
        }

        wx1 += dx13;
        wx2 += dx12;

        wz1 += dz13;
        wz2 += dz12;
    }

    if y1 == y2 {
        if x1 < x2 {
            wx1 = x1;
            wx2 = x2;
            wz1 = z1;
            wz2 = z2;
        } else {
            wx1 = x2;
            wx2 = x1;
            wz1 = z2;
            wz2 = z1;
        }
    }

    if _dx13 < dx23 {
        mem::swap(&mut _dx13, &mut dx23);
        mem::swap(&mut _dz13, &mut dz23);
    }
    
    for i in y2..y3 {
        if !is_partial || !(i < 0 || i >= height) {
            let dz_k = (wz2 - wz1) / (wx2.round() - wx1.round() + 1.);
            let mut z_j = wz1;
            for j in wx1.round() as i32 ..= wx2.round() as i32 {
                if !is_partial || !(j < 0 || j >= width) {
                    let index = get_index(i as u32, j as u32, width as u32);
                    if z_j < z_buf[index] {
                        z_buf[index] = z_j;
                        face_index_buffer[index] = face_index as i32;
                        object_index_buffer[index] = object_index as i32;
                    }
                }
                z_j += dz_k;
            }
        }

        wx1 += _dx13;
        wx2 += dx23;
        wz1 += _dz13;
        wz2 += dz23;
    }
}

pub fn find_color_in_point(
    point: &Vertex,
    v1: &Vertex, v2: &Vertex, v3: &Vertex,
    vn1: &Vector4<f64>, vn2: &Vector4<f64>, vn3: &Vector4<f64>,
    direct_light_direction: &Vector4<f64>,
    color: &Pixel
) -> Pixel {
    let v1 = Point3::from_homogeneous(*v1).unwrap();
    let v2 = Point3::from_homogeneous(*v2).unwrap();
    let v3 = Point3::from_homogeneous(*v3).unwrap();
    let point = Point3::from_homogeneous(*point).unwrap();
    let v12 = v2 - v1;
    let v23 = v3 - v2;
    let v31 = v1 - v3;
    let v13 = v3 - v1;
    let face_normal = v12.cross(&v13);
    let denom = face_normal.dot(&face_normal);
    let v1p = point - v1;
    let v2p = point - v2;
    let v3p = point - v3;
    // let n = face_normal.magnitude();
    let w = v12.cross(&v1p).dot(&face_normal) / denom;
    let u = v23.cross(&v2p).dot(&face_normal) / denom;
    let v = v31.cross(&v3p).dot(&face_normal) / denom;
    // let u = v13.cross(&v1p).magnitude() / n;
    // let v = v12.cross(&v1p).magnitude() / n;
    // let w = v23.cross(&v1p).magnitude() / n;
    // unsafe {log(&format!("{}", u + v + w))}
    // let vn12 = vn2 - vn1;
    // let vn23 = vn3 - vn2;
    // let vn31 = vn1 - vn3;
    let normal = vn1 * u + vn2 * v + vn3 * w;
    let cos = normal.dot(&(-direct_light_direction));

    Pixel{
        r: (color.r as f64 * cos) as u8,
        g: (color.g as f64 * cos) as u8,
        b: (color.b as f64 * cos) as u8,
        a: color.a
    }
}

pub fn draw_face_phong(pixels: &mut Vec<Pixel>, z_buf: &mut Vec<f64>, width: i32, height: i32,
    vertices: &Vec<Vector4<f64>>,
    vertices_normals_obj_space: &Vec<Vector4<f64>>,
    face: &Face,
    direct_light_direction_obj_space: &Vector4<f64>,
    color: &Pixel,
    is_partial: bool
) {

    let mut v1 = &vertices[face.vertices_indexes[0] as usize];
    let mut v2 = &vertices[face.vertices_indexes[1] as usize];
    let mut v3 = &vertices[face.vertices_indexes[2] as usize];
    let mut vn1 = &vertices_normals_obj_space[face.vertices_normals_indexes[0] as usize];
    let mut vn2 = &vertices_normals_obj_space[face.vertices_normals_indexes[1] as usize];
    let mut vn3 = &vertices_normals_obj_space[face.vertices_normals_indexes[2] as usize];

    // sort on y
    if v2[1] < v1[1] {
        mem::swap(&mut v2,&mut v1);
        mem::swap(&mut vn2,&mut vn1);
    }
    if v3[1] < v1[1] {
        mem::swap(&mut v3,&mut v1);
        mem::swap(&mut vn3,&mut vn1);
    }
    if v3[1] < v2[1] {
        mem::swap(&mut v3,&mut v2);
        mem::swap(&mut vn3,&mut vn2);
    }

    let x1 = v1[0].round();
    let y1 = v1[1].round();
    let z1 = v1[2];

    let x2 = v2[0].round();
    let y2 = v2[1].round();
    let z2 = v2[2];

    let x3 = v3[0].round();
    let y3 = v3[1].round();
    let z3 = v3[2];

    // let z = (v1[2] + v2[2] + v3[2]) / 3.;

    // dx
    let mut dx13 = if y1 != y3 { (x3 - x1) / (y3 - y1) } else { 0. };
    let mut _dx13 = dx13;
    let mut dx12 = if y1 != y2 { (x2 - x1) / (y2 - y1) } else { 0. };
    let mut dx23 = if y2 != y3 { (x3 - x2) / (y3 - y2) } else { 0. };

    // dz
    let mut dz13 = if y1 != y3 { (z3 - z1) / (y3 - y1) } else { 0. };
    let mut _dz13 = dz13;
    let mut dz12 = if y1 != y2 { (z2 - z1) / (y2 - y1) } else { 0. };
    let mut dz23 = if y2 != y3 { (z3 - z2) / (y3 - y2) } else { 0. };

    let mut wx1 = x1;
    let mut wx2 = x1;

    let mut wz1 = z1;
    let mut wz2 = z1;

    if dx13 > dx12 {
        mem::swap(&mut dx13, &mut dx12);
        mem::swap(&mut dz13, &mut dz12);
    }
    
    let y1 = y1 as i32;
    let y2 = y2 as i32;
    let y3 = y3 as i32;

    for i in y1..y2 {
        if !is_partial || !(i < 0 || i >= height) {
            let dz_k = (wz2 - wz1) / (wx2.round() - wx1.round() + 1.);
            let mut z_j = wz1;
            for j in wx1.round() as i32 ..= wx2.round() as i32 {
                if !is_partial || !(j < 0 || j >= width) {
                    let index = get_index(i as u32, j as u32, width as u32);
                    if z_buf[index] == 0. || z_j < z_buf[index] {
                        z_buf[index] = z_j;
                        let point = Vector4::new(j as f64, i as f64, z_j, 1.);
                        pixels[index] = find_color_in_point(&point, v1, v2, v3, vn1, vn2, vn3, direct_light_direction_obj_space, color);
                    }
                }
                z_j += dz_k;
            }
        }

        wx1 += dx13;
        wx2 += dx12;

        wz1 += dz13;
        wz2 += dz12;
    }

    if y1 == y2 {
        if x1 < x2 {
            wx1 = x1;
            wx2 = x2;
            wz1 = z1;
            wz2 = z2;
        } else {
            wx1 = x2;
            wx2 = x1;
            wz1 = z2;
            wz2 = z1;
        }
    }

    if _dx13 < dx23 {
        mem::swap(&mut _dx13, &mut dx23);
        mem::swap(&mut _dz13, &mut dz23);
    }
    
    for i in y2..y3 { // must be y2..=y3 but gives out of bounds exeption..
        if !is_partial || !(i < 0 || i >= height) {
            let dz_k = (wz2 - wz1) / (wx2.round() - wx1.round() + 1.);
            let mut z_j = wz1;
            for j in wx1.round() as i32 ..= wx2.round() as i32 {
                if !is_partial || !(j < 0 || j >= width) {
                    let index = get_index(i as u32, j as u32, width as u32);
                    if z_buf[index] == 0. || z_j < z_buf[index] {
                        z_buf[index] = z_j;
                        let point = Vector4::new(j as f64, i as f64, z_j, 1.);
                        pixels[index] = find_color_in_point(&point, v1, v2, v3, vn1, vn2, vn3, direct_light_direction_obj_space, color);
                    }
                }
                z_j += dz_k;
            }
        }

        wx1 += _dx13;
        wx2 += dx23;
        wz1 += _dz13;
        wz2 += dz23;
    }
}
