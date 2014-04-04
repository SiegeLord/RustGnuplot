// This file is released into Public Domain.
#![feature(globs)]

extern crate gnuplot;
extern crate getopts;

use getopts::*;
use std::os;
use std::vec::Vec;

use gnuplot::*;

fn main()
{
	let args = os::args();
	
	let opts = ~[
		optflag("n", "no-show", "do not run the gnuplot process")
	];
	
	let matches = match getopts(args.tail(), opts)
	{
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};
	
	let show = !matches.opt_present("n");

	let w = 61i32;
	let h = 61i32;
	let mut z1 = Vec::with_capacity((w * h) as uint);
	for i in range(0, h)
	{
		for j in range(0, w)
		{
			let y = 8.0 * (i as f64) / h as f64 - 4.0;
			let x = 8.0 * (j as f64) / w as f64 - 4.0;
			z1.push(x.cos() * y.cos() / ((x*x + y*y).sqrt() + 1.0));
		}
	}
	
	let mut fg = Figure::new();
	
	fg.axes3d()
	.set_title("Surface", [])
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), [])
	.set_x_label("X", [])
	.set_y_label("Y", [])
	.set_z_label("Z", [])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Fix(1.0), 2, [Mirror(false)], [])
	.set_view(45.0, 45.0);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg3.1.gnuplot");

	let mut fg = Figure::new();

	fg.axes3d()
	.set_title("Map", [])
	.surface(z1.iter(), w, h, None, [])
	.set_x_label("X", [])
	.set_y_label("Y", [])
	.set_view_map();
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg3.2.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.set_grid(2, 2);
	
	fg.axes3d()
	.set_pos_grid(1, 1)
	.set_title("Base", [])
	.show_contours(true, false, Cubic(10), Auto)
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), [])
	.set_view(45.0, 45.0);

	fg.axes3d()
	.set_pos_grid(1, 2)
	.set_title("Surface", [])
	.show_contours(false, true, Linear, Auto)
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), [])
	.set_view(45.0, 45.0);

	fg.axes3d()
	.set_pos_grid(2, 1)
	.set_title("Both + Fix Levels", [])
	.show_contours(true, true, Linear, Fix(1))
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), [])
	.set_view(45.0, 45.0);
	
	fg.axes3d()
	.set_pos_grid(2, 2)
	.set_title("Custom Levels", [])
	.show_contours_custom(true, false, Linear, Some(0f32).iter())
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), [])
	.set_view(45.0, 45.0);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg3.3.gnuplot");
}
