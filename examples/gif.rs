// This file is released into Public Domain.
use gnuplot::*;
use std::f32;

fn main()
{
	let mut fg = Figure::new();
	let mut x = vec![];
	for i in 0..100i32
	{
		x.push(i as f32 * 0.1 - 5.0);
	}

	let mut t = 0.0;
	fg.set_terminal("gif animate optimize delay 2 size 480,360", "fg.gif.gif");
	for i in 0..100
	{
		if i > 0
		{
			fg.new_page();
		}
		fg.axes2d().set_y_range(Fix(-1.0), Fix(1.0)).lines(
			x.iter(),
			x.iter().map(|&x| (x + t as f32 * 0.1 * 2. * f32::consts::PI).sin()),
			&[],
		);
		fg.axes2d().set_y_range(Fix(-1.0), Fix(1.0)).lines(
			x.iter(),
			x.iter().map(|&x| (x + t as f32 * 0.1 * 2. * f32::consts::PI).cos()),
			&[Color("red")],
		);
		t += 0.1;
	}
	fg.echo_to_file("fg.gif.gnuplot");
	fg.show();
}
