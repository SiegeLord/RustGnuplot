// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let coords = [[0., 0.], [1., 1.], [2., 0.5], [2., -0.5], [1., -1.]];

	let mut fg = Figure::new();
	fg.set_title("Polygons");

	let ax = fg.axes2d();
	ax.polygon(
		coords.iter().map(|x| x[0]),
		coords.iter().map(|x| x[1]),
		&[],
	);

	ax.polygon(
		coords.iter().map(|x| x[0] + 2.),
		coords.iter().map(|x| x[1]),
		&[FillAlpha(0.), BorderColor("black"), LineWidth(4.)],
	);
	ax.polygon(
		coords.iter().map(|x| x[0]),
		coords.iter().map(|x| x[1] + 2.),
		&[Color("#FF0000"), BorderColor("black"), LineWidth(4.)],
	);
	ax.polygon(
		coords.iter().map(|x| x[0] + 2.),
		coords.iter().map(|x| x[1] + 2.),
		&[
			FillPattern(Fix(BigCrosses)),
			Color("#FF0000"),
			BorderColor("black"),
			LineWidth(4.),
			LineStyle(Dash),
		],
	);
	c.show(&mut fg, "polygons");
}

fn main()
{
	Common::new().map(|c| example(c));
}
