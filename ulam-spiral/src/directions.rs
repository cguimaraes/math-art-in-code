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

pub enum Directions {
    Right = 0,
    Up = 1,
    Left = 2,
    Down = 3
}

impl Directions
{
    pub fn rotate_counter_clockwise(&self) -> Directions {
        match self {
            Directions::Right => return Directions::Up,
            Directions::Up => return Directions::Left,
            Directions::Left => return Directions::Down,
            Directions::Down => return Directions::Right
        };
    }
}
