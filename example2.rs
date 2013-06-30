extern mod gnuplot;

use gnuplot::*;

fn main()
{
	let arr = [0, 1, 2, 3, 4, 5];
	let x = arr.iter();
	let y1 = x.transform(|&v| { v * v });
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Arrows", [])
	.lines(x, y1, [LineWidth(3.0), Color("brown"), LineType(DotDash)])
	.arrow(Graph(0.5), Graph(1.0), Axis(1.0), Axis(1.0), [HeadType(Filled), HeadSize(0.1), ShaftType(DotDotDash), ShaftWidth(2.0), ArrowColor("red")])
	.arrow(Graph(0.5), Graph(1.0), Axis(3.0), Axis(9.0), [HeadType(Open), ArrowColor("green")]);
	
	fg.show();
	fg.echo_to_file("fg7.gnuplot");
}
