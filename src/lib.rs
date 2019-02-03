// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#![allow(unused_must_use)]
#![forbid(unstable_features)]
/*!
A simple gnuplot controller.

# Example

~~~no_run
# extern crate gnuplot;
# fn main() {
use gnuplot::{Figure, Caption, Color};

let x = [0u32, 1, 2];
let y = [3u32, 4, 5];
let mut fg = Figure::new();
fg.axes2d()
.lines(&x, &y, &[Caption("A line"), Color("black")]);
fg.show();
# }
~~~
*/
pub use crate::axes2d::Axes2D;
pub use crate::axes3d::Axes3D;
pub use crate::axes_common::AxesCommon;
pub use crate::coordinates::*;
pub use crate::datatype::*;
pub use crate::figure::*;
pub use crate::options::*;

#[macro_use]
mod util;

mod axes2d;
mod axes3d;
mod axes_common;
mod coordinates;
mod datatype;
mod figure;
mod options;
mod writer;
