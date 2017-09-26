// This file is released into Public Domain.
extern crate gnuplot;

use common::*;

use gnuplot::*;
use std::iter::repeat;

mod common;

fn example(c: Common)
{
	let x = &[1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
	let x = x.iter2();
	let y1: Vec<f32> = x.map(|&v| {
		let z = v - 4.0;
		z * z - 5.0
	}).collect();
	let y1 = y1.iter2();
	let y2: Vec<f32> = x.map(|&v| {
		let z = v - 4.0;
		-z * z + 5.0
	}).collect();
	let y2 = y2.iter2();
	let y3: Vec<f32> = x.map(|&v| v - 4.0).collect();
	let y3 = y3.iter2();
	let y4: Vec<f32> = x.map(|&v| 0.9 * v - 4.0).collect();
	let y4 = y4.iter2();
	let x_err = repeat(0.1f32);
	let y_err = repeat(0.2f32);

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_size(0.75, 1.0)
		.set_title("Example Plot fg1.1", &[])
		.set_x_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
		.set_y_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
		.set_legend(Graph(1.0), Graph(0.5), &[Placement(AlignLeft, AlignCenter)], &[TextAlign(AlignRight)])
		.set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
		.set_x_label("Abscissa", &[])
		.set_y_label("Ordinate", &[])
		.arrow(
			Axis(5.7912),
			Axis(2.7912),
			Axis(5.7912),
			Axis(1.7912),
			&[ArrowType(Closed), ArrowSize(0.1), LineWidth(2.0), Color("black")],
		)
		.label("Here", Axis(5.7912), Axis(3.1), &[TextAlign(AlignCenter)])
		.fill_between(x, y1.map(|&y| y * 0.85 - 1.0), y1.map(|&y| y * 1.15 + 1.0), &[Color("#aaaaff")])
		.lines(x, y1, &[Caption("(x - 4)^2 - 5"), LineWidth(1.5), Color("black")])
		.y_error_lines(x, y2, repeat(1.0f32), &[Caption("(x - 4)^2 + 5"), LineWidth(1.5), Color("red")])
		.lines_points(x, y3, &[Caption("x - 4"), PointSymbol('t'), LineWidth(1.5), LineStyle(Dash), Color("#11ff11")]);

	c.show(&mut fg, "fg1.1.gnuplot");

	if !c.no_show
	{
		fg.set_terminal("pdfcairo", "fg1.1.pdf");
		fg.show();
		fg.set_terminal("pngcairo", "fg1.1.png");
		fg.show();
	}

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_pos_grid(2, 2, 0)
		.lines(x, y1, &[Caption("Lines"), LineWidth(3.0), Color("violet")])
		.set_title("Plot1 fg1.2", &[]);

	fg.axes2d()
		.set_pos_grid(2, 1, 1)
		.points(x, y2, &[Caption("Points"), PointSymbol('D'), Color("#ffaa77"), PointSize(2.0)])
		.set_title("Plot2", &[]);

	c.show(&mut fg, "fg1.2.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d().lines(x, y1, &[Caption("Lines"), LineWidth(3.0), Color("violet")]);

	fg.axes2d()
		.set_pos(0.2, 0.4)
		.set_size(0.3, 0.6)
		.set_aspect_ratio(Fix(1.0))
		.points(x, y2, &[Caption("Points"), PointSymbol('T'), Color("#ffaa77")])
		.set_title("Inset fg1.3", &[]);

	c.show(&mut fg, "fg1.3.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.lines(x, y1, &[Caption("Lines"), LineWidth(3.0), Color("violet")])
		.set_y_range(Fix(-30.0), Auto)
		.set_y_label("This axis is manually scaled on the low end", &[])
		.set_title("Range fg1.4", &[]);

	c.show(&mut fg, "fg1.4.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.x_error_lines(
			x,
			y1,
			x_err.clone(),
			&[Caption(r"x\\_error\\_lines"), LineWidth(2.0), PointSymbol('O'), Color("red")],
		)
		.y_error_lines(
			x,
			y2,
			y_err.clone(),
			&[Caption(r"y\\_error\\_lines"), LineWidth(2.0), PointSymbol('S'), Color("blue")],
		)
		.x_error_bars(x, y3, x_err, &[Caption(r"x\\_error\\_bars"), PointSymbol('T'), Color("cyan")])
		.y_error_bars(x, y4, y_err, &[Caption(r"y\\_error\\_bars"), PointSymbol('R'), Color("green")])
		.set_title("Error fg1.5", &[]);

	c.show(&mut fg, "fg1.5.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_size(1.0, 0.8)
		.set_pos(0.0, 0.2)
		.fill_between(x, y1, y3, &[Color("red"), FillAlpha(0.5), FillRegion(Above), Caption("A > B")])
		.fill_between(x, y1, y3, &[Color("green"), FillAlpha(0.5), FillRegion(Below), Caption("A < B")])
		.fill_between(x, y2, y3, &[Color("blue"), FillAlpha(0.5), FillRegion(Between), Caption("Between C and B")])
		.lines(x, y1, &[Color("black"), LineWidth(2.0), LineStyle(Dash), Caption("A")])
		.lines(x, y2, &[Color("black"), LineWidth(2.0), Caption("C")])
		.lines(x, y3, &[Color("black"), LineWidth(2.0), LineStyle(DotDotDash), Caption("B")])
		.set_title("Fill and legend fg1.6", &[])
		.set_legend(
			Graph(0.5),
			Graph(-0.2),
			&[Horizontal, Placement(AlignCenter, AlignTop), Title("Legend Title")],
			&[TextAlign(AlignRight)],
		);

	c.show(&mut fg, "fg1.6.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_pos(0.1, 0.1)
		.set_size(0.8, 0.8)
		.lines(x, y1, &[Caption("(x - 4)^2 - 5"), LineWidth(3.0), Color("violet"), LineStyle(DotDash)])
		.points(x, y2, &[Caption("(x - 4)^2 + 5"), PointSymbol('S'), Color("#ffaa77")])
		.lines_points(x, y3, &[Caption("x - 4"), PointSymbol('O'), Color("black"), LineStyle(SmallDot)])
		.set_x_label("X Label", &[Font("Arial", 24.0), TextColor("red"), Rotate(45.0)])
		.set_y_label("Y Label", &[Rotate(0.0)])
		.set_title("Goings nuts with the formatting fg1.7", &[Font("Times", 24.0), TextOffset(-10.0, 0.5)])
		.label(
			"Intersection",
			Axis(2.208),
			Axis(-1.791),
			&[
				MarkerSymbol('*'),
				TextAlign(AlignCenter),
				TextOffset(0.0, -1.0),
				MarkerColor("red"),
				MarkerSize(2.0),
			],
		);

	c.show(&mut fg, "fg1.7.gnuplot");
}

fn main()
{
	Common::new().map(|c| example(c));
}
