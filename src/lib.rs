// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#![crate_name="gnuplot"]

#![crate_type = "lib"]

#![feature(old_io)]
#![feature(old_path)]
#![feature(core)]
#![allow(unused_must_use)]

/*!
A simple gnuplot controller.

# Example

~~~no_run
# #![feature(globs)]
# extern crate gnuplot;
# fn main() {
use gnuplot::{Figure, Caption, Color};

let x = &[0u32, 1, 2];
let y = &[3u32, 4, 5];
let mut fg = Figure::new();
fg.axes2d()
.lines(x.iter(), y.iter(), &[Caption("A line"), Color("black")]);
fg.show();
# }
~~~
*/

pub use coordinates::*;
pub use datatype::*;
pub use figure::*;
pub use axes2d::Axes2D;
pub use axes3d::Axes3D;
pub use options::*;
pub use axes_common::AxesCommon;

#[macro_use]
mod util;

mod axes2d;
mod axes3d;
mod axes_common;
mod writer;
mod figure;
mod options; 
mod datatype;
mod coordinates;
