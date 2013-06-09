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
	{
		let ax = fg.axes2d();
		
		ax.lines(x, y1, [Caption("x^2"), LineWidth(3.0), Color("violet"), LineDash(DotDash)]);
		ax.points(x, y2, [Caption("-x^2"), PointSymbol('S'), Color("#ffaa77")]);
		ax.set_x_label("X Label");
		ax.set_y_label("Y Label");
		ax.set_title("Simple Plot");
	}
	fg.show();
	fg.echo_to_file("fg1.gnuplot");
	
	fg.set_terminal("pdfcairo", "fg1.pdf");
	fg.show();
	
	let mut fg = Figure::new();
	{
		let ax = fg.axes2d();
		ax.set_pos_grid(1, 1);
		ax.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);
		ax.set_title("Plot1");
	}
	{	
		let ax = fg.axes2d();
		ax.set_pos_grid(1, 2);
		ax.points(x, y2, [Caption("Points"), PointSymbol('D'), Color("#ffaa77")]);
		ax.set_title("Plot2");
	}
	fg.set_grid(1, 2);
	fg.show();
	fg.echo_to_file("fg2.gnuplot");
	
	let mut fg = Figure::new();
	{
		let ax = fg.axes2d();
		ax.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);
	}
	{	
		let ax = fg.axes2d();
		ax.set_pos(0.1, 0.4);
		ax.set_size(0.3, 0.6);
		ax.set_aspect_ratio(Fix(1.0));
		ax.points(x, y2, [Caption("Points"), PointSymbol('T'), Color("#ffaa77")]);
		ax.set_title("Inset");
	}
	fg.show();
	fg.echo_to_file("fg3.gnuplot");
	
	let mut fg = Figure::new();
	{
		let ax = fg.axes2d();
		ax.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);
		ax.set_y_range(Fix(-30.0), Auto);
		ax.set_y_label("This axis is manually scaled on the low end");
	}
	fg.show();
	fg.echo_to_file("fg4.gnuplot");
}
