// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

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

}

fn main() {
	Common::new().map(|c| example(c));
}
