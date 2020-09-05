mod utils;

use wasm_bindgen::prelude::*;
use rand::Rng;

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Pixel {
//     Black = 0,
//     White = 1,
// }

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[wasm_bindgen]
pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>
}

impl Image {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height - 1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width - 1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }
    //
    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.pixels[idx] as u8;
    //         }
    //     }
    //     count
    // }
}

#[wasm_bindgen]
impl Image {
    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0, self.height);
        let col = rng.gen_range(0, self.width);

        let idx = self.get_index(row, col);
        self.pixels[idx].r = self.pixels[idx].r ^ 255;
        self.pixels[idx].g = self.pixels[idx].g ^ 255;
        self.pixels[idx].b = self.pixels[idx].b ^ 255;

        // let mut next = self.pixels.clone();
        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         let idx = self.get_index(row, col);
        //         let pixel = self.pixels[idx];
        //         let live_neighbors = self.live_neighbor_count(row, col);
        //
        //         let next_pixel = match (pixel, live_neighbors) {
        //             // Rule 1: Any live pixel with fewer than two live neighbours
        //             // dies, as if caused by underpopulation.
        //             (Pixel::White, x) if x < 2 => Pixel::Black,
        //             // Rule 2: Any live pixel with two or three live neighbours
        //             // lives on to the next generation.
        //             (Pixel::White, 2) | (Pixel::White, 3) => Pixel::White,
        //             // Rule 3: Any live pixel with more than three live
        //             // neighbours dies, as if by overpopulation.
        //             (Pixel::White, x) if x > 3 => Pixel::Black,
        //             // Rule 4: Any dead pixel with exactly three live neighbours
        //             // becomes a live pixel, as if by reproduction.
        //             (Pixel::Black, 3) => Pixel::White,
        //             // All other pixels remain in the same state.
        //             (otherwise, _) => otherwise,
        //         };
        //
        //         self.pixels[idx] = next_pixel;
        //         // next[idx] = next_pixel;
        //     }
        // }
    }

    pub fn new() -> Image {
        let width = 512;
        let height = 512;

        // let pixels = (0..width * height)
        //     .map(|i| {
        //         if i % 2 == 0 || i % 7 == 0 {
        //             Pixel::White
        //         } else {
        //             Pixel::Black
        //         }
        //     })
        //     .collect();

        let pixels = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Pixel {
                        r: 255,
                        g: 255,
                        b: 255,
                        a: 255
                    }
                } else {
                    Pixel {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255
                    }
                }
            })
            .collect();

        Image {
            width,
            height,
            pixels
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_ptr()
    }
}