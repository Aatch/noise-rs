// Copyright 2013 The noise-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
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

#![deny(missing_copy_implementations)]

pub use seed::Seed;
pub use perlin::{perlin2_fast, perlin2_best, perlin3_fast, perlin3_best, perlin4_fast, perlin4_best};
pub use simplex::simplex2;
pub use brownian::{brownian2, brownian3, brownian4};

mod gen;
mod gradients;

pub mod util;
pub mod source;

pub mod seed;
pub mod perlin;
pub mod simplex;
pub mod brownian;

pub type Point2<T> = [T, ..2];
pub type Point3<T> = [T, ..3];
pub type Point4<T> = [T, ..4];

#[deriving(Copy, Clone)]
pub enum Quality {
    Fast,
    Best,
}
