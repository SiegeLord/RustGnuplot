// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();
	let _ax = fg
		.axes2d()
		.label(
			"multi\nline string",
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.9),
			&[],
		)
		.label(
			"x^2 x_2 {/Times*2 abc} \\{\\}\\^\\_",
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.8),
			&[],
		)
		.label(
			"Monospace",
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.6),
			&[Font("Monospace", 32.)],
		)
		.label(
			"Align Right",
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.5),
			&[TextAlign(AlignRight)],
		)
		.label(
			"Align Centre",
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.4),
			&[TextAlign(AlignCenter)],
		)
		.label(
			"~{Over}{Print}", // Why does gnuplot have this feature?
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.3),
			&[TextAlign(AlignCenter)],
		)
		.label(
			"Tab\tCharacter", // Strange rendering on this one
			Coordinate::Graph(0.5),
			Coordinate::Graph(0.2),
			&[TextAlign(AlignCenter)],
		)
		.lines(&[-2., -2.], &[-3., 3.], &[])
		.set_x_ticks(None, &[], &[])
		.set_y_ticks(None, &[], &[])
		.set_border(true, &[], &[]);

	c.show(&mut fg, "text");
}

fn main()
{
	Common::new().map(|c| example(c));
}
