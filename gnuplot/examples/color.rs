use std::iter;

// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

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
	let x = 0..10;

	let mut fg = Figure::new();
	let ax = fg.axes2d();
	ax.set_title("Demo of RGBColor in various forms", &[]);
	ax.set_legend(Graph(0.5), Graph(0.9), &[], &[]);
	let colors = [
		Color("black"),
		Color(ColorType::RGBColor("black")),
		Color("red"),
		Color(ColorType::RGBColor("#ff0000")), // red using Hex coded RRGGBB
		Color(ColorType::RGBColor("#ff8888")), // pink using Hex coded RRGGBB
		Color(ColorType::RGBColor("#88ff0000")), // pink using Hex coded AARRGGBB
		Color(ColorType::RGBColor("#ff0000")), // red using Hex coded RRGGBB
	];

	for (i, color) in colors.into_iter().enumerate() {
		ax.lines_points(
			x.clone(),
			x.clone().map(|v| v * 2 + i),
			&[Caption(&format!("{}: {:?}", i, color)), color],
		);
	}

	c.show(&mut fg, "rgb_color");

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

	// Demo/test of variable color in many different plot styles
	// derived from https://gnuplot.sourceforge.net/demo_6.0/varcolor.html
	//
	// The loop is run four times with different color sets: each one sets all the elements of a given
	// plot to a different color, while making sure the colors align by x position: i.e. everything at x = 1
	// uses the first color, everything at x = 2 uses the second and so on.
	//
	//
	// The first color loop demonstrates usage of VariableIndexColor with indices to use gnuplot's default color styles,
	// but make them align for multiple plot items on the same axis. This is implicity constructed from a Vec<u8> using
	// the `Color` function but could equivalently be created explicitly using `ColorType::VariableIndexColor(row_index.clone())`
	//
	// The second color loop use a VariablePaletteColor: this selects the color based on the current color palette and the
	// input value for each data point. The palette is scaled to the maximum value in the vector of `f64`s passed
	// to the VariablePaletteColor
	for (color, label) in [
		(Color(row_index.clone()), "VariableIndexColor"),
		(
			ColorOpt(VariablePaletteColor(row_index.iter().map(|v| *v as f64).collect())),
			"VariablePaletteColor",
		),
	] {
		let mut fg = Figure::new();
		let ax = fg.axes2d();
		ax.set_title(
			&format!("variable color boxerror, points, xyerrorbars, and boxxyerror.\nColor used is a {label}"),
			&[],
		);
		ax.set_y_range(Fix(-4.0), Fix(11.5));
		// unset colorbox
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
			&[color.clone()],
		);

		c.show(&mut fg, "variable_color");
	}
}

fn main() {
	Common::new().map(|c| example(c));
}
