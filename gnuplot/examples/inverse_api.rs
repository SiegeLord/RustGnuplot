// This file is released into Public Domain.
use crate::common::*;
use gnuplot as gp;
use gnuplot::*;

mod common;

trait PlotElement
{
	fn convert(&self, axes: &mut gp::Axes2D);
	fn to_axes2d(&self) -> Axes2D;
}

#[derive(Clone)]
struct Lines
{
	x: Vec<f32>,
	y: Vec<f32>,
	options: Vec<PlotOption<String>>,
}

impl PlotElement for Lines
{
	fn convert(&self, axes: &mut gp::Axes2D)
	{
		let mut options: Vec<PlotOption<&str>> = vec![];
		for o in &self.options
		{
			options.push(match o
			{
				PointSymbol(v) => PointSymbol(*v),
				PointSize(v) => PointSize(*v),
				Caption(v) => Caption(&v),
				LineWidth(v) => LineWidth(*v),
				Color(v) => Color(v.to_ref()),
				BorderColor(v) => BorderColor(v.to_ref()),
				LineStyle(v) => LineStyle(*v),
				FillAlpha(v) => FillAlpha(*v),
				FillRegion(v) => FillRegion(*v),
				ArrowType(v) => ArrowType(*v),
				ArrowSize(v) => ArrowSize(*v),
				WhiskerBars(v) => WhiskerBars(*v),
				FillPattern(v) => FillPattern(*v),
				Axes(v1, v2) => Axes(*v1, *v2),
			});
		}

		axes.lines(self.x.clone(), self.y.clone(), &options);
	}

	fn to_axes2d(&self) -> Axes2D
	{
		Axes2D::new(vec![Box::new(self.clone())])
	}
}

impl<T: PlotElement + Clone + 'static> PlotElement for (T, T)
{
	fn convert(&self, axes: &mut gp::Axes2D)
	{
		self.0.convert(axes);
		self.1.convert(axes);
	}

	fn to_axes2d(&self) -> Axes2D
	{
		Axes2D::new(vec![Box::new(self.0.clone()), Box::new(self.1.clone())])
	}
}

fn lines<'l, Tx: IntoIterator<Item = f32>, Ty: IntoIterator<Item = f32>>(x: Tx, y: Ty) -> Lines
{
	Lines {
		x: x.into_iter().collect(),
		y: y.into_iter().collect(),
		options: vec![],
	}
}

#[allow(dead_code)]
impl Lines
{
	fn show(&self)
	{
		self.to_axes2d().show();
	}
}

struct Axes2D
{
	plot_elements: Vec<Box<dyn PlotElement>>,
	title: String,
	x_axis: Axis,
}

impl Axes2D
{
	fn new(plot_elements: Vec<Box<dyn PlotElement>>) -> Axes2D
	{
		Axes2D {
			plot_elements: plot_elements,
			title: "".into(),
			x_axis: axis(),
		}
	}

	fn title(&mut self, title: &str) -> &mut Axes2D
	{
		self.title = title.into();
		self
	}

	fn x(&mut self, axis: &Axis) -> &mut Axes2D
	{
		self.x_axis = axis.clone();
		self
	}

	fn show(&self)
	{
		let mut fg = Figure::new();
		let mut ax = fg.axes2d();
		ax.set_title(&self.title, &[]);
		ax.set_x_log(self.x_axis.log_scale);
		for pe in &self.plot_elements
		{
			pe.convert(&mut ax);
		}
		fg.show().unwrap();
	}
}

#[derive(Clone)]
struct Axis
{
	log_scale: Option<f64>,
}

impl Axis
{
	fn log_scale(&mut self, log_scale: Option<f64>) -> &mut Self
	{
		self.log_scale = log_scale;
		self
	}
}

fn axis() -> Axis
{
	Axis { log_scale: None }
}

fn example(c: Common)
{
	let z = (1..100).map(|z| z as f32 / 10.0);
	let x = z.clone().map(|z| z.cos());
	let y = z.clone().map(|z| z.sin());

	let mut fg = Figure::new();

	fg.axes2d().lines(z.clone(), y.clone(), &[]);

	c.show(&mut fg, "inverse_api_old_1");

	//~ fg.axes2d().set_title("Old API", &[]).lines(
	//~ z.clone(),
	//~ y.clone(),
	//~ &[LineWidth(2.), Color("#ffaa77".into())],
	//~ ).lines(
	//~ z.clone(),
	//~ x.clone(),
	//~ &[],
	//~ );

	//~ c.show(&mut fg, "inverse_api_old_2");

	//~ lines(z.clone(), y.clone()).show();

	let mut axes = (lines(z.clone(), y.clone()), lines(z.clone(), x.clone())).to_axes2d();
	axes.title("Test");
	axes.x(axis().log_scale(Some(10.)));

	if !c.no_show
	{
		axes.show();
	}
}

fn main()
{
	Common::new().map(|c| example(c));
}
