// This file is released into Public Domain.
#[feature(globs)];

extern mod gnuplot;
extern mod extra;

use extra::getopts::groups::*;
use std::iter::{Repeat, range_step};
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

	let x = [1, 2, 3, 4, 5];
	let x = x.iter();
	let y1 = x.map(|&v| { v * v }).to_owned_vec();
	let y1 = y1.iter();
	
	let x2 = [1, 4, 5];
	let x2 = x2.iter();
	let y2 = x2.map(|&v| { v * v }).to_owned_vec();
	let y2 = y2.iter();
	let w = Repeat::new(0.5);
	
	let x3 = [-3, -2, -1, 0, 2, 3];
	let x3 = x3.iter();
	let y3 = x3.map(|&v| { v * v * v }).to_owned_vec();
	let y3 = y3.iter();
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Arrows", [])
	.lines(x, y1, [LineWidth(3.0), Color("brown"), LineStyle(DotDash)])
	.arrow(Graph(0.5), Graph(1.0), Axis(1.0), Axis(1.0), [ArrowType(Filled), ArrowSize(0.1), LineStyle(DotDotDash), LineWidth(2.0), Color("red")])
	.arrow(Graph(0.5), Graph(1.0), Axis(3.0), Axis(9.0), [ArrowType(Open), Color("green")]);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg8.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Boxes", [])
	.boxes(x2, y2, [LineWidth(2.0), Color("cyan"), BorderColor("blue"), LineStyle(DotDash)])
	.boxes_set_width(x, y1, w, [LineWidth(2.0), Color("gray"), BorderColor("black")]);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg9.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Axis Ticks", [])
	.lines(x3, y3, [LineWidth(2.0), Color("blue")])
	.set_x_ticks_custom(range_step(0, 10, 2).map(|x| Major(x as f32, Fix(~"%.2f ms"))).chain(range_step(1, 10, 2).map(|x| Minor(x as f32))).chain(Some(Major(-2.1f32, Fix(~"%.2f ms"))).move_iter()), 
	                    [MajorScale(2.0), MinorScale(0.5), OnAxis(true)], [TextColor("blue"), TextAlign(AlignCenter)])
	.set_y_ticks(Fix(2.0), 2, [Mirror(false)], []);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg10.gnuplot");
	
	let mut fg = Figure::new();
	
	fg.axes2d()
	.set_title("Border, Axes", [])
	.set_border(true, [Left, Bottom], [LineWidth(2.0)])
	.set_x_ticks(Fix(1.0), 2, [Mirror(false)], [])
	.set_y_ticks(Fix(5.0), 0, [Mirror(false)], [])
	.lines(x3, y3, [LineWidth(2.0), Color("blue")])
	.set_x_axis(true, [LineWidth(2.0), LineStyle(DotDotDash)])
	.set_y_axis(true, [LineWidth(2.0), Color("red")]);
	
	if show
	{
		fg.show();
	}
	fg.echo_to_file("fg11.gnuplot");
}
