// This file is released into Public Domain.
#![feature(unboxed_closures)]
#![allow(unstable)]

extern crate gnuplot;

use std::io::timer::sleep;
use std::num::Float;
use std::time::duration::Duration;

use gnuplot::*;

fn main()
{
	println!("This is a silly example of doing an animation... Ctrl-C to quit.");
	let mut fg = Figure::new();
	let mut x = vec![];
	for i in 0..100
	{
		x.push(i as f32 * 0.1 - 5.0);
	}

	let mut t = 0.0;
	loop
	{
		fg.clear_axes();
		fg.axes2d()
		.set_y_range(Fix(-1.0), Fix(1.0))
		.lines(x.iter(), x.iter().map(|&x| (x + t).sin()), &[]);
		t += 0.1;
		fg.show();
		sleep(Duration::milliseconds(500));
	}
}
