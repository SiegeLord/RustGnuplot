#[link(name = "gnuplot",
       vers = "0.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustGnuplot")];

#[comment = "Rust gnuplot controller"];
#[license = "zlib"];
#[crate_type = "lib"];

pub use axes2d::*;
pub use axes3d::*;
pub use figure::*;
pub use options::*;
pub use util::*;

mod axes_common;
mod axes2d;
mod axes3d;
mod figure;
mod options; 
mod util;
