extern mod gnuplot;

use gnuplot::*;

fn main()
{
	let arr = [0, 1, 2, 3, 4, 5];
	let x = arr.iter();
	let y1 = x.transform(|&v| { v * v });
	let y2 = x.transform(|&v| { -v * v + 10 });
	let y3 = x.transform(|&v| { -2 * v + 5 });
	let x_err = RepeatIterator{ value: 0.3 };
	let y_err = RepeatIterator{ value: 5.0 };
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_pos(0.1, 0.1)
	.set_size(0.8, 0.8)
	.lines(x, y1, [Caption("x^2"), LineWidth(3.0), Color("violet"), LineType(DotDash)])
	.points(x, y2, [Caption("-x^2"), PointSymbol('S'), Color("#ffaa77")])
	.lines_points(x, y3, [Caption("-2 x"), PointSymbol('O'), Color("black"), LineType(SmallDot)])
	.set_x_label("X Label", [Font("Arial", 24.0), TextColor("red"), Rotate(45.0)])
	.set_y_label("Y Label", [Rotate(0.0)])
	.set_title("Simple Plot", [Font("Times", 24.0), Position(-10.0, 0.5)]);
	
	fg.show();
	fg.echo_to_file("fg1.gnuplot");
	
	fg.set_terminal("pdfcairo", "fg1.pdf");
	fg.show();
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_pos_grid(1, 1)
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")])
	.set_title("Plot1", []);
	
	fg.axes2d()
	.set_pos_grid(1, 2)
	.points(x, y2, [Caption("Points"), PointSymbol('D'), Color("#ffaa77")])
	.set_title("Plot2", []);
	
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
	.set_title("Inset", []);

	fg.show();
	fg.echo_to_file("fg3.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.lines(x, y1, [Caption("Lines"), LineWidth(3.0), Color("violet")])
	.set_y_range(Fix(-30.0), Auto)
	.set_y_label("This axis is manually scaled on the low end", []);

	fg.show();
	fg.echo_to_file("fg4.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.x_error_lines(x, y1, x_err, [LineWidth(2.0), PointSymbol('O'), Color("red")])
	.y_error_lines(x, y2, y_err, [LineWidth(2.0), PointSymbol('S'), Color("blue")])
	.set_title("Errors", []);

	fg.show();
	fg.echo_to_file("fg5.gnuplot");
	
	let mut fg = Figure::new();

	fg.axes2d()
	.fill_between(x, y1, y3, [Color("red"), FillAlpha(0.5), FillRegion(Above), Caption("A > B")])
	.fill_between(x, y1, y3, [Color("green"), FillAlpha(0.5), FillRegion(Below), Caption("A < B")])
	.fill_between(x, y2, y3, [Color("blue"), FillAlpha(0.5), FillRegion(Closed), Caption("Between C and B")])
	.lines(x, y1, [Color("black"), LineWidth(2.0), LineType(Dash), Caption("A")])
	.lines(x, y2, [Color("black"), LineWidth(2.0), Caption("C")])
	.lines(x, y3, [Color("black"), LineWidth(2.0), LineType(DotDotDash), Caption("B")])
	.set_title("Fill", []);

	fg.show();
	fg.echo_to_file("fg6.gnuplot");
}

struct RepeatIterator<T>
{
	value : T
}

impl<T : Clone> Iterator<T> for RepeatIterator<T>
{
	fn next(&mut self) -> Option<T>
	{
		Some(self.value.clone())
	}
}
