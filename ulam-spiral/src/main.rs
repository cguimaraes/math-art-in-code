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

use clap::Parser;

/// Generate Ulam Spiral
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of Ulam Spiral to draw (Square or Circle)
    #[arg(short, long)]
    format: UlamSpiralFormat,

    /// Last value for the Ulam Spiral
    #[arg(short, long, default_value_t = 1001)]
    last: usize,

    /// Output file
    #[arg(short, long)]
    output: String,

    /// Image width
    #[arg(short, long, default_value_t = 1001)]
    width: usize,

    /// Zoom out scale multiplier
    #[arg(short, long, default_value_t = 1)]
    scale: usize,

    /// Dot size for prime numbers
    #[arg(short, long, default_value_t = 1)]
    dot: usize,

    /// Verbosity
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let ulam_spiral = UlamSpiral::new(args.last, args.format);
    if args.verbose { ulam_spiral.print(true); }
    ulam_spiral.save_as_image(&args.output, args.width, args.scale, args.dot);
}
