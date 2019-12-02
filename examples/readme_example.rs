// This file is released into Public Domain.
use crate::common::*;

use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("A plot", &[])
		.set_legend(Graph(0.5), Graph(0.9), &[], &[])
		.set_x_label("x", &[])
		.set_y_label("y^2", &[])
		.lines(
			&[-3., -2., -1., 0., 1., 2., 3.],
			&[9., 4., 1., 0., 1., 4., 9.],
			&[Caption("Parabola")],
		);

	c.show(&mut fg, "readme_example");
	if !c.no_show
	{
		fg.set_terminal("pngcairo", "readme_example.png");
		fg.show().unwrap();
	}
}

fn main()
{
	Common::new().map(|c| example(c));
}
