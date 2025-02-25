// This file is released into Public Domain.
use crate::common::*;
use gnuplot::*;

mod common;

fn example(c: Common)
{
	let mut fg = Figure::new();

	fg.axes2d()
		.set_title("Multiple axes", &[])
		.lines_points(
			[0.0f32, 1.0, 2.0].iter(),
			[-1.0f32, 0.0, 1.0].iter(),
			&[Axes(X1, Y1), Color("blue")],
		)
		.lines_points(
			[-0.6f32, 1.5, 2.5].iter(),
			[-5.0f32, 0.0, 5.0].iter(),
			&[Axes(X1, Y2), Color("red")],
		)
		.set_y_ticks(Some((Auto, 0)), &[Mirror(false)], &[TextColor("blue")]) // Make Y1 not mirror.
		.set_y2_ticks(Some((Auto, 0)), &[Mirror(false)], &[TextColor("red")]) // Make Y2 not mirror, and visible.
		.set_y_label("Blue", &[TextColor("blue")])
		.set_y2_label("Red", &[TextColor("red")])
		.label("Blue Label", Axis(1.), Axis(0.), &[TextColor("blue"), TextAlign(AlignRight)])
		.label("Red Label", Axis(2.0), Axis2(2.5), &[TextColor("red")]);

	c.show(&mut fg, "multiple_axes");
}

fn main()
{
	Common::new().map(|c| example(c));
}
