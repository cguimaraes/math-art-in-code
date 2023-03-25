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

pub fn is_prime(n: usize) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in 2..(n / 2) + 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn rotate_point(point: (usize, usize), angle: f32) -> (f32, f32) {
    (
        point.0 as f32 * angle.cos() - point.1 as f32 * angle.sin(),
        point.0 as f32 * angle.sin() + point.1 as f32 * angle.cos(),
    )
}
