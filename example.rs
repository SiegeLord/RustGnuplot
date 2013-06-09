extern mod gnuplot;
use std::iterator::*;

use gnuplot::*;

fn main()
{
	let arr = [0, 1, 2, 3, 4, 5];
	let x = arr.iter();
	let y1 = x.transform(|&v| { v*v });
	let y2 = x.transform(|&v| { -v*v });
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.lines(x, y1, [Caption("x^2"), LineWidth(3.0), Color("violet"), LineDash(DotDash)])
	.points(x, y2, [Caption("-x^2"), PointSymbol('S'), Color("#ffaa77")])
	.set_x_label("X Label")
	.set_y_label("Y Label")
	.set_title("Simple Plot");
	
	fg.show();
	fg.echo_to_file("fg1.gnuplot");
	
	fg.set_terminal("pdfcairo", "fg1.pdf");
	fg.show();
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_pos_grid(1, 1)
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")])
	.set_title("Plot1");
	
	fg.axes2d()
	.set_pos_grid(1, 2)
	.points(x, y2, [Caption("Points"), PointSymbol('D'), Color("#ffaa77")])
	.set_title("Plot2");
	
	fg.set_grid(1, 2);
	fg.show();
	fg.echo_to_file("fg2.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);

	fg.axes2d()
	.set_pos(0.1, 0.4)
	.set_size(0.3, 0.6)
	.set_aspect_ratio(Fix(1.0))
	.points(x, y2, [Caption("Points"), PointSymbol('T'), Color("#ffaa77")])
	.set_title("Inset");

	fg.show();
	fg.echo_to_file("fg3.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")])
	.set_y_range(Fix(-30.0), Auto)
	.set_y_label("This axis is manually scaled on the low end");

	fg.show();
	fg.echo_to_file("fg4.gnuplot");
}
