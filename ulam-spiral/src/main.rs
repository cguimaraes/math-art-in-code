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

use std::ops::{Index, IndexMut};
use image::{ImageBuffer, Luma};

struct SquareMatrix {
    elems: Vec<usize>
}

impl SquareMatrix {
    pub fn new(size: usize) -> Self {
        assert!(size > 0 && size % 2 != 0, "Matrix size must be odd and bigger than 0!");

        let mut elems = vec![0; size * size];

        let mut dir: Directions = Directions::RIGHT;
        let mut step: usize = 1;
        let mut xy_cursor: (usize, usize) = (size / 2, size / 2);
        elems[(xy_cursor.1 * size) + xy_cursor.0] = 1;

        let mut i = 2;
        'outer: loop {
            for _ in 0..2 { // Step is incremented every two directions
                for _ in 0..step { // Number of steps before turning direction
                    if i > (size * size) {
                        break 'outer;
                    }

                    match dir {
                        Directions::RIGHT => xy_cursor.0 += 1,
                        Directions::UP => xy_cursor.1 -= 1,
                        Directions::LEFT => xy_cursor.0 -= 1,
                        Directions::DOWN => xy_cursor.1 += 1,
                    }

                    elems[(xy_cursor.1 * size) + xy_cursor.0] = i;
                    i = i + 1;
                }
                dir = dir.rotate_counter_clockwise();
            }
            step = step + 1;
        }

        Self {
            elems
        }
    }

    pub fn len(&self) -> usize {
        return self.elems.len();
    }
}

impl Index<usize> for SquareMatrix {
    type Output = usize;

    fn index(&self, idx: usize) -> &usize {
        return &self.elems[idx];
    }
}

impl IndexMut<usize> for SquareMatrix {
    fn index_mut(&mut self, idx: usize) -> &mut usize {
        return &mut self.elems[idx];
    }
}

enum Directions {
    RIGHT = 0,
    UP = 1,
    LEFT = 2,
    DOWN = 3
}

impl Directions
{
    fn rotate_counter_clockwise(&self) -> Directions {
        match self {
            Directions::RIGHT => return Directions::UP,
            Directions::UP => return Directions::LEFT,
            Directions::LEFT => return Directions::DOWN,
            Directions::DOWN => return Directions::RIGHT
        };
    }
}

fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in 2..(n / 2) + 1 {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn print_matrix(m: &SquareMatrix, as_numbers: bool) {
    let size: u32 = (m.len() as f32).sqrt() as u32;

    for pos in 0..m.len() as u32 {
        print!("{}{}",
               if as_numbers {m[pos.try_into().unwrap()]} else {if is_prime(m[pos.try_into().unwrap()]) { 1 } else { 0 }},
               if (pos + 1) % size == 0 {"\n"} else {""});
    }
}

fn save_as_image(m: &SquareMatrix, path: &str) {
    let size: u32 = (m.len() as f32).sqrt() as u32;
    let mut img: image::GrayImage = ImageBuffer::new(size, size);

    for pos in 0..m.len() as u32 {
        img.put_pixel(pos % size,
                      pos / size,
                      if is_prime(m[pos.try_into().unwrap()]) { Luma([0]) } else { Luma([255]) });
    }

    img.save(path).unwrap();
}

fn main() {
    let size: usize = 4001;
    let verbose: bool = false;
    let output: &str = "./test.png";
    
    let matrix = SquareMatrix::new(size);
    if verbose { print_matrix(&matrix, true); }
    save_as_image(&matrix, output);
}
