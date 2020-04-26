// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let x = 0..10;

	let mut fg = Figure::new();

	let ax = fg.axes2d();
	ax.set_title("Dash type", &[]);
	ax.set_legend(Graph(0.3), Graph(0.9), &[], &[]);
	for (i, &dt) in [Solid, SmallDot, Dot, Dash, DotDash, DotDotDash]
		.iter()
		.enumerate()
	{
		ax.lines(
			x.clone(),
			x.clone().map(|v| v * 2 + 2 * i),
			&[
				LineWidth(2.),
				Color("black"),
				LineStyle(dt),
				Caption(&format!("{:?}", dt)),
			],
		);
	}

	c.show(&mut fg, "dash_type");
}

fn main()
{
	Common::new().map(|c| example(c));
}
