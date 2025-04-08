// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Box and whisker", &[])
		.box_and_whisker(
			[-0.6f32, 1.5, 2.5].iter(),
			[-1.0f32, 0.0, 1.0].iter(),
			[-2.0f32, -1.0, 0.0].iter(),
			[2.0f32, 3.0, 4.0].iter(),
			[1.0f32, 2.0, 3.0].iter(),
			&[
				BoxWidth([0.5f64, 0.25, 0.125].into()),
				WhiskerBars(0.5),
				Color("blue".into()),
				LineWidth(2.0),
				LineStyle(SmallDot),
				FillAlpha(0.5),
			],
		)
		.box_and_whisker(
			[0.0f32, 1.0, 2.0].iter(),
			[-1.0f32, 0.0, 1.0].iter(),
			[-2.0f32, -1.0, 0.0].iter(),
			[2.0f32, 3.0, 4.0].iter(),
			[1.0f32, 2.0, 3.0].iter(),
			&[],
		)
		.set_x_range(Fix(-1.0), Fix(3.0))
		.set_y_range(Fix(-3.0), Fix(5.0));

	c.show(&mut fg, "box_and_whisker");
}

fn main()
{
	Common::new().map(|c| example(c));
}
