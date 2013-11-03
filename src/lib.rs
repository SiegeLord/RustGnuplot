// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

#[link(name = "gnuplot",
       vers = "0.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustGnuplot")];

#[comment = "Rust gnuplot controller"];
#[license = "zlib"];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

/*!
A simple gnuplot controller.

# Example

~~~ {.rust}
use gnuplot::*;

let x = [0, 1, 2];
let y = [3, 4, 5];
let mut fg = Figure::new();
{
   fg.axes2d()
   .lines(x.iter(), y.iter(), [Caption("A line"), Color("black")]);
}
fg.show();
~~~
*/

pub use internal::axes2d::Axes2D;
pub use internal::axes3d::Axes2D;
pub use internal::coordinates::external::*;
pub use figure::*;
pub use options::*;
pub use datatype::*;

#[macro_escape]
mod util;
mod axes_common;
mod writer;
pub mod figure;
pub mod options; 
pub mod datatype;

mod internal
{
	pub mod axes2d;
	pub mod axes3d;
	pub mod coordinates;
}
