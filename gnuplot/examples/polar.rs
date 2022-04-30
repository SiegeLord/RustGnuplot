// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let t = (0..100).map(|t| t as f32 / 10.0);
	let r = t.clone().map(|t| t * t);

	let mut fg = Figure::new();

	fg.axes_polar().set_title("Polar plot", &[]).lines(
		t,
		r,
		&[PointSymbol('o'), Color("#ffaa77"), PointSize(2.0)],
	);

	c.show(&mut fg, "polar");
}

fn main()
{
	Common::new().map(|c| example(c));
}
