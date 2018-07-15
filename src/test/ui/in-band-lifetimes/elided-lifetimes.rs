// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-rustfix
// compile-flags: --edition 2018

#![allow(unused)]
#![deny(elided_lifetimes_in_paths)]
//~^ NOTE lint level defined here

use std::cell::{RefCell, Ref};


struct Foo<'a> { x: &'a u32 }

fn foo(x: &Foo) {
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
}

fn bar(x: &Foo<'_>) {}


struct Wrapped<'a>(&'a str);

struct WrappedWithBow<'a> {
    gift: &'a str
}

fn wrap_gift(gift: &str) -> Wrapped {
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
    Wrapped(gift)
}

fn wrap_gift_with_bow(gift: &str) -> WrappedWithBow {
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
    WrappedWithBow { gift }
}

macro_rules! autowrapper {
    ($type_name:ident, $fn_name:ident, $lt:lifetime) => {
        struct $type_name<$lt> {
            gift: &$lt str
        }

        fn $fn_name(gift: &str) -> $type_name {
            //~^ ERROR implicit lifetime parameters in types are deprecated
            //~| HELP indicate the anonymous lifetime
            $type_name { gift }
        }
    }
}

autowrapper!(Autowrapped, autowrap_gift, 'a);
//~^ NOTE in this expansion of autowrapper!
//~| NOTE in this expansion of autowrapper!

macro_rules! anytuple_ref_ty {
    ($($types:ty),*) => {
        Ref<($($types),*)>
        //~^ ERROR implicit lifetime parameters in types are deprecated
        //~| HELP indicate the anonymous lifetime
    }
}

fn main() {
    let honesty = RefCell::new((4, 'e'));
    let loyalty: Ref<(u32, char)> = honesty.borrow();
    //~^ ERROR implicit lifetime parameters in types are deprecated
    //~| HELP indicate the anonymous lifetime
    let generosity = Ref::map(loyalty, |t| &t.0);

    let laughter = RefCell::new((true, "magic"));
    let yellow: anytuple_ref_ty!(bool, &str) = laughter.borrow();
    //~^ NOTE in this expansion of anytuple_ref_ty!
    //~| NOTE in this expansion of anytuple_ref_ty!
}
