// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


#[deriving_eq]
enum t { a, b(~str), }

fn make(i: int) -> t {
    if i > 10 { return a; }
    let mut s = ~"hello";
    // Ensure s is non-const.

    s += ~"there";
    return b(s);
}

fn main() {
    let mut i = 0;


    // The auto slot for the result of make(i) should not leak.
    while make(i) != a { i += 1; }
}
