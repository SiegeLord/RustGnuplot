// This file is released into Public Domain.
#![feature(globs)]
#![feature(unboxed_closures)]

extern crate gnuplot;

use std::vec::Vec;
use std::num::{Float, FloatMath};

use gnuplot::*;

mod common;

fn example(show: |fg: &mut Figure, filename: &str|, set_term: |fg: &mut Figure|)
{
	let w = 61u;
	let h = 61u;
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
	set_term(&mut fg);
	
	fg.axes3d()
	.set_title("Surface", &[])
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_view(45.0, 45.0);
	
	show(&mut fg, "fg3.1.gnuplot");

	let mut fg = Figure::new();
	set_term(&mut fg);

	fg.axes3d()
	.set_title("Map", &[])
	.surface(z1.iter(), w, h, None, &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_view_map();
	
	show(&mut fg, "fg3.2.gnuplot");
	
	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes3d()
	.set_pos_grid(2, 2, 0)
	.set_title("Base", &[])
	.show_contours(true, false, Cubic(10), Fix(""), Auto)
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_view(45.0, 45.0);

	fg.axes3d()
	.set_pos_grid(2, 2, 1)
	.set_title("Surface", &[])
	.show_contours(false, true, Linear, Fix(""), Auto)
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_view(45.0, 45.0);

	fg.axes3d()
	.set_pos_grid(2, 2, 2)
	.set_title("Both + Fix Levels", &[])
	.show_contours(true, true, Linear, Fix("%f"), Fix(1))
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_view(45.0, 45.0);
	
	fg.axes3d()
	.set_pos_grid(2, 2, 3)
	.set_title("Custom Levels", &[])
	.show_contours_custom(true, false, Linear, Fix(""), Some(0f32).iter())
	.surface(z1.iter(), w, h, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_view(45.0, 45.0);
	
	show(&mut fg, "fg3.3.gnuplot");
}

fn main()
{
	common::run().map(|(_, f, t)| example(|fg, fi| f.call((fg, fi)), |fg| t.call((fg,))));
}
