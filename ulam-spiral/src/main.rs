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
    pub fn new(size: usize) -> Self {
        Self {
            elems: vec![0; size * size]
        }
    }

    pub fn len(&self) -> usize {
        return self.elems.len();
    }

    pub fn width(&self) -> usize {
        return (self.len() as f32).sqrt() as usize;
    }
}

impl IntoIterator for SquareMatrix {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}

impl Index<(usize, usize)> for SquareMatrix {
    type Output = u32;

    fn index(&self, idx: (usize, usize)) -> &u32 {
        let width = self.width();
        return &self.elems[((idx.1 * width) + idx.0)];
    }
}

impl IndexMut<(usize, usize)> for SquareMatrix {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut u32 {
        let width = self.width();
        return &mut self.elems[((idx.1 * width) + idx.0)];
    }
}

struct UlamSpiral {
    matrix: SquareMatrix
}

impl UlamSpiral {
    pub fn new(size: usize) -> Self {
        assert!(size > 0 && size % 2 != 0, "Matrix size must be odd and bigger than 0!");

        let mut matrix = SquareMatrix::new(size);

        let mut dir = Directions::RIGHT;
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
                        Directions::RIGHT => xy_cursor.0 += 1,
                        Directions::UP => xy_cursor.1 -= 1,
                        Directions::LEFT => xy_cursor.0 -= 1,
                        Directions::DOWN => xy_cursor.1 += 1,
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
        return &self.matrix.elems;
    }

    pub fn width(&self) -> usize {
        return self.matrix.width();
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
    let width = us.width();

    for (i, &elem) in us.elems().into_iter().enumerate() {
        print!("{}{}",
               if as_numbers {elem} else {if is_prime(elem) { 1 } else { 0 }},
               if (i + 1) % width == 0 {"\n"} else {""});
    }
}

fn save_as_image(us: &UlamSpiral, path: &str) {
    let width = us.width();
    let mut img: image::GrayImage = ImageBuffer::new(width.try_into().unwrap(), width.try_into().unwrap());

    for (i, &elem) in us.elems().into_iter().enumerate() {
        let x = i  % width;
        let y = i / width;
        img.put_pixel(x.try_into().unwrap(),
                      y.try_into().unwrap(),
                      if is_prime(elem) { Luma([0]) } else { Luma([255]) });
    }

    img.save(path).unwrap();
}

fn main() {
    let size = 3;
    let verbose = true;
    let output = "./test.png";
    
    let ulam_spiral = UlamSpiral::new(size);
    if verbose { print_matrix(&ulam_spiral, true); }
    save_as_image(&ulam_spiral, output);
}
