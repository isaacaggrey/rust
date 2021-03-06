// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.




// -*- rust -*-
fn id<T: Copy>(x: T) -> T { return x; }

type triple = {x: int, y: int, z: int};

fn main() {
    let mut x = 62;
    let mut y = 63;
    let a = 'a';
    let mut b = 'b';
    let p: triple = {x: 65, y: 66, z: 67};
    let mut q: triple = {x: 68, y: 69, z: 70};
    y = id::<int>(x);
    log(debug, y);
    assert (x == y);
    b = id::<char>(a);
    log(debug, b);
    assert (a == b);
    q = id::<triple>(p);
    x = p.z;
    y = q.z;
    log(debug, y);
    assert (x == y);
}
