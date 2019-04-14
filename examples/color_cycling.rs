// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let x = 0..10;

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	let ax = fg.axes2d();
	ax.set_title("Color cycling", &[]);
	ax.set_legend(Graph(0.2), Graph(0.9), &[], &[]);
	for i in 0..10
	{
		ax.lines_points(
			x.clone(),
			x.clone().map(|v| v * 2 + i),
			&[Caption(&format!("{}", i))],
		);
	}

	c.show(&mut fg, "fg.color_cycling.gnuplot");
}

fn main()
{
	Common::new().map(|c| example(c));
}
