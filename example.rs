extern mod gnuplot;
use std::iterator::*;

use gnuplot::*;

fn main()
{
	let arr = [0, 1, 2, 3, 4, 5];
	let x = arr.iter();
	let y1 = x.transform(|&v| { v*v });
	let y2 = x.transform(|&v| { -v*v });
	
	let mut fg1 = Figure::new();
	{
		let ax = fg1.axes2d();
		
		ax.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);
		ax.points(x, y2, [Caption("Points"), PointSymbol('S'), Color("#ffaa77")]);
		ax.set_x_label("X Label");
		ax.set_y_label("Y Label");
		ax.set_title("Simple Plot");
	}
	fg1.show();
	fg1.echo_to_file("fg1.gnuplot");
	
	let mut fg2 = Figure::new();
	{
		let ax = fg2.axes2d();
		ax.set_pos_grid(1, 1);
		ax.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")]);
		ax.set_title("Plot1");
	}
	{	
		let ax = fg2.axes2d();
		ax.set_pos_grid(1, 2);
		ax.points(x, y2, [Caption("Points"), PointSymbol('S'), Color("#ffaa77")]);
		ax.set_title("Plot2");
	}
	fg2.set_grid(1, 2);
	fg2.show();
	fg2.echo_to_file("fg2.gnuplot");
}
