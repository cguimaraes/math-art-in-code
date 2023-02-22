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

fn create_matrix(size: usize) -> Vec<usize> {
    assert!(size > 0 && size % 2 != 0, "Matrix size must be odd and bigger than 0!");

    let mut matrix = vec![0; size * size];
    let mut dir: u8 = 0; /* 0: Right, 1: Up, 2: Left, 3: Down */
    let mut step: usize = 1;
    let mut x: usize = size / 2;
    let mut y: usize = size / 2;

    matrix[(y * size) + x] = 1;

    let mut i = 2;
    while i < (size * size) {
        for _j in 0..2 { // Step is incremented every two directions
            for _n in 0..step {                
                if i > (size * size) {
                    break;
                }

                match dir {
                    0 => x = x + 1,
                    1 => y = y - 1,
                    2 => x = x - 1,
                    3 => y = y + 1,
                    _ => panic!("I must not be here!"),
                }
                matrix[(y * size) + x] = i;
                i = i + 1;                
            }
            dir = (dir + 1) % 4;
        }
        step = step + 1;        
    }

    return matrix;
}

fn print_matrix(m: &Vec<usize>, as_numbers: bool) {
    let size: u32 = (m.len() as f32).sqrt() as u32;

    let mut pos: u32 = 1;
    for item in m {
        print!("{}{}",
               if as_numbers {*item} else {if is_prime(*item) { 1 } else { 0 }},
               if pos % size == 0 {"\n"} else {""});
        pos = pos + 1;
    }
}

fn save_as_image(m: Vec<usize>, path: &str) {
    let size: u32 = (m.len() as f32).sqrt() as u32;
    let mut img: image::GrayImage = ImageBuffer::new(size, size);

    let mut pos: u32 = 0;
    for item in m {
        img.put_pixel(pos % size,
                      pos / size,
                      if is_prime(item) { Luma([0]) } else { Luma([255]) });
        pos = pos + 1;
    }

    img.save(path).unwrap();
}

fn main() {
    let size: usize = 4001;
    let verbose: bool = false;
    let output: &str = "./test.png";
    
    let matrix = create_matrix(size);
    if verbose { print_matrix(&matrix, true); }
    save_as_image(matrix, output);
}
