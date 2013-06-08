extern mod gnuplot;
use std::iterator::*;

use gnuplot::*;

fn main()
{
	let x = [0, 1, 2, 3, 4, 5];
	let mut fg = Figure::new();
	{
		let ax = fg.axes2d();
		
		ax.lines(x.iter(), x.iter().transform(|&v| { v*v }), [Caption("Lines")]);
		ax.points(x.iter(), x.iter().transform(|&v| { -v*v }), [Caption("Points")]);
	}
	fg.show();
	fg.echo_to_file("out.gnuplot");
}
