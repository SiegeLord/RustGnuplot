// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common) {
	let x = 0..10;

	let mut fg = Figure::new();

	let ax = fg.axes2d();
	ax.set_title("Color cycling", &[]);
	ax.set_legend(Graph(0.5), Graph(0.9), &[], &[]);
	let colors = [
		Color("black"),
		ColorOpt(ColorType::RGBColor("black")),
		Color("red"),
		ColorOpt(ColorType::RGBColor("#ff0000")),   // red using Hex coded RRGGBB
		ColorOpt(ColorType::RGBColor("#ff8888")),   // pink using Hex coded RRGGBB
		ColorOpt(ColorType::RGBColor("#88ff0000")), // pink using Hex coded AARRGGBB
		ColorOpt(ColorType::RGBColor("#ff0000")),   // red using Hex coded RRGGBB
	];

	for (i, color) in colors.into_iter().enumerate() {
		ax.lines_points(
			x.clone(),
			x.clone().map(|v| v * 2 + i),
			&[Caption(&format!("{}: {:?}", i, color)), color],
		);
	}

	c.show(&mut fg, "color_cycling");

	// let mut fg = Figure::new();

	// fg.axes2d()
	// 	.set_title("Box XY Error", &[])
	// 	.box_xy_error_delta(
	// 		[0.0f32, 1.0, 2.0].iter(),
	// 		[-1.0f32, 0.0, 1.0].iter(),
	// 		[0.25f32, 0.375, 0.15].iter(),
	// 		[2.0f32, 3.0, 4.0].iter(),
	// 		&[],
	// 	)
	// 	.box_xy_error_low_high(
	// 		[-0.6f32, 1.5, 2.5].iter(),
	// 		[-1.0f32, 0.0, 1.0].iter(),
	// 		[-0.9f32, -1.0, 2.2].iter(),
	// 		[-0.45f32, 3.0, 2.95].iter(),
	// 		[-1.5f32, 4.5, 3.0].iter(),
	// 		[0.5f32, 4.75, 0.125].iter(),
	// 		&[
	// 			Color("blue"),
	// 			LineWidth(2.0),
	// 			LineStyle(SmallDot),
	// 			FillAlpha(0.5),
	// 		],
	// 	)
	// 	.set_x_range(Fix(-1.0), Fix(3.0))
	// 	.set_y_range(Fix(-3.0), Fix(5.0));

	// c.show(&mut fg, "box_xy_error");
}

fn main() {
	Common::new().map(|c| example(c));
}
