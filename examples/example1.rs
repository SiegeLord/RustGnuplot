// This file is released into Public Domain.
#[feature(globs)];

extern mod gnuplot;
extern mod extra;

use extra::getopts::groups::*;
use std::iter::Repeat;
use std::os;

use gnuplot::*;

fn main()
{
	let args = os::args();
	
	let opts = ~[
		optflag("n", "no-show", "do not run the gnuplot process")
	];
	
	let matches = match getopts(args.tail(), opts)
	{
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};
	
	let show = !matches.opt_present("n");

	let x = range(1.0, 6.0);
	let y1 = x.map(|v| { v * v }).to_owned_vec();
	let y1 = y1.iter();
	let y2 = x.map(|v| { -v * v + 10.0 }).to_owned_vec();
	let y2 = y2.iter();
	let y3 = x.map(|v| { -2.0 * v + 5.0 }).to_owned_vec();
	let y3 = y3.iter();
	let x_err = Repeat::new(0.3);
	let y_err = Repeat::new(5.0);
	
	let mut fg = Figure::new();
	fg.set_grid(1, 2);

	fg.axes2d()
	.set_pos_grid(1, 1)
	.fill_between(x, y1.map(|&y| y * 0.85 - 1.0), y1.map(|&y| y * 1.15 + 1.0), [Color("#aaaaff")])
	.lines(x, y1, [Caption("Data 1"), LineWidth(2.0), Color("black")])
	.set_title("Plot1", [])
	.set_aspect_ratio(Fix(1.0))
	.set_x_ticks(Fix(1.0), 2, [Mirror(false)], [])
	.set_y_ticks(Fix(1.0), 2, [Mirror(false)], [])
	.set_border(true, [Left, Bottom], []);
	
	fg.axes2d()
	.set_pos_grid(1, 2)
	.points(x, y2, [Caption("Points"), PointSymbol('D'), Color("#ffaa77"), PointSize(2.0)])
	.set_title("Plot2", []);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.1.gnuplot");
	
	if show
	{
		fg.set_terminal("pdfcairo", "fg1.pdf");
		fg.show();
	}

	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_pos_grid(1, 1)
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")])
	.set_title("Plot1", []);
	
	fg.axes2d()
	.set_pos_grid(1, 2)
	.points(x, y2, [Caption("Points"), PointSymbol('D'), Color("#ffaa77"), PointSize(2.0)])
	.set_title("Plot2", []);
	
	fg.set_grid(1, 2);
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.2.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);

	fg.axes2d()
	.set_pos(0.1, 0.4)
	.set_size(0.3, 0.6)
	.set_aspect_ratio(Fix(1.0))
	.points(x, y2, [Caption("Points"), PointSymbol('T'), Color("#ffaa77")])
	.set_title("Inset", []);

	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.3.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")])
	.set_y_range(Fix(-30.0), Auto)
	.set_y_label("This axis is manually scaled on the low end", []);

	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.4.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.x_error_lines(x, y1, x_err, [LineWidth(2.0), PointSymbol('O'), Color("red")])
	.y_error_lines(x, y2, y_err, [LineWidth(2.0), PointSymbol('S'), Color("blue")])
	.set_title("Errors", []);

	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.5.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.set_size(1.0, 0.8)
	.set_pos(0.0, 0.2)
	.fill_between(x, y1, y3, [Color("red"), FillAlpha(0.5), FillRegion(Above), Caption("A > B")])
	.fill_between(x, y1, y3, [Color("green"), FillAlpha(0.5), FillRegion(Below), Caption("A < B")])
	.fill_between(x, y2, y3, [Color("blue"), FillAlpha(0.5), FillRegion(Between), Caption("Between C and B")])
	.lines(x, y1, [Color("black"), LineWidth(2.0), LineStyle(Dash), Caption("A")])
	.lines(x, y2, [Color("black"), LineWidth(2.0), Caption("C")])
	.lines(x, y3, [Color("black"), LineWidth(2.0), LineStyle(DotDotDash), Caption("B")])
	.set_title("Fill and legend", [])
	.set_legend(Graph(0.5), Graph(-0.2), [Horizontal, Placement(AlignCenter, AlignTop), Title("Legend Title")], [TextAlign(AlignRight)]);

	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.6.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_pos(0.1, 0.1)
	.set_size(0.8, 0.8)
	.lines(x, y1, [Caption("x^2"), LineWidth(3.0), Color("violet"), LineStyle(DotDash)])
	.points(x, y2, [Caption("-x^2 + 10"), PointSymbol('S'), Color("#ffaa77")])
	.lines_points(x, y3, [Caption("-2 x + 5"), PointSymbol('O'), Color("black"), LineStyle(SmallDot)])
	.set_x_label("X Label", [Font("Arial", 24.0), TextColor("red"), Rotate(45.0)])
	.set_y_label("Y Label", [Rotate(0.0)])
	.set_title("Goings nuts with the formatting", [Font("Times", 24.0), TextOffset(-10.0, 0.5)])
	.label("Intersection", Axis(1.449), Axis(2.101), [MarkerSymbol('*'), TextAlign(AlignCenter), TextOffset(0.0, -1.0), MarkerColor("red"), MarkerSize(2.0)]);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg1.7.gnuplot");
}
