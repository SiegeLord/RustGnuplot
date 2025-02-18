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
		Color(ColorType::RGBColor("#ff0000")),   // red using Hex coded RRGGBB
		Color(ColorType::RGBColor("#ff8888")),   // pink using Hex coded RRGGBB
		Color(ColorType::RGBColor("#88ff0000")), // pink using Hex coded AARRGGBB
		Color(ColorType::RGBColor("#ff0000")),   // red using Hex coded RRGGBB
	];

	for (i, color) in colors.into_iter().enumerate() {
		ax.lines_points(
			x.clone(),
			x.clone().map(|v| v * 2 + i),
			&[Caption(&format!("{}: {:?}", i, color)), color],
		);
	}

	c.show(&mut fg, "rgb_color");


	let data:Vec<Vec<f64>> = CANDLESTICKS_STR.split("\n").map(|line| line.split("\t").map(|v| v.trim().parse::<f64>().unwrap()).collect()).collect();
	let extract_col = |i| data.iter().map(|l| l[i]).collect::<Vec<_>>();

	let d1 = extract_col(0);
	let d2 = extract_col(1);
	let d3 = extract_col(2);
	// let d4 = extract_col(3);
	let d5 = extract_col(4);
	let d6 = extract_col(5);
	let row_index:Vec<_>  = (1..=d1.len() as u8).collect();

	// Demo/test of variable color in many different plot styles
	// derived from plot 1 in https://gnuplot.sourceforge.net/demo_6.0/varcolor.html
	// this demonstrates usage of VariableIndexColor with indcies to use gnuplots dfefalt color styles,
	// but make them align for multiple plot items on the same axis.
	// i.e. evertything at x = 1 uses the first gnuplot color, everything at x = 2 uses the second and so on.
	let mut fg = Figure::new();
	let ax = fg.axes2d();
	ax.set_title("variable color points, candlesticks, boxes, and boxxyerror", &[]);
	ax.set_y_range(Fix(-4.0), Fix(10.0));
	ax.set_x_range(Fix(0.0), Fix(11.0));

	let by3 = |x| (((x%3.0)+1.0)/6.0);
	let by4 = |x| (((x%4.0)+1.0)/7.0);

	// let ax.financebars(...)
	// points replaces circles (as circles not implemented)
	ax.points(&d1, &d2, &[Color(VariableIndexColor(row_index.clone())), PointSymbol('O'), PointSize(3.0)]);
	ax.box_xy_error_delta(
		&d1,
		iter::repeat(8),
		d1.iter().map(by3),
		d1.iter().map(by4),
		&[Color(VariableIndexColor(row_index.clone()))]);
	ax.boxes_set_width(
		&d1,
		d2.iter().map(|v| -v/2.0),
		iter::repeat(0.2),
		&[Color(VariableIndexColor(row_index.clone()))]);
	// box_and_whisker is rust gnuplot's name for candlestick
	ax.box_and_whisker_set_width(
		&d1,
		d3,
		d2,
		d6,
		d5,
		iter::repeat(0.3),
		&[Color(VariableIndexColor(row_index.clone()))]);
	// set boxwidth 0.2 abs
	// set bars front
	// rgbfudge(x) = x*51*32768 + (11-x)*51*128 + int(abs(5.5-x)*510/9.)
	c.show(&mut fg, "variable_color");

}

fn main() {
	Common::new().map(|c| example(c));
}
