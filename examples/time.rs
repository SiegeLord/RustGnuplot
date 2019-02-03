// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;
use std::time::Duration;

mod common;

fn example(c: Common)
{
	let x1 = &[
		Duration::from_secs(0),
		Duration::from_secs(3600 * 12),
		Duration::from_secs(3600 * 24),
		Duration::from_secs(3600 * 36),
	];
	let x2 = &[
		Duration::from_millis(0),
		Duration::from_millis(500),
		Duration::from_millis(1000),
		Duration::from_millis(1500),
	];
	let y = &[0i32, -1, 1, 0];

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Time 1: Hours", &[])
		.lines(x1, y, &[])
		.set_x_ticks(Some((Auto, 1)), &[Format("%H hours")], &[])
		.set_x_time(true);

	c.show(&mut fg, "fg.time.1.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);
	fg.axes2d()
		.set_title("Time 2: Seconds", &[])
		.lines(x2, y, &[])
		.set_x_ticks(Some((Auto, 1)), &[Format("%.1S secs")], &[])
		.set_x_time(true);

	c.show(&mut fg, "fg.time.2.gnuplot");
}

fn main()
{
	Common::new().map(|c| example(c));
}
