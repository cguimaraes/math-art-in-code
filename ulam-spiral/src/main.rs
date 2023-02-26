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

mod ulam_spiral;
mod square_matrix;
mod directions;
mod utils;

use crate::ulam_spiral::{UlamSpiral, UlamSpiralFormat};

fn main() {
    let size = 3;
    let verbose = true;
    let output = "./test.png";
    
    let ulam_spiral = UlamSpiral::new(size, UlamSpiralFormat::Square);
    if verbose { ulam_spiral.print(true); }
    ulam_spiral.save_as_image(output);
}
