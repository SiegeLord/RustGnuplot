#[link(name = "gnuplot",
       vers = "0.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustGnuplot")];

#[comment = "Rust gnuplot controller"];
#[license = "zlib"];
#[crate_type = "lib"];

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

pub use axes2d::*;
pub use axes3d::*;
pub use figure::*;
pub use options::*;
pub use datatype::*;
pub use coordinates::*;

mod axes_common;
mod writer;

pub mod axes2d;
pub mod axes3d;
pub mod figure;
pub mod options; 
pub mod datatype;
pub mod coordinates;
