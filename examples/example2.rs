// This file is released into Public Domain.
#![feature(globs)]

extern crate gnuplot;
extern crate getopts;

use getopts::*;
use std::iter::{Repeat, range_step};
use std::os;

use gnuplot::*;

fn example(show: |fg: &mut Figure, filename: &str|, set_term: |fg: &mut Figure|)
{
	let x = [1i32, 2, 3, 4, 5];
	let x = x.iter();
	let y1: Vec<i32> = x.map(|&v| { v * v }).collect();
	let y1 = y1.iter();
	
	let x2 = [1i32, 4, 5];
	let x2 = x2.iter();
	let y2: Vec<i32> = x2.map(|&v| { v * v }).collect();
	let y2 = y2.iter();
	let w = Repeat::new(0.5);
	
	let x3 = [-3i32, -2, -1, 0, 2, 3];
	let x3 = x3.iter();
	let y3: Vec<i32> = x3.map(|&v| { v * v * v }).collect();
	let y3 = y3.iter();
	
	let zw = 16u;
	let zh = 16u;
	let mut z1 = Vec::with_capacity((zw * zh) as uint);
	for i in range(0, zh)
	{
		for j in range(0, zw)
		{
			let y = 8.0 * (i as f64) / zh as f64 - 4.0;
			let x = 8.0 * (j as f64) / zw as f64 - 4.0;
			z1.push(x + y);
		}
	}
	
	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes2d()
	.set_title("Arrows", [])
	.lines(x, y1, [LineWidth(3.0), Color("brown"), LineStyle(DotDash)])
	.arrow(Graph(0.5), Graph(1.0), Axis(1.0), Axis(1.0), [ArrowType(Filled), ArrowSize(0.1), LineStyle(DotDotDash), LineWidth(2.0), Color("red")])
	.arrow(Graph(0.5), Graph(1.0), Axis(3.0), Axis(9.0), [ArrowType(Open), Color("green")]);
	
	show(&mut fg, "fg2.1.gnuplot");
	
	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes2d()
	.set_title("Boxes", [])
	.boxes(x2, y2, [LineWidth(2.0), Color("cyan"), BorderColor("blue"), LineStyle(DotDash)])
	.boxes_set_width(x, y1, w, [LineWidth(2.0), Color("gray"), BorderColor("black")]);
	
	show(&mut fg, "fg2.2.gnuplot");
	
	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes2d()
	.set_title("Axis Ticks", [])
	.lines(x3, y3, [LineWidth(2.0), Color("blue")])
	.set_x_ticks_custom(range_step(0, 10, 2).map(|x| Major(x as f32, Fix("%.2f ms".to_owned())))
	                    .chain(range_step(1, 10, 2).map(|x| Minor(x as f32))).chain(Some(Major(-2.1f32, Fix("%.2f ms".to_owned()))).move_iter()), 
						[MajorScale(2.0), MinorScale(0.5), OnAxis(true)], [TextColor("blue"), TextAlign(AlignCenter)])
	.set_y_ticks(Fix(2.0), 2, [Mirror(false)], []);
	
	show(&mut fg, "fg2.3.gnuplot");
	
	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes2d()
	.set_title("Border, Axes", [])
	.set_border(true, [Left, Bottom], [LineWidth(2.0)])
	.set_x_ticks(Fix(1.0), 2, [Mirror(false)], [])
	.set_y_ticks(Fix(5.0), 0, [Mirror(false)], [])
	.lines(x3, y3, [LineWidth(2.0), Color("blue")])
	.set_x_axis(true, [LineWidth(2.0), LineStyle(DotDotDash)])
	.set_y_axis(true, [LineWidth(2.0), Color("red")]);
	
	show(&mut fg, "fg2.4.gnuplot");
	
	let mut fg = Figure::new();
	set_term(&mut fg);
	
	fg.axes2d()
	.set_title("Image", [])
	.image(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), []);
	
	show(&mut fg, "fg2.5.gnuplot");
}

fn main()
{
	let args: Vec<_> = os::args().iter().map(|s| s.to_strbuf()).collect();
	
	let opts = 
	[
		optflag("n", "no-show", "do not run the gnuplot process."),
		optflag("h", "help", "show this help and exit."),
		optopt("t", "terminal", "specify what terminal to use for gnuplot.", "TERM")
	];
	
	let matches = match getopts(args.tail(), opts)
	{
		Ok(m) => m,
		Err(f) => fail!("{}", f)
	};
	if matches.opt_present("h")
	{
		println!("{}", usage("A RustGnuplot example.", opts));
		return;
	}
	
	example(
	|fg, filename|
	{
		if !matches.opt_present("n")
		{
			fg.show();
		}
		fg.echo_to_file(filename);
	},
	|fg|
	{
		matches.opt_str("t").map(|t|
		{
			fg.set_terminal(t.as_slice(), "");
		});
	});
}
