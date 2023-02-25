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

use crate::square_matrix::SquareMatrix;
use crate::directions::Directions;
use crate::utils::is_prime;

pub struct UlamSpiral {
    matrix: SquareMatrix
}

impl UlamSpiral {
    pub fn new(size: usize) -> Self {
        assert!(size > 0 && size % 2 != 0, "Matrix size must be odd and bigger than 0!");

        let mut matrix = SquareMatrix::new(size);

        let mut dir = Directions::Right;
        let mut step = 1;
        let mut xy_cursor = (size / 2, size / 2);
        matrix[(xy_cursor.0, xy_cursor.1)] = 1;

        let mut i = 2;
        'outer: loop {
            for _ in 0..2 { // Step is incremented every two directions
                for _ in 0..step { // Number of steps before turning direction
                    if i > (size * size) {
                        break 'outer;
                    }

                    match dir {
                        Directions::Right => xy_cursor.0 += 1,
                        Directions::Up => xy_cursor.1 -= 1,
                        Directions::Left => xy_cursor.0 -= 1,
                        Directions::Down => xy_cursor.1 += 1,
                    }

                    matrix[(xy_cursor.0, xy_cursor.1)] = i.try_into().unwrap();
                    i = i + 1;
                }
                dir = dir.rotate_counter_clockwise();
            }
            step = step + 1;
        }

        Self {
            matrix
        }
    }

    pub fn elems(&self) -> &Vec<u32> {
        return &self.matrix.elems();
    }

    pub fn width(&self) -> usize {
        return self.matrix.width();
    }

    pub fn print(&self, as_numbers: bool) {
        let width = self.width();
    
        for (i, &elem) in self.elems().into_iter().enumerate() {
            print!("{}{}",
                   if as_numbers {elem} else {if is_prime(elem) { 1 } else { 0 }},
                   if (i + 1) % width == 0 {"\n"} else {""});
        }
    }

    pub fn save_as_image(&self, path: &str) {
        let width = self.width();
        let mut img: image::GrayImage = ImageBuffer::new(width.try_into().unwrap(), width.try_into().unwrap());
    
        for (i, &elem) in self.elems().into_iter().enumerate() {
            let x = i  % width;
            let y = i / width;
            img.put_pixel(x.try_into().unwrap(),
                          y.try_into().unwrap(),
                          if is_prime(elem) { Luma([0]) } else { Luma([255]) });
        }
    
        img.save(path).unwrap();
    }
}
