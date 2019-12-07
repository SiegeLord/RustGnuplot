// This file is released into Public Domain.
use crate::common::*;
use gnuplot::{MultiplotFillDirection::*, MultiplotFillOrder::*};

use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();
	fg.set_multiplot_layout(2, 2)
		.set_title("Multiple parabolas")
		.set_scale(0.8, 0.8)
		.set_offset(0.0, 0.0)
		.set_multiplot_fill_order(RowsFirst, Upwards);

	fg.axes2d()
		.lines(
			&[-3., -2., -1., 0., 1., 2., 3.],
			&[9., 4., 1., 0., 1., 4., 9.],
			&[Caption("Parabola 1")],
		)
		.set_x_label("X label", &[])
		.set_title("Parabola 1", &[])
		.label("Test 1", Axis(-3.), Axis(-3.), &[])
		.label("Test 2", Axis(3.), Axis(3.), &[])
		.arrow(Axis(-3.), Axis(-3.), Axis(3.), Axis(3.), &[]);

	fg.axes2d().lines(
		&[-3., -2., -1., 0., 1., 2., 3.],
		&[10., 5., 2., 0., 2., 5., 10.],
		&[Caption("Parabola 2")],
	);

	fg.axes2d().lines(
		&[-3., -2., -1., 0., 1., 2., 3.],
		&[11., 6., 3., 0., 3., 6., 11.],
		&[Caption("Parabola 3")],
	);

	c.show(&mut fg, "multiplot_options");
}

fn main()
{
	Common::new().map(|c| example(c));
}
