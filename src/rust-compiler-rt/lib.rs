// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![crate_type = "staticlib"]
#![no_std]
#![feature(core, no_std)]

#[no_mangle]
pub extern "C" fn __powisf2(x: f32, n: i32) -> f32 {
    let (mut r, mut a) = (1., x);
    let mut b = if n < 0 { n.wrapping_neg() as u32 } else { n as u32 };
    loop {
        if b & 1 != 0 { r *= a }
        b >>= 1;
        if b == 0 { break }
        a *= a;
    }
    if n < 0 { r.recip() } else { r }
}
#[no_mangle]
pub extern "C" fn __powidf2(x: f64, n: i32) -> f64 {
    let (mut r, mut a) = (1., x);
    let mut b = if n < 0 { n.wrapping_neg() as u32 } else { n as u32 };
    loop {
        if b & 1 != 0 { r *= a }
        b >>= 1;
        if b == 0 { break }
        a *= a;
    }
    if n < 0 { r.recip() } else { r }
}
