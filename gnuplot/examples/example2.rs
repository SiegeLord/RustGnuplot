// This file is released into Public Domain.
use crate::common::*;

use gnuplot::*;

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

	let x3 = &[1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
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

	fg.axes2d()
		.set_title("Arrows fg2.1", &[])
		.lines(
			x,
			y1,
			&[LineWidth(3.0), Color("brown".into()), LineStyle(DotDash)],
		)
		.arrow(
			Graph(0.5),
			Graph(1.0),
			Axis(1.0),
			Axis(1.0),
			&[
				ArrowType(Filled),
				ArrowSize(0.1),
				LineStyle(DotDotDash),
				LineWidth(2.0),
				Color("red".into()),
			],
		)
		.arrow(
			Graph(0.5),
			Graph(1.0),
			Axis(3.0),
			Axis(9.0),
			&[ArrowType(Open), Color("green".into())],
		);

	c.show(&mut fg, "example2_1");

	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Boxes fg2.2", &[])
		.boxes(
			x2,
			y2,
			&[
				LineWidth(2.0),
				Color("cyan".into()),
				BorderColor("blue".into()),
				LineStyle(DotDash),
			],
		)
		.boxes(
			x,
			y1,
			&[
				LineWidth(2.0),
				Color("gray".into()),
				BorderColor("black".into()),
				BoxWidth([0.5, 0.4, 0.55, 0.7, 0.2].into()),
			],
		);

	c.show(&mut fg, "example2_2");

	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Axis Ticks fg2.3", &[])
		.lines(x3, y3, &[LineWidth(2.0), Color("blue".into())])
		.set_x_ticks_custom(
			x3.map(|&x| Major(x as f32, Fix("%.2f ms".to_string())))
				.chain(x3.map(|&i| i as f32 + 0.5).map(|x| Minor(x))),
			&[MajorScale(2.0), MinorScale(0.5), OnAxis(true)],
			&[TextColor("blue".into()), TextAlign(AlignCenter)],
		)
		.set_x_log(Some(10.0))
		.set_y_ticks(
			Some((Fix(100.0), 1)),
			&[Mirror(false), Format("%.1f s")],
			&[],
		);

	c.show(&mut fg, "example2_3");

	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Border, Axes fg2.4", &[])
		.set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
		.set_x_ticks(Some((Fix(1.0), 1)), &[Mirror(false)], &[])
		.set_y_ticks(Some((Fix(50.0), 0)), &[Mirror(false)], &[])
		.lines(x3, y3, &[LineWidth(2.0), Color("blue".into())])
		.set_x_axis(true, &[LineWidth(2.0), LineStyle(DotDotDash)])
		.set_y_axis(true, &[LineWidth(2.0), Color("red".into())]);

	c.show(&mut fg, "example2_4");

	let mut fg = Figure::new();

	fg.axes2d().set_title("Image fg2.5", &[]).image(
		z1.iter(),
		zw,
		zh,
		Some((-4.0, -4.0, 4.0, 4.0)),
		&[],
	);

	c.show(&mut fg, "example2_5");

	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Image without borders fg2.6", &[])
		.set_border(false, &[], &[])
		.set_x_ticks(None, &[], &[])
		.set_y_ticks(None, &[], &[])
		.image(z1.iter(), zw, zh, Some((-4.0, -4.0, 4.0, 4.0)), &[]);

	c.show(&mut fg, "example2_6");

	let x4 = &[1f32, 10.0, 100.0];
	let y4 = &[1f32, 3f32, 9f32];

	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Logarithmic fg2.7", &[])
		.lines(x4.iter(), y4.iter(), &[])
		.set_x_ticks(Some((Auto, 1)), &[], &[])
		.set_y_ticks(Some((Auto, 1)), &[], &[])
		.set_x_log(Some(10.0))
		.set_y_log(Some(3.0));

	c.show(&mut fg, "example2_7");

	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Axis Grid fg2.8", &[])
		.lines(x3, y3, &[LineWidth(2.0), Color("blue".into())])
		.set_y_ticks(Some((Auto, 2)), &[], &[])
		.set_grid_options(true, &[LineStyle(DotDotDash), Color("black".into())])
		.set_minor_grid_options(&[LineStyle(SmallDot), Color("red".into())])
		.set_x_grid(true)
		.set_y_grid(true)
		.set_y_minor_grid(true);

	c.show(&mut fg, "example2_8");
}

fn main()
{
	Common::new().map(|c| example(c));
}
