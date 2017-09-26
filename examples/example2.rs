// This file is released into Public Domain.
extern crate gnuplot;

use common::*;

use gnuplot::*;
use std::iter::repeat;

mod common;

fn example(c: Common)
{
	let x = &[1i32, 2, 3, 4, 5];
	let x = x.iter2();
	let y1: Vec<i32> = x.map(|&v| v * v).collect();
	let y1 = y1.iter2();

	let x2 = &[1i32, 4, 5];
	let x2 = x2.iter2();
	let y2: Vec<i32> = x2.map(|&v| v * v).collect();
	let y2 = y2.iter2();
	let w = repeat(0.5f32);

	let x3 = &[-3i32, -2, -1, 0, 2, 3];
	let x3 = x3.iter2();
	let y3: Vec<i32> = x3.map(|&v| v * v * v).collect();
	let y3 = y3.iter2();

	let zw = 16;
	let zh = 16;
	let mut z1 = Vec::with_capacity((zw * zh) as usize);
	for i in 0..zh
	{
		for j in 0..zw
		{
			let y = 8.0 * (i as f64) / zh as f64 - 4.0;
			let x = 8.0 * (j as f64) / zw as f64 - 4.0;
			z1.push(x + y);
		}
	}

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Arrows fg2.1", &[])
		.lines(x, y1, &[LineWidth(3.0), Color("brown"), LineStyle(DotDash)])
		.arrow(
			Graph(0.5),
			Graph(1.0),
			Axis(1.0),
			Axis(1.0),
			&[ArrowType(Filled), ArrowSize(0.1), LineStyle(DotDotDash), LineWidth(2.0), Color("red")],
		)
		.arrow(Graph(0.5), Graph(1.0), Axis(3.0), Axis(9.0), &[ArrowType(Open), Color("green")]);

	c.show(&mut fg, "fg2.1.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Boxes fg2.2", &[])
		.boxes(x2, y2, &[LineWidth(2.0), Color("cyan"), BorderColor("blue"), LineStyle(DotDash)])
		.boxes_set_width(x, y1, w, &[LineWidth(2.0), Color("gray"), BorderColor("black")]);

	c.show(&mut fg, "fg2.2.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Axis Ticks fg2.3", &[])
		.lines(x3, y3, &[LineWidth(2.0), Color("blue")])
		.set_x_ticks_custom(
			(0..5)
				.map(|i| 2 * i)
				.map(|x| Major(x as f32, Fix("%.2f ms".to_string())))
				.chain((0..5).map(|i| 2 * i + 1).map(|x| Minor(x as f32)))
				.chain(Some(Major(-2.1f32, Fix("%.2f ms".to_string()))).into_iter()),
			&[MajorScale(2.0), MinorScale(0.5), OnAxis(true)],
			&[TextColor("blue"), TextAlign(AlignCenter)],
		)
		.set_y_ticks(Some((Fix(2.0), 1)), &[Mirror(false), Format("%.1f s")], &[]);

	c.show(&mut fg, "fg2.3.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Border, Axes fg2.4", &[])
		.set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
		.set_x_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
		.set_y_ticks(Some((Fix(5.0), 0)), &[Mirror(false)], &[])
		.lines(x3, y3, &[LineWidth(2.0), Color("blue")])
		.set_x_axis(true, &[LineWidth(2.0), LineStyle(DotDotDash)])
		.set_y_axis(true, &[LineWidth(2.0), Color("red")]);

	c.show(&mut fg, "fg2.4.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Image fg2.5", &[])
		.image(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[]);

	c.show(&mut fg, "fg2.5.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Image without borders fg2.6", &[])
		.set_border(false, &[], &[])
		.set_x_ticks(None, &[], &[])
		.set_y_ticks(None, &[], &[])
		.image(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[]);

	c.show(&mut fg, "fg2.6.gnuplot");

	let x4 = &[1f32, 10.0, 100.0];
	let y4 = &[1f32, 3f32, 9f32];

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Logarithmic fg2.7", &[])
		.lines(x4.iter(), y4.iter(), &[])
		.set_x_ticks(Some((Auto, 1)), &[], &[])
		.set_y_ticks(Some((Auto, 1)), &[], &[])
		.set_x_log(Some(10.0))
		.set_y_log(Some(3.0));

	c.show(&mut fg, "fg2.7.gnuplot");

	let mut fg = Figure::new();
	c.set_term(&mut fg);

	fg.axes2d()
		.set_title("Axis Grid fg2.8", &[])
		.lines(x3, y3, &[LineWidth(2.0), Color("blue")])
		.set_grid_options(true, &[LineStyle(DotDotDash), Color("black")])
		.set_x_grid(true)
		.set_y_grid(true);

	c.show(&mut fg, "fg2.8.gnuplot");
}

fn main()
{
	Common::new().map(|c| example(c));
}
