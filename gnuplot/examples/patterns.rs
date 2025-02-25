// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();

	let ax = fg.axes2d();
	ax.set_title("Patterns", &[]);
	ax.set_legend(Graph(1.), Graph(0.95), &[MaxRows(3)], &[]);
	ax.set_y_range(Auto, Fix(8.));
	ax.set_box_width(0.5, false);
	for i in 0..=8
	{
		ax.boxes(&[i], &[5], &[FillPattern(Auto)]);
	}

	for (i, &pattern) in [
		Pattern0,
		BigCrosses,
		SmallCrosses,
		Pattern3,
		BigBackSlashes,
		BigForwardSlashes,
		SmallForwardSlashes,
		SmallBackSlashes,
		Pattern8,
	]
	.iter()
	.enumerate()
	{
		ax.boxes(
			&[i],
			&[-5],
			&[
				FillPattern(Fix(pattern)),
				Caption(&format!("{:?}", pattern)),
			],
		);
	}

	c.show(&mut fg, "patterns");
}

fn main()
{
	Common::new().map(|c| example(c));
}
