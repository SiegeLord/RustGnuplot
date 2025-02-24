use std::iter;

// This file is released into Public Domain.
use crate::common::*;
use gnuplot::{palettes::MAGMA, *};

mod common;

// https://github.com/gnuplot/gnuplot/blob/master/demo/candlesticks.dat
static CANDLESTICKS_STR: &str = "1	1.5	2 	2.4	4	6.
2	1.5	3 	3.5	4	5.5
3	4.5	5 	5.5	6	6.5
4	3.7	4.5 	5.0	5.5	6.1
5	3.1	3.5	4.2 	5	6.1
6	1  	4 	5.0	6   	9
7	4  	4 	4.8	6   	6.1
8	4  	5 	5.1	6   	6.1
9	1.5	2 	2.4	3	3.5
10	2.7	3 	3.5	4	4.3";

fn example(c: Common) {

	let data: Vec<Vec<f64>> = CANDLESTICKS_STR
		.split("\n")
		.map(|line| {
			line.split("\t")
				.map(|v| v.trim().parse::<f64>().unwrap())
				.collect()
		})
		.collect();
	let extract_col = |i| data.iter().map(|l| l[i]).collect::<Vec<_>>();

	let d1 = extract_col(0);
	let d2 = extract_col(1);
	// let d3 = extract_col(2);
	// let d4 = extract_col(3);
	let d5 = extract_col(4);
	let d6 = extract_col(5);
	let row_index: Vec<_> = (1..=d1.len() as u8).collect();
	let by3 = |x| (((x % 3.0) + 1.0) / 6.0);
	let by4 = |x| (((x % 4.0) + 1.0) / 7.0);

	let argb_formula = |x: &f64| {
		let a = 255.0 * (x - 5.5).abs() / 5.5;
		let r = x * 51.0 / 2.0;
		let g = (11.0 - x) * 51.0 / 2.0;
		let b = ((5.5 - x).abs() * 2.0 * 510.0 / 9.0).round();
		(a as u8, r as u8, g as u8, b as u8)
	};
	// Demo/test of variable color in many different plot styles
	// Inspired by https://gnuplot.sourceforge.net/demo_6.0/varcolor.html
	//
	// The loop is run four times with different color sets: each one sets all the elements of a given
	// plot to a different color, while making sure the colors align by x position: i.e. everything at x = 1
	// uses the first color, everything at x = 2 uses the second and so on.
	//
	//
	// The first color loop demonstrates usage of VariableIndexColor with indices to use gnuplot's default color styles,
	// but make them align for multiple plot items on the same axis. This is implicity constructed from a Vec<u8> using
	// the `Color` function but could equivalently be created explicitly using `ColorOpt(ColorType::VariableIndexColor(row_index.clone()))`
	//
	// The second color loop uses a VariablePaletteColor: this selects the color based on the current color palette and the
	// input value for each data point. The palette is scaled to the maximum value in the vector of `f64`s passed
	// to the VariablePaletteColor.
	//
	// The third color loop uses an (implicit) VariableARGBColor. The `Vec<(u8, u8, u8, u8)>` needed to constcruct the color
	// is calculated in this case by the `argb_formula()` closure. An explicit VariableARGBColor could also be constructed using
	// `ColorOpt(ColorType::VariableARGBColor(data)``. A VariableRGBColor is also defined that takes a 3-tuple of u8, rather than
	// a 4 tuple.
	for (color, label) in [
		(Color(row_index.clone()), "VariableIndexColor"),
		(
			ColorOpt(VariablePaletteColor(
				row_index.iter().map(|v| *v as f64).collect(),
			)),
			"VariablePaletteColor",
		),
		(
			Color(d1.iter().map(argb_formula).collect::<Vec<_>>()),
			"VariableARGBColor",
		),
	] {
		let mut fg = Figure::new();
		let ax = fg.axes2d();
		ax.set_title(
			&format!("variable color boxerror, points, xyerrorbars, and boxxyerror.\nColor used is a {label}"),
			&[],
		);
		ax.set_y_range(Fix(-4.0), Fix(11.5));
		ax.box_error_low_high_set_width(
			&d1,
			&d5,
			&d2,
			&d6,
			iter::repeat(0.2),
			&[
				color.clone(),
				FillAlpha(0.5),
				BorderColor(RGBColor("black")),
			],
		);
		ax.points(&d1, iter::repeat(1), &[color.clone(), PointSymbol('D')]);
		ax.xy_error_bars(
			&d1,
			iter::repeat(8),
			d1.iter().map(by3),
			d1.iter().map(by4),
			&[color.clone()],
		);
		ax.points(
			&d1,
			d2.iter().map(|v| -v / 2.0),
			&[color.clone(), PointSymbol('O'), PointSize(3.0)],
		);
		ax.box_xy_error_delta(
			&d1,
			iter::repeat(10),
			d1.iter().map(by3),
			d1.iter().map(by4),
			&[color.clone(), BorderColor(RGBColor("black"))],
		);

		c.show(&mut fg, "variable_color");
	}

	// #####################################################################
	// The example below shows the same graphs as in the loop, but using a set of saved colormaps
	// similar to palette in gnuplot terms, but the current palette is applied to all plots by default, and
	// multiple named colormaps can be created).
	//
	// As with VariablePaletteColor, this Color takes a vector of f64 that says which point in the colormap to use,
	// but it also takes a the name of the colormap from which to draw the colors.
	//
	// Note that the Color range appears to be shared across plots: i.e. if one plot has
	// color data (the Vec<f64>) in the range 0-1, and another in the range 1-100, all the
	// colors in the first plot will be right at the bottom end of it's colormap, even if that's
	// a different colormap to the one used in the second plot.
	let mut fg = Figure::new();
	let ax = fg.axes2d();

	// First create the colormaps we will later refer to
	// MAGMA is one of the colormaps provide with rust gnuplot
	ax.create_colormap("magma", MAGMA);
	// HOT is one of the colormaps provide with rust gnuplot
	ax.create_colormap("hot", HOT);
	//  ocean (green-blue-white) as per the gnuplot documentation
	ax.create_colormap("ocean", PaletteType::Formula(23, 28, 3));

	let color_values: Vec<f64> = row_index.iter().map(|v| *v as f64).collect();

	ax.set_title(
		&format!("variable color boxerror, points, xyerrorbars, and boxxyerror.\nColor used is a SavedColormap"),
		&[],
	);
	ax.set_y_range(Fix(-4.0), Fix(11.5));
	ax.box_error_low_high_set_width(
		&d1,
		&d5,
		&d2,
		&d6,
		iter::repeat(0.2),
		&[
			Color(SavedColorMap("magma", color_values.clone())),
			FillAlpha(0.5),
			BorderColor(RGBColor("black")),
		],
	);
	ax.points(
		&d1,
		iter::repeat(1),
		&[
			Color(SavedColorMap(
				"hot",
				color_values.iter().map(|v| 11.0 - *v).collect(),
			)),
			PointSymbol('D'),
		],
	);
	ax.xy_error_bars(
		&d1,
		iter::repeat(8),
		d1.iter().map(by3),
		d1.iter().map(by4),
		&[Color(SavedColorMap("ocean", color_values.clone()))],
	);
	ax.points(
		&d1,
		d2.iter().map(|v| -v / 2.0),
		&[
			Color(SavedColorMap("magma", color_values.clone())),
			PointSymbol('O'),
			PointSize(3.0),
		],
	);
	ax.box_xy_error_delta(
		&d1,
		iter::repeat(10),
		d1.iter().map(by3),
		d1.iter().map(by4),
		&[
			Color(SavedColorMap("hot", color_values.clone())),
			BorderColor(RGBColor("black")),
		],
	);

	c.show(&mut fg, "variable_palette");
}

fn main() {
	Common::new().map(|c| example(c));
}
