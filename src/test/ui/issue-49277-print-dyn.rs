// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt::{Debug, Display};

macro_rules! demonstrate_type_mismatch {
    ($name:ident, $value:expr, $type1:ty, $type2:ty) => {
        fn $name() {
            let x: $type1 = $value;
            let _: $type2 = x;
        }
    }
}

macro_rules! demonstrate_trait_object_type_mismatches {
    ($value:expr, $T:path, $U:path) => {
        demonstrate_type_mismatch!(_box, Box::new($value), Box<$T>, Box<$U>);
        demonstrate_type_mismatch!(_ref, &$value, &dyn $T, &dyn $U);
        demonstrate_type_mismatch!(_mut_ref, &mut $value, &mut dyn $T, &mut dyn $U);
        demonstrate_type_mismatch!(_slice_ref, &[&$value], &[&dyn $T], &[&dyn $U]);
    }
}

demonstrate_trait_object_type_mismatches!(1, Debug, Display);

fn main() {}
