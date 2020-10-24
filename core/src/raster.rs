use std::mem;
use crate::types::*;

pub fn get_index(row: u32, column: u32, width: u32) -> usize {
    (row * width + column) as usize
}

pub fn draw_line<T>(pixels: &mut Vec<T>, width: u32, height: u32, mut x0: i32, mut y0: i32, mut x1:i32, mut y1: i32, &color: &T) where T: Copy {
    
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
    let direction = if y1 > y0 { 1 } else { -1 };
    let derror2 = i32::abs(dy) * 2;
    let mut error2 = 0;

    let mut y = y0;
    let mut x = x0;
    while x <= x1 {
        let index;
        if steep {
            index = get_index(x as u32, y as u32, width);
        } else {
            index = get_index(y as u32, x as u32, width);
        }
        pixels[index] = color;
        
        error2 += derror2;

        if error2 > dx {
            y += direction;
            error2 -= dx * 2;
        }
        x += 1;
    }
}

pub fn draw_face(pixels: &mut Vec<Pixel>, z_buf: &mut Vec<f64>, width: i32, height: i32,
    v1: &Vertex, v2: &Vertex, v3: &Vertex, is_partial: bool, &color: &Pixel, &lineColor: &Pixel
) {

    let mut v1 = v1;
    let mut v2 = v2;
    let mut v3 = v3;

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

    let x2 = v2[0].round();
    let y2 = v2[1].round();

    let x3 = v3[0].round();
    let y3 = v3[1].round();

    let z = (v1[2] + v2[2] + v3[2]) / 3.;
    
    let mut dx13 = if y1 != y3 { (x3 - x1) / (y3 - y1) } else { 0. };
    let mut _dx13 = dx13;
    let mut dx12 = if y1 != y2 { (x2 - x1) / (y2 - y1) } else { 0. };
    let mut dx23 = if y2 != y3 { (x3 - x2) / (y3 - y2) } else { 0. };

    let mut wx1 = x1;
    let mut wx2 = x1;

    if dx13 > dx12 {
        mem::swap(&mut dx13, &mut dx12);
    }
    
    let y1 = y1 as i32;
    let x1 = x1 as i32;
    let y2 = y2 as i32;
    let x2 = x2 as i32;
    let y3 = y3 as i32;
    let x3 = x3 as i32;

    for i in y1..y2 {
        if !is_partial || !(i < 0 || i >= height) {
            for j in wx1.floor() as i32 .. wx2.ceil() as i32 {
                if is_partial && (j < 0 || j >= width) {
                    continue;
                }
                let index = get_index(i as u32, j as u32, width as u32);
                if z_buf[index] == 0. || z < z_buf[index] {
                    z_buf[index] = z;
                    pixels[index] = color;
                }
            }
        }

        // let border1_index = get_index(i as u32, wx1.floor() as u32, width);
        // if z_buf[border1_index] == 0. || z < z_buf[border1_index] {
        //     z_buf[border1_index] = z - 0.0001;
        //     pixels[border1_index] = lineColor;
        // }

        // let border2_index = get_index(i as u32, wx2.floor() as u32, width);
        // if z_buf[border2_index] == 0. || z < z_buf[border2_index] {
        //     z_buf[border2_index] = z - 0.0001;
        //     pixels[border2_index] = lineColor;
        // }
        
        wx1 += dx13;
        wx2 += dx12;
    }

    if y1 == y2 {
        if x1 < x2 {
            wx1 = x1 as f64;
            wx2 = x2 as f64;
        } else {
            wx1 = x2 as f64;
            wx2 = x1 as f64;
        }
    }

    if _dx13 < dx23 {
        mem::swap(&mut _dx13, &mut dx23);
    }
    
    for i in y2..y3 {
        if !is_partial || !(i < 0 || i >= height) {
            for j in wx1.floor() as i32 .. wx2.ceil() as i32 {
                if is_partial && (j < 0 || j >= width) {
                    continue;
                }
                let index = get_index(i as u32, j as u32, width as u32);
                if z_buf[index] == 0. || z < z_buf[index] {
                    z_buf[index] = z;
                    pixels[index] = color;
                }
            }
        }
        // let border1_index = get_index(i as u32, wx1.floor() as u32, width);
        // if z_buf[border1_index] == 0. || z < z_buf[border1_index] {
        //     z_buf[border1_index] = z - 0.0001;
        //     pixels[border1_index] = lineColor;
        // }

        // let border2_index = get_index(i as u32, wx2.floor() as u32, width);
        // if z_buf[border2_index] == 0. || z < z_buf[border2_index] {
        //     z_buf[border2_index] = z - 0.0001;
        //     pixels[border2_index] = lineColor;
        // }

        wx1 += _dx13;
        wx2 += dx23;
    }

    // draw_line(pixels, width, height, x1, y1, x2, y2, &lineColor);
    // draw_line(pixels, width, height, x1, y1, x3, y3, &lineColor);
    // draw_line(pixels, width, height, x2, y2, x3, y3, &lineColor);
}

pub fn check_vertex_in_view_box(&v: &Vertex, &cube_min: &Vertex, &cube_max: &Vertex) -> bool {
    v[0] > cube_min[0] && v[0] < cube_max[0] &&
    v[1] > cube_min[1] && v[1] < cube_max[1] &&
    v[2] > cube_min[2] && v[2] < cube_max[2]
}