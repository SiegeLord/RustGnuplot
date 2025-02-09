// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Box XY Error", &[])
		.box_xy_error_delta(
			[0.0f32, 1.0, 2.0].iter(),
			[-1.0f32, 0.0, 1.0].iter(),
			[0.25f32, 0.375, 0.15].iter(),
			[2.0f32, 3.0, 4.0].iter(),
			&[],
		)
		.box_xy_error_low_high(
			[-0.6f32, 1.5, 2.5].iter(),
			[-1.0f32, 0.0, 1.0].iter(),
			[-0.9f32, -1.0, 2.2].iter(),
			[-0.45f32, 3.0, 2.95].iter(),
			[-1.5f32, 4.5, 3.0].iter(),
			[0.5f32, 4.75, 0.125].iter(),
			&[
				Color("blue"),
				LineWidth(2.0),
				LineStyle(SmallDot),
				FillAlpha(0.5),
			],
		)
		.set_x_range(Fix(-1.0), Fix(3.0))
		.set_y_range(Fix(-3.0), Fix(5.0));

	c.show(&mut fg, "box_xy_error");
}

fn main()
{
	Common::new().map(|c| example(c));
}
