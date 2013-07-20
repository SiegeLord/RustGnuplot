extern mod gnuplot;

use gnuplot::*;

fn main()
{
	let arr = [1, 2, 3, 4, 5];
	let x = arr.iter();
	let y1 = x.transform(|&v| { v * v });
	
	let arr2 = [1, 4, 5];
	let x2 = arr2.iter();
	let y2 = x2.transform(|&v| { v * v });
	let w = RepeatIterator{ value : 0.5 };
	
	let arr3 = [-3, -2, -1, 0, 2, 3];
	let x3 = arr3.iter();
	let y3 = x3.transform(|&v| { v * v * v });
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Arrows", [])
	.lines(x, y1, [LineWidth(3.0), Color("brown"), LineStyle(DotDash)])
	.arrow(Graph(0.5), Graph(1.0), Axis(1.0), Axis(1.0), [ArrowType(Filled), ArrowSize(0.1), LineStyle(DotDotDash), LineWidth(2.0), Color("red")])
	.arrow(Graph(0.5), Graph(1.0), Axis(3.0), Axis(9.0), [ArrowType(Open), Color("green")]);
	
	fg.show();
	fg.echo_to_file("fg7.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Boxes", [])
	.boxes(x2, y2, [LineWidth(2.0), Color("cyan"), BorderColor("blue"), LineStyle(DotDash)])
	.boxes_set_width(x, y1, w, [LineWidth(2.0), Color("gray"), BorderColor("black")]);
	
	fg.show();
	fg.echo_to_file("fg8.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Axis Ticks", [])
	.lines(x3, y3, [LineWidth(2.0), Color("blue")])
	.set_x_tics(Fix(0.0), 2.0, Auto, [MinorIntervals(2), MajorScale(2.0), MinorScale(0.5), OnAxis(true)], [TextColor("blue"), Align(AlignCenter)])
	.set_y_tics(Auto, 2.0, Auto, [Mirror(false)], []);
	
	fg.show();
	fg.echo_to_file("fg9.gnuplot");
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
