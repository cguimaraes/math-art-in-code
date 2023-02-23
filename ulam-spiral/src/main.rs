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
    elems: Vec<u32>
}

impl SquareMatrix {
    pub fn len(&self) -> u32 {
        return self.elems.len() as u32;
    }

    pub fn width(&self) -> u32 {
        return (self.len() as f32).sqrt() as u32;
    }
}

impl Index<(u32, u32)> for SquareMatrix {
    type Output = u32;

    fn index(&self, idx: (u32, u32)) -> &u32 {
        let width: u32 = self.width();
        return &self.elems[((idx.1 * width) + idx.0) as usize];
    }
}

impl IndexMut<(u32, u32)> for SquareMatrix {
    fn index_mut(&mut self, idx: (u32, u32)) -> &mut u32 {
        let width: u32 = self.width();
        return &mut self.elems[((idx.1 * width) + idx.0) as usize];
    }
}

struct UlamSpiral {
    matrix: SquareMatrix
}

impl UlamSpiral {
    pub fn new(size: u32) -> Self {
        assert!(size > 0 && size % 2 != 0, "Matrix size must be odd and bigger than 0!");

        let mut matrix = SquareMatrix {
            elems: vec![0; (size * size) as usize],
        };

        let mut dir: Directions = Directions::RIGHT;
        let mut step: u32 = 1;
        let mut xy_cursor: (u32, u32) = (size / 2, size / 2);
        matrix[(xy_cursor.0, xy_cursor.1)] = 1;

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

                    matrix[(xy_cursor.0, xy_cursor.1)] = i;
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
        return &self.matrix.elems;
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

fn is_prime(n: u32) -> bool {
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

fn print_matrix(us: &UlamSpiral, as_numbers: bool) {
    let width: u32 = us.matrix.width();

    let mut i : u32 = 1;
    for &elem in us.elems() {
        print!("{}{}",
               if as_numbers {elem} else {if is_prime(elem) { 1 } else { 0 }},
               if i % width == 0 {"\n"} else {""});
        i += 1;
    }
}

fn save_as_image(us: &UlamSpiral, path: &str) {
    let width: u32 = us.matrix.width();
    let mut img: image::GrayImage = ImageBuffer::new(width, width);

    for pos in 0..us.matrix.len() {
        let x : u32 = pos % width;
        let y : u32 = pos / width;
        img.put_pixel(x, y, if is_prime(us.matrix[(x, y)]) { Luma([0]) } else { Luma([255]) });
    }

    img.save(path).unwrap();
}

fn main() {
    let size: u32 = 3;
    let verbose: bool = true;
    let output: &str = "./test.png";
    
    let ulam_spiral = UlamSpiral::new(size);
    if verbose { print_matrix(&ulam_spiral, true); }
    save_as_image(&ulam_spiral, output);
}
