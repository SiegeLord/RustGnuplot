// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let z = (0..100).map(|z| z as f32 / 10.0);
	let x = z.clone().map(|z| z.cos());
	let y = z.clone().map(|z| z.sin());

	let mut fg = Figure::new();

	fg.axes3d().set_title("3D lines", &[]).lines(
		x,
		y,
		z,
		&[PointSymbol('o'), Color("#ffaa77".into()), PointSize(2.0)],
	);

	c.show(&mut fg, "lines_3d");
}

fn main()
{
	Common::new().map(|c| example(c));
}
