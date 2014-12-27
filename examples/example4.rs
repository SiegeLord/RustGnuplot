// This file is released into Public Domain.
#![feature(globs)]
#![feature(unboxed_closures)]

extern crate gnuplot;

use std::num::{Float, FloatMath};

use gnuplot::*;

mod common;

fn example(show: |fg: &mut Figure, filename: &str|, set_term: |fg: &mut Figure|)
{
	let zw = 61u;
	let zh = 61u;
	let mut z1 = Vec::with_capacity((zw * zh) as uint);
	for i in range(0, zh)
	{
		for j in range(0, zw)
		{
			let y = 8.0 * (i as f64) / zh as f64 - 4.0;
			let x = 8.0 * (j as f64) / zw as f64 - 4.0;
			z1.push(x.cos() * y.cos() / ((x*x + y*y).sqrt() + 1.0));
		}
	}

	let mut fg = Figure::new();
	set_term(&mut fg);

	fg.axes2d()
	.set_title("Image", &[])
	.set_cb_range(Fix(-1.0), Fix(1.0))
	.set_cb_ticks(Some((Fix(0.25), 1)), &[], &[])
	.set_cb_label("Label", &[Rotate(0.0)])
	.image(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[]);

	show(&mut fg, "fg4.1.gnuplot");

	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes3d()
	.set_title("Surface", &[])
	.surface(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[])
	.set_x_label("X", &[])
	.set_y_label("Y", &[])
	.set_z_label("Z", &[])
	.set_z_range(Fix(-1.0), Fix(1.0))
	.set_z_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
	.set_cb_range(Fix(-1.0), Fix(1.0))
	.set_view(45.0, 45.0);

	show(&mut fg, "fg4.2.gnuplot");
}

fn main()
{
	common::run().map(|(_, f, t)| example(|fg, fi| f.call((fg, fi)), |fg| t.call((fg,))));
}
