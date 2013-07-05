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
