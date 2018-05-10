// This file is released into Public Domain.
extern crate gnuplot;

use common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let z = (0..100).map(|z| z as f32 / 10.0);
	let x = z.clone().map(|z| z.cos());
	let y = z.clone().map(|z| z.sin());

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes3d()
		.set_title("3D points", &[])
		.points(x, y, z, &[PointSymbol('o'), Color("#ffaa77"), PointSize(2.0)]);

	c.show(&mut fg, "fg.points_3d.gnuplot");
}

fn main()
{
	Common::new().map(|c| example(c));
}
