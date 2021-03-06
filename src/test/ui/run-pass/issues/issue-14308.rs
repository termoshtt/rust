// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass

struct A(isize);

fn main() {
    let x = match A(3) {
        A(..) => 1
    };
    assert_eq!(x, 1);
    let x = match A(4) {
        A(1) => 1,
        A(..) => 2
    };
    assert_eq!(x, 2);
}
