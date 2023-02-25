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

pub struct SquareMatrix {
    elems: Vec<u32>
}

impl SquareMatrix {
    pub fn new(size: usize) -> Self {
        Self {
            elems: vec![0; size * size]
        }
    }

    pub fn elems(&self) -> &Vec<u32> {
        return &self.elems;
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
