// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused)]

struct PersonalityInventory {
    expressivity: f32,
    instrumentality: f32
}

impl PersonalityInventory {
    fn expressivity(&self) -> f32 {
        match *self {
            PersonalityInventory { expressivity: exp, ... } => exp
            //~^ ERROR expected field pattern, found `...`
            //~| ERROR cannot find value `exp` in this scope [E0425]
            //~| ERROR pattern does not mention field `expressivity` [E0027]
            //~| ERROR pattern does not mention field `instrumentality` [E0027]
        }
    }
}

fn main() {}
