//
// Copyright 2023 Carlos Guimaraes
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use image::{ImageBuffer, Luma};
use imageproc::drawing::draw_filled_circle_mut;

use crate::directions::Directions;
use crate::square_matrix::SquareMatrix;
use crate::utils::{is_prime, rotate_point};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum UlamSpiralFormat {
    Square,
    Circle,
}

pub struct UlamSpiral {
    matrix: SquareMatrix,
    format: UlamSpiralFormat,
}

impl UlamSpiral {
    pub fn new(last: usize, format: UlamSpiralFormat) -> Self {
        assert!(
            last > 0 && last % 2 != 0,
            "Matrix size must be odd and bigger than 0!"
        );

        let mut matrix = SquareMatrix::new(last);

        let mut dir = Directions::Right;
        let mut step = 1;
        let mut xy_cursor = (last / 2, last / 2);
        matrix[xy_cursor] = 1;

        let mut i = 2;
        'outer: loop {
            for _ in 0..2 {
                // Step is incremented every two directions
                for _ in 0..step {
                    // Number of steps before turning direction
                    if i > (last * last) {
                        break 'outer;
                    }

                    match dir {
                        Directions::Right => xy_cursor.0 += 1,
                        Directions::Up => xy_cursor.1 -= 1,
                        Directions::Left => xy_cursor.0 -= 1,
                        Directions::Down => xy_cursor.1 += 1,
                    }

                    matrix[xy_cursor] = i;
                    i += 1;
                }
                dir = dir.rotate_counter_clockwise();
            }
            step += 1;
        }

        Self { matrix, format }
    }

    pub fn elems(&self) -> &Vec<usize> {
        self.matrix.elems()
    }

    pub fn width(&self) -> usize {
        self.matrix.width()
    }

    pub fn print(&self, as_numbers: bool) {
        let width = self.width();

        for (i, &elem) in self.elems().iter().enumerate() {
            print!(
                "{}{}",
                if as_numbers {
                    elem
                } else if is_prime(elem) {
                    1
                } else {
                    0
                },
                if (i + 1) % width == 0 { "\n" } else { "" }
            );
        }
    }

    pub fn save_as_image(&self, path: &str, width: usize, scale: usize, dot: usize) {
        let mut img: image::GrayImage = ImageBuffer::new(width as u32, width as u32);

        for (i, &elem) in self.elems().iter().enumerate() {
            let (x, y);
            match self.format {
                UlamSpiralFormat::Square => {
                    x = (((width * scale / 2) as isize - (self.width() / 2) as isize)
                        + (i % self.width()) as isize)
                        / scale as isize;
                    y = (((width * scale / 2) as isize - (self.width() / 2) as isize)
                        + (i / self.width()) as isize)
                        / scale as isize;
                }

                UlamSpiralFormat::Circle => {
                    let (xr, yr) = rotate_point((elem, 0), elem as f32);
                    x = (((width * scale / 2) as f32 + xr) / scale as f32) as isize;
                    y = (((width * scale / 2) as f32 + yr) / scale as f32) as isize;
                }
            }

            if x < width as isize && y < width as isize && x >= 0 && y >= 0 && is_prime(elem) {
                if dot == 1 {
                    img.put_pixel(x as u32, y as u32, Luma([255]));
                } else {
                    draw_filled_circle_mut::<image::GrayImage>(
                        &mut img,
                        (x as i32, y as i32),
                        dot as i32,
                        Luma([255]),
                    )
                }
            }
        }

        img.save(path).unwrap();
    }
}
