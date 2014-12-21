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

use std::rand;

use util::signed_modulus;

const TABLE_SIZE: uint = 256;

#[allow(missing_copy_implementations)]
pub struct Seed {
    values: [uint, ..TABLE_SIZE*2],
}

impl Seed {
    pub fn new(seed: uint) -> Seed {
        let mut rng: rand::StdRng = rand::SeedableRng::from_seed([seed].as_slice());
        let mut seq: Vec<uint> = range(0u, TABLE_SIZE).collect();
        for i in range(0, TABLE_SIZE) {
            let mut swap_i: uint = rand::Rand::rand(&mut rng);
            swap_i = swap_i % TABLE_SIZE;
            let swap = seq[swap_i as uint];
            seq[swap_i as uint] = seq[i];
            seq[i] = swap;
        }

        // It's unfortunate that this double-initializes the array, but Rust doesn't currently provide a
        // clean way to do this in one pass. Hopefully won't matter, as Seed creation will usually be a
        // one-time event.
        let mut new_seed = Seed {
            values: [0, ..TABLE_SIZE*2],
        };
        for i in range(0, TABLE_SIZE*2) {
            new_seed.values[i] = seq[i % TABLE_SIZE];
        }
        return new_seed;
    }

    pub fn get1d(&self, x: int) -> uint {
        return self.values[(signed_modulus(x, TABLE_SIZE as int))];
    }

    pub fn get2d(&self, x: int, y: int) -> uint {
        return self.values[(signed_modulus(y, TABLE_SIZE as int)) + self.get1d(x)];
    }

    pub fn get3d(&self, x: int, y: int, z: int) -> uint {
        return self.values[(signed_modulus(z, TABLE_SIZE as int)) + self.get2d(x, y)];
    }

    pub fn get4d(&self, x: int, y: int, z: int, w: int) -> uint {
        return self.values[(signed_modulus(w, TABLE_SIZE as int)) + self.get3d(x, y, z)];
    }
}
