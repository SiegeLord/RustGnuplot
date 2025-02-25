use std::{fmt::Debug, iter};

// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn color_name<T: Debug>(color: &PlotOption<T>) -> String
{
	let s = format!("{:?}", color).replace("ColorOpt(", "");
	let mut chars = s.chars();
	chars.next_back();
	chars.as_str().to_string()
}

fn example(c: Common)
{
	let x = 0..5;

	let colors = [
		Color("black"),                            // Conversion to RGBString is implicit
		Color(ColorType::RGBString("black")),      // Explicit use of RGBString
		Color("red"),                              // Conversion to RGBString is implicit
		Color(RGBString("#ff0000")),               // red using Hex coded RRGGBB
		Color(RGBString("#00ff0000")),             // red using Hex coded AARRGGBB
		Color("#ff8888"), // pink using Hex coded RRGGBB. Conversion to RGBString is implict
		Color("#88ff0000"), // pink using Hex coded AARRGGBB. Conversion to RGBString is implict
		Color(ColorType::RGBString("#ffff0000")), // transparent using Hex coded AARRGGBB
		Color((128, 0, 255)), // purple using implict RGBInteger
		Color(RGBInteger(128, 0, 255)), // purple using explict RGBInteger
		Color((0.5, 0.0, 1.0)), // purple using implict float to int conversion
		Color(floats_to_rgb(0.5, 0.0, 1.0)), // purple using explicit float to int conversion
		Color((128, 128, 0, 255)), // pale purple using implict ARGBInteger
		Color(ARGBInteger(128, 128, 0, 255)), // pale purple using explict ARGBInteger
		Color((0.5, 0.5, 0.0, 1.0)), // pale purple using implict float to int conversion
		Color(floats_to_argb(0.5, 0.5, 0.0, 1.0)), // pale purple using explicit float to int conversion
	];

	let mut fg = Figure::new();
	let ax = fg.axes2d();
	ax.set_title(
		"Demo of RGBString in various forms\nSee code comments for how to construct the colors",
		&[],
	)
	.set_x_range(Fix(-9.0), Auto)
	.set_legend(Graph(0.5), Graph(0.9), &[], &[Font("", 14.0)]);

	let n_colors = colors.len();
	for (i, color) in colors.into_iter().enumerate()
	{
		ax.box_xy_error_delta(
			x.clone(),
			iter::repeat((n_colors - 1) - i),
			iter::repeat(0.4),
			iter::repeat(0.2),
			&[
				Caption(&color_name(&color)),
				LineWidth(1.0),
				BorderColor("black"),
				color,
			],
		);
	}

	// Draw line across the boxes in fixed black and background colors
	ax.lines(
		[0, 0],
		[0, n_colors - 1],
		&[
			LineWidth(7.0),
			Color(Black),
			Caption(&color_name(&Color(Black))),
		],
	);

	ax.lines(
		[4, 4],
		[0, n_colors - 1],
		&[
			LineWidth(7.0),
			Color(Background),
			Caption(&color_name(&Color(Background))),
		],
	);

	// any of the forms used for Color can also be used with TextColor and BorderColor
	ax.set_x_label(
		"Labels can be colored using the TextColor function",
		&[TextColor((128, 0, 255))],
	);

	c.show(&mut fg, "rgb_color");

	// ########################################################################

	let mut fg = Figure::new();
	let ax = fg.axes2d();
	let max_cb = 10.0;
	ax.set_cb_range(Fix(0.0), Fix(max_cb));
	for color_value in 0..=10
	{
		let color_float = color_value as f64;
		let frac_color = Color(PaletteFracColor(color_float / max_cb));
		let cb_range_color = Color(PaletteCBColor(color_float));

		ax.box_xy_error_delta(
			[color_value],
			[0],
			[0.4],
			[0.4],
			&[
				Caption(&color_name(&frac_color)),
				LineWidth(1.0),
				BorderColor("black"),
				frac_color,
			],
		)
		.box_xy_error_delta(
			[color_value],
			[1],
			[0.4],
			[0.4],
			&[
				Caption(&color_name(&cb_range_color)),
				LineWidth(1.0),
				BorderColor("black"),
				cb_range_color,
			],
		);
	}
	ax.set_x_range(Fix(-10.0), Fix(11.0))
		.set_y_range(Fix(-0.5), Fix(1.5))
		.set_legend(Graph(0.45), Graph(0.9), &[], &[Font("", 14.0)]);
	c.show(&mut fg, "palette_colors");
}

fn main()
{
	Common::new().map(|c| example(c));
}
