// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use self::DataSourceType::*;

pub use self::LabelType::*;
pub use self::PlotType::*;
use crate::coordinates::*;

use crate::datatype::*;
use crate::options::*;
use crate::util::{escape, OneWayOwned};
use crate::writer::*;
use crate::ColorType;
use std::borrow::Borrow;
use std::fs;
use std::path;

pub struct PlotElement
{
	data: Vec<f64>,
	num_rows: usize,
	num_cols: usize,
	plot_type: PlotType,
	source_type: DataSourceType,
	is_3d: bool,
	options: Vec<PlotOption<String>>,
}

impl PlotElement
{
	pub fn new_plot<'l>(
		plot_type: PlotType, data: Vec<f64>, num_rows: usize, num_cols: usize, options: &[PlotOption<&str>],
	) -> PlotElement{
		PlotElement {
			data,
			num_rows,
			num_cols,
			plot_type,
			source_type: Record,
			is_3d: false,
			options: options.to_one_way_owned(),
		}
	}

	pub fn new_plot_matrix<T: DataType, X: IntoIterator<Item = T>>(
		plot_type: PlotType, is_3d: bool, mat: X, num_rows: usize, num_cols: usize,
		dimensions: Option<(f64, f64, f64, f64)>, options: Vec<PlotOption<String>>,
	) -> PlotElement
	{
		let mut count = 0;
		let mut data = vec![];
		// TODO: Reserve.
		for x in mat
		{
			data.push(x.get());
			count += 1;
		}

		if count < num_rows * num_cols
		{
			data.resize(num_rows * num_cols, f64::NAN);
		}

		let source_type = match dimensions
		{
			Some((x1, y1, x2, y2)) => SizedArray(x1, y1, x2, y2),
			None => Array,
		};

		PlotElement {
			data,
			num_rows,
			num_cols,
			plot_type,
			source_type,
			is_3d,
			options,
		}
	}

	fn write_args(&self, source: &str, writer: &mut dyn Writer, version: GnuplotVersion)
	{
		let options = &self.options;
		match self.source_type
		{
			Record =>
			{
				write!(
					writer,
					r#" "{}" binary endian=little record={} format="%float64" using "#,
					source, self.num_rows
				);

				let mut col_idx = 1;
				while col_idx < self.num_cols + 1
				{
					write!(writer, "{}", col_idx);
					if col_idx < self.num_cols
					{
						writer.write_str(":");
					}
					col_idx += 1;
				}
			}
			_ =>
			{
				write!(
					writer,
					r#" "{}" binary endian=little array=({},{}) format="%float64" "#,
					source, self.num_cols, self.num_rows
				);

				if let SizedArray(x1, y1, x2, y2) = self.source_type
				{
					let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };

					let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
					write!(writer, "origin=({:.12e},{:.12e}", x1, y1);
					if self.is_3d
					{
						write!(writer, ",0");
					}
					write!(writer, ") ");
					if self.num_cols > 1
					{
						write!(
							writer,
							"dx={:.12e} ",
							(x2 - x1) / (self.num_cols as f64 - 1.0)
						);
					}
					else
					{
						write!(writer, "dx=1 ");
					}
					if self.num_rows > 1
					{
						write!(
							writer,
							"dy={:.12e} ",
							(y2 - y1) / (self.num_rows as f64 - 1.0)
						);
					}
					else
					{
						write!(writer, "dy=1 ");
					}
				}
			}
		}

		writer.write_str(" with ");
		let type_str = match self.plot_type
		{
			Lines => "lines",
			Points => "points",
			LinesPoints => "linespoints",
			XErrorLines => "xerrorlines",
			YErrorLines => "yerrorlines",
			XErrorBars => "xerrorbars",
			YErrorBars => "yerrorbars",
			XYErrorBars => "xyerrorbars",
			FillBetween => "filledcurves",
			Polygons => "polygons",
			Boxes => "boxes",
			BoxAndWhisker => "candlestick",
			BoxXYError => "boxxyerror",
			BoxErrorBars => "boxerrorbars",
			Pm3D => "pm3d",
			Image => "image",
		};
		writer.write_str(type_str);

		if self.plot_type.is_fill()
		{
			if let FillBetween = self.plot_type
			{
				first_opt! {options,
					FillRegion(d) =>
					{
						match d
						{
							Above => {writer.write_str(" above");},
							Below => {writer.write_str(" below");},
							Between => (),  // This is the default behavior.
						}
					}
				}
			}

			writer.write_str(" fill ");

			let mut is_pattern = false;
			first_opt! {self.options,
				FillPattern(pattern_opt) =>
				{
					is_pattern = true;
					writer.write_str("pattern ");
					if let Fix(val) = pattern_opt
					{
						write!(writer, "{}", val as i32);
					}
				}
			}

			if !is_pattern
			{
				writer.write_str("transparent solid");
				let mut alpha = 1.;
				first_opt! {self.options,
					FillAlpha(a) =>
					{
						alpha = a;
					}
				}
				write!(writer, " {:.12e}", alpha);
			}

			if self.plot_type.is_line()
			{
				first_opt! {self.options,
					BorderColorOpt(ref s) =>
					{
						write!(writer, " border {}", s.command());
					}
				}
			}
			else
			{
				writer.write_str(" noborder");
			}
		}

		if self.plot_type.is_line()
		{
			AxesCommonData::write_line_options(writer, options, version);
		}

		if self.plot_type.is_points()
		{
			first_opt! {self.options,
				PointSymbol(s) =>
				{
					write!(writer, " pt {}", char_to_symbol(s));
				}
			}

			first_opt! {self.options,
				PointSize(z) =>
				{
					write!(writer, " ps {}", z);
				}
			}
		}

		AxesCommonData::write_color_options(writer, &self.options, None);

		writer.write_str(" t \"");
		first_opt! {self.options,
			Caption(ref s) =>
			{
				writer.write_str(&escape(s));
			}
		}
		writer.write_str("\"");

		first_opt! {self.options,
			WhiskerBars(f) =>
			{
				write!(writer, " whiskerbars {}", f);
			}
		}

		first_opt! {self.options,
			Axes(x, y) =>
			{
				write!(writer, " axes {}{}",
					match x
					{
						XAxis::X1 => "x1",
						XAxis::X2 => "x2",
					},
					match y
					{
						YAxis::Y1 => "y1",
						YAxis::Y2 => "y2",
					}
				);
			}
		}
	}

	fn write_data(&self, writer: &mut dyn Writer)
	{
		for d in &self.data
		{
			writer.write_le_f64(*d);
		}
	}
}

pub struct LabelData
{
	pub label_type: LabelType,
	pub text: String,
	pub options: Vec<LabelOption<String>>,
}

impl LabelData
{
	fn new(label_type: LabelType) -> Self
	{
		Self {
			label_type,
			text: "".into(),
			options: vec![],
		}
	}

	pub fn set(&mut self, text: String, options: Vec<LabelOption<String>>)
	{
		self.text = text;
		self.options = options;
	}

	pub fn write_out_commands(&self, writer: &mut dyn Writer)
	{
		let w = writer;
		w.write_str("set ");

		self.label_type.write_label_str(w);

		w.write_str(" \"");
		w.write_str(&escape(&self.text));
		w.write_str("\"");

		write_out_label_options(self.label_type, &self.options[..], w);

		w.write_str("\n");
	}

	pub fn reset_state(&self, writer: &mut dyn Writer)
	{
		if let Label(tag, ..) = self.label_type
		{
			writeln!(writer, "unset label {}", tag);
		}
	}
}

#[derive(Copy, Clone)]
pub enum LabelType
{
	XLabel,
	YLabel,
	X2Label,
	Y2Label,
	ZLabel,
	CBLabel,
	TitleLabel,
	Label(i32, Coordinate, Coordinate),
	AxesTicks,
}

impl LabelType
{
	fn is_label(&self) -> bool
	{
		matches!(*self, Label(..))
	}

	fn write_label_str(&self, w: &mut dyn Writer)
	{
		match *self
		{
			XLabel =>
			{
				w.write_str("xlabel");
			}
			YLabel =>
			{
				w.write_str("ylabel");
			}
			X2Label =>
			{
				w.write_str("x2label");
			}
			Y2Label =>
			{
				w.write_str("y2label");
			}
			ZLabel =>
			{
				w.write_str("zlabel");
			}
			CBLabel =>
			{
				w.write_str("cblabel");
			}
			TitleLabel =>
			{
				w.write_str("title");
			}
			Label(tag, ..) =>
			{
				write!(w, "label {}", tag);
			}
			_ => panic!("Invalid label type"),
		}
	}

	fn from_axis(axis_type: TickAxis) -> Self
	{
		match axis_type
		{
			TickAxis::X => XLabel,
			TickAxis::Y => YLabel,
			TickAxis::X2 => X2Label,
			TickAxis::Y2 => Y2Label,
			TickAxis::Z => ZLabel,
			TickAxis::CB => CBLabel,
		}
	}
}

pub fn write_out_label_options(
	label_type: LabelType, options: &[LabelOption<String>], writer: &mut dyn Writer,
)
{
	let w = writer;
	if let Label(_, x, y) = label_type
	{
		write!(w, " at {},{} front", x, y);
	}

	first_opt! {options,
		TextOffset(x, y) =>
		{
			write!(w, " offset character {:.12e},{:.12e}", x, y);
		}
	}

	first_opt! {options,
		TextColor(ref s) =>
		{
			write!(w, r#" tc rgb "{}""#, s);
		}
	}

	first_opt! {options,
		Font(ref f, s) =>
		{
			write!(w, r#" font "{},{}""#, f, s);
		}
	}

	first_opt! {options,
		Rotate(a) =>
		{
			write!(w, " rotate by {:.12e}", a);
		}
	}

	if label_type.is_label()
	{
		let mut have_point = false;
		first_opt! {options,
			MarkerSymbol(s) =>
			{
				write!(w, " point pt {}", char_to_symbol(s));
				have_point = true;
			}
		}

		if have_point
		{
			first_opt! {options,
				MarkerColor(ref s) =>
				{
					write!(w, r#" lc rgb "{}""#, s.command());
				}
			}

			first_opt! {options,
				MarkerSize(z) =>
				{
					write!(w, " ps {:.12e}", z);
				}
			}
		}

		first_opt! {options,
			TextAlign(a) =>
			{
				write!(w, "{}", match a
				{
					AlignLeft => " left",
					AlignRight => " right",
					_ => " center",
				});
			}
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub enum TickAxis
{
	X,
	Y,
	X2,
	Y2,
	Z,
	CB,
}

impl TickAxis
{
	pub fn get_axis_str(&self) -> &str
	{
		match *self
		{
			TickAxis::X => "x",
			TickAxis::Y => "y",
			TickAxis::X2 => "x2",
			TickAxis::Y2 => "y2",
			TickAxis::Z => "z",
			TickAxis::CB => "cb",
		}
	}

	pub fn get_tick_str(&self) -> &str
	{
		match *self
		{
			TickAxis::X => "xtics",
			TickAxis::Y => "ytics",
			TickAxis::X2 => "x2tics",
			TickAxis::Y2 => "y2tics",
			TickAxis::Z => "ztics",
			TickAxis::CB => "cbtics",
		}
	}

	pub fn get_mtick_str(&self) -> &str
	{
		match *self
		{
			TickAxis::X => "mxtics",
			TickAxis::Y => "mytics",
			TickAxis::X2 => "mx2tics",
			TickAxis::Y2 => "my2tics",
			TickAxis::Z => "mztics",
			TickAxis::CB => "mcbtics",
		}
	}

	pub fn get_range_str(&self) -> &str
	{
		match *self
		{
			TickAxis::X => "xrange",
			TickAxis::Y => "yrange",
			TickAxis::X2 => "x2range",
			TickAxis::Y2 => "y2range",
			TickAxis::Z => "zrange",
			TickAxis::CB => "cbrange",
		}
	}
}

pub enum PlotType
{
	Lines,
	Points,
	LinesPoints,
	XErrorLines,
	YErrorLines,
	XErrorBars,
	XYErrorBars,
	YErrorBars,
	FillBetween,
	Polygons,
	Boxes,
	BoxErrorBars,
	BoxAndWhisker,
	BoxXYError,
	Pm3D,
	Image,
}

impl PlotType
{
	fn is_line(&self) -> bool
	{
		matches!(
			*self,
			Lines
				| LinesPoints
				| XErrorLines
				| Boxes | YErrorLines
				| BoxAndWhisker
				| BoxXYError | BoxErrorBars | Polygons
		)
	}

	fn is_points(&self) -> bool
	{
		matches!(
			*self,
			Points | LinesPoints | XErrorLines | YErrorLines | XErrorBars | YErrorBars | XYErrorBars
		)
	}

	fn is_fill(&self) -> bool
	{
		matches!(
			*self,
			Boxes | FillBetween | BoxAndWhisker | BoxXYError | BoxErrorBars | Polygons
		)
	}
}

pub enum TickType
{
	None,
	Custom(Vec<Tick<f64, String>>),
	Auto(AutoOption<f64>, u32),
}

pub struct AxisData
{
	pub tick_options: Vec<TickOption<String>>,
	pub label_options: Vec<LabelOption<String>>,
	pub tick_type: TickType,
	pub log_base: Option<f64>,
	pub axis: TickAxis,
	pub min: AutoOption<f64>,
	pub max: AutoOption<f64>,
	pub reverse: bool,
	pub grid: bool,
	pub mgrid: bool,
	pub is_time: bool,
	pub show: bool,
	pub label: LabelData,
	pub options: Vec<PlotOption<String>>,
}

impl AxisData
{
	pub fn new(axis: TickAxis) -> Self
	{
		AxisData {
			tick_options: vec![],
			label_options: vec![],
			tick_type: TickType::Auto(Auto, 0),
			log_base: None,
			axis,
			min: Auto,
			max: Auto,
			reverse: false,
			grid: false,
			mgrid: false,
			is_time: false,
			show: false,
			label: LabelData::new(LabelType::from_axis(axis)),
			options: vec![],
		}
	}

	pub fn write_out_commands(&self, w: &mut dyn Writer, version: GnuplotVersion)
	{
		if self.axis != TickAxis::CB
		{
			if self.show
			{
				w.write_str("set ");
				w.write_str(self.axis.get_axis_str());
				w.write_str("zeroaxis ");

				AxesCommonData::write_color_options(w, &self.options, Some(ColorType::RGBColor("black".into())));
				AxesCommonData::write_line_options(w, &self.options, version);
			}
			else
			{
				w.write_str("unset ");
				w.write_str(self.axis.get_axis_str());
				w.write_str("zeroaxis ");
			}
		}

		w.write_str("\n");

		let log = match self.log_base
		{
			Some(base) =>
			{
				w.write_str("set logscale ");
				w.write_str(self.axis.get_axis_str());
				write!(w, " {:.12e}", base);
				true
			}
			None =>
			{
				w.write_str("unset logscale ");
				w.write_str(self.axis.get_axis_str());
				false
			}
		};
		w.write_str("\n");

		w.write_str("set ");
		w.write_str(self.axis.get_axis_str());
		w.write_str("data");
		if self.is_time
		{
			w.write_str(" time");
		}
		w.write_str("\n");

		match self.tick_type
		{
			TickType::Auto(_, mticks) =>
			{
				write!(w, "set m{} ", self.axis.get_tick_str());
				if log
				{
					writeln!(w, "default");
				}
				else
				{
					writeln!(w, "{}", mticks as i32 + 1);
				}
			}
			_ =>
			{
				writeln!(w, "unset m{}", self.axis.get_tick_str());
			}
		}
		w.write_str("\n");

		w.write_str("set ");
		w.write_str(self.axis.get_range_str());
		w.write_str(" [");
		match self.min
		{
			Fix(v) => write!(w, "{:.12e}", v),
			Auto => w.write_str("*"),
		};
		w.write_str(":");
		match self.max
		{
			Fix(v) => write!(w, "{:.12e}", v),
			Auto => w.write_str("*"),
		};
		if self.reverse
		{
			w.write_str("] reverse\n");
		}
		else
		{
			w.write_str("]\n");
		}

		let mut write_tick_options = true;
		match self.tick_type
		{
			TickType::None =>
			{
				write!(w, "unset {0}", self.axis.get_tick_str());
				write_tick_options = false;
			}
			TickType::Auto(incr, _) =>
			{
				w.write_str("set ");
				w.write_str(self.axis.get_tick_str());

				match incr
				{
					Auto =>
					{
						w.write_str(" autofreq");
					}
					Fix(incr) =>
					{
						if incr <= 0.0
						{
							panic!("'incr' must be positive, but is actually {}", incr);
						}
						w.write_str(" ");
						write!(w, " {:.12e}", incr);
					}
				}
			}
			TickType::Custom(ref ticks) =>
			{
				w.write_str("set ");
				w.write_str(self.axis.get_tick_str());
				w.write_str(" (");

				let mut first = true;
				for tick in ticks
				{
					if first
					{
						first = false;
					}
					else
					{
						w.write_str(",");
					}

					let a = Auto;
					let (ref pos, label, level) = match *tick
					{
						Minor(ref pos) => (pos, &a, 1),
						Major(ref pos, ref label) => (pos, label, 0),
					};

					match *label
					{
						Fix(ref label) =>
						{
							w.write_str("\"");
							w.write_str(&label[..]);
							w.write_str("\" ");
						}
						Auto => (),
					}
					write!(w, "{:.12e} {}", pos.get(), level);
				}
				w.write_str(")");
			}
		}

		if write_tick_options
		{
			let label_options = &self.label_options;
			let tick_options = &self.tick_options;

			write_out_label_options(AxesTicks, &label_options[..], &mut *w);

			first_opt! {tick_options,
				OnAxis(b) =>
				{
					w.write_str(match b
					{
						true => " axis",
						false => " border",
					});
				}
			}

			first_opt! {tick_options,
				Mirror(b) =>
				{
					w.write_str(match b
					{
						true => " mirror",
						false => " nomirror",
					});
				}
			}

			first_opt! {tick_options,
				Inward(b) =>
				{
					w.write_str(match b
					{
						true => " in",
						false => " out",
					});
				}
			}

			let mut minor_scale = 0.5;
			let mut major_scale = 0.5;

			first_opt! {tick_options,
				MinorScale(s) =>
				{
					minor_scale = s;
				}
			}

			first_opt! {tick_options,
				MajorScale(s) =>
				{
					major_scale = s;
				}
			}

			write!(w, " scale {:.12e},{:.12e}", major_scale, minor_scale);

			first_opt! {tick_options,
				Format(ref f) =>
				{
					write!(w, r#" format "{}""#, f);
				}
			}
		}
		w.write_str("\n");
		self.label.write_out_commands(w);
		w.write_str("\n");
	}

	pub fn set_ticks_custom<T: DataType, TL: IntoIterator<Item = Tick<T, String>>>(
		&mut self, ticks: TL, tick_options: Vec<TickOption<String>>,
		label_options: Vec<LabelOption<String>>,
	)
	{
		self.tick_type = TickType::Custom(
			ticks
				.into_iter()
				.map(|t| match t
				{
					Major(t, l) => Major(t.get(), l),
					Minor(t) => Minor(t.get()),
				})
				.collect(),
		);
		self.tick_options = tick_options;
		self.label_options = label_options;
	}

	pub fn set_ticks(
		&mut self, tick_placement: Option<(AutoOption<f64>, u32)>,
		tick_options: Vec<TickOption<String>>, label_options: Vec<LabelOption<String>>,
	)
	{
		if let Some((incr, mticks)) = tick_placement
		{
			self.tick_type = TickType::Auto(incr, mticks);
			self.tick_options = tick_options;
			self.label_options = label_options;
		}
		else
		{
			self.tick_type = TickType::None
		}
	}

	pub fn set_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>)
	{
		self.min = min;
		self.max = max;
	}

	pub fn set_log(&mut self, base: Option<f64>)
	{
		self.log_base = base;
	}

	pub fn set_reverse(&mut self, reverse: bool)
	{
		self.reverse = reverse;
	}

	pub fn set_grid(&mut self, show: bool)
	{
		self.grid = show;
	}

	pub fn set_minor_grid(&mut self, show: bool)
	{
		self.mgrid = show;
	}

	pub fn set_time(&mut self, is_time: bool)
	{
		self.is_time = is_time;
	}
}

pub fn char_to_symbol(c: char) -> i32
{
	match c
	{
		'.' => 0,
		'+' => 1,
		'x' => 2,
		'*' => 3,
		's' => 4,
		'S' => 5,
		'o' => 6,
		'O' => 7,
		't' => 8,
		'T' => 9,
		'd' => 10,
		'D' => 11,
		'r' => 12,
		'R' => 13,
		a => panic!("Invalid symbol {}", a),
	}
}

enum DataSourceType
{
	Record,
	Array,
	SizedArray(f64, f64, f64, f64),
}

pub struct Margins
{
	pub left: Option<f32>,
	pub right: Option<f32>,
	pub top: Option<f32>,
	pub bottom: Option<f32>,
}

impl Default for Margins
{
	fn default() -> Self
	{
		Self::new()
	}
}

impl Margins
{
	pub fn new() -> Self
	{
		Margins {
			left: None,
			right: None,
			top: None,
			bottom: None,
		}
	}

	pub fn write_out_commands(&self, w: &mut dyn Writer)
	{
		let mut write_margin = |margin, v| {
			write!(w, "set {}", margin);
			if let Some(v) = v
			{
				write!(w, " at screen {}", v);
			}
			w.write_str("\n");
		};

		write_margin("lmargin", self.left);
		write_margin("rmargin", self.right);
		write_margin("tmargin", self.top);
		write_margin("bmargin", self.bottom);
	}
}

#[derive(Copy, Clone)]
pub struct Position
{
	x: f64,
	y: f64,
}

#[derive(Copy, Clone)]
pub struct Size
{
	w: f64,
	h: f64,
}

pub struct AxesCommonData
{
	pub grid_options: Vec<PlotOption<String>>,
	pub minor_grid_options: Vec<PlotOption<String>>,
	pub grid_front: bool,
	pub elems: Vec<PlotElement>,
	pub x_axis: AxisData,
	pub x2_axis: AxisData,
	pub y_axis: AxisData,
	pub y2_axis: AxisData,
	pub cb_axis: AxisData,
	pub labels: Vec<LabelData>,
	pub title: LabelData,
	pub position: Option<Position>,
	pub size: Option<Size>,
	pub aspect_ratio: AutoOption<f64>,
	pub margins: Margins,
	pub palette: PaletteType<Vec<(f32, f32, f32, f32)>>,
}

impl AxesCommonData
{
	pub fn new() -> AxesCommonData
	{
		let mut ret = AxesCommonData {
			grid_options: vec![],
			minor_grid_options: vec![],
			grid_front: false,
			elems: Vec::new(),
			x_axis: AxisData::new(TickAxis::X),
			y_axis: AxisData::new(TickAxis::Y),
			x2_axis: AxisData::new(TickAxis::X2),
			y2_axis: AxisData::new(TickAxis::Y2),
			cb_axis: AxisData::new(TickAxis::CB),
			labels: vec![],
			title: LabelData::new(TitleLabel),
			position: None,
			size: None,
			aspect_ratio: Auto,
			margins: Margins::new(),
			palette: COLOR.to_one_way_owned(),
		};
		ret.x2_axis.tick_type = TickType::None;
		ret.y2_axis.tick_type = TickType::None;
		ret
	}

	pub fn write_grid_options(&self, c: &mut dyn Writer, axes: &[TickAxis], version: GnuplotVersion)
	{
		if !axes.is_empty()
		{
			c.write_str("set grid ");
			for axis in axes
			{
				c.write_str(axis.get_tick_str());
				c.write_str(" ");
				if self.x_axis.axis == *axis && self.x_axis.mgrid
					|| self.y_axis.axis == *axis && self.y_axis.mgrid
					|| self.x2_axis.axis == *axis && self.x2_axis.mgrid
					|| self.y2_axis.axis == *axis && self.y2_axis.mgrid
				{
					c.write_str(axis.get_mtick_str());
					c.write_str(" ");
				}
			}

			if self.grid_front
			{
				c.write_str("front ");
			}
			else
			{
				c.write_str("back ");
			}

			AxesCommonData::write_line_options(c, &self.grid_options, version);
			AxesCommonData::write_color_options(c, &self.grid_options, None);
			c.write_str(", ");
			AxesCommonData::write_line_options(c, &self.minor_grid_options, version);
			AxesCommonData::write_color_options(c, &self.minor_grid_options, None);
			c.write_str("\n");
		}
	}

	pub fn write_line_options(
		c: &mut dyn Writer, options: &[PlotOption<String>], version: GnuplotVersion,
	)
	{
		let mut found = false;
		c.write_str(" lw ");
		first_opt! {options,
			LineWidth(w) =>
			{
				write!(c, "{:.12e}", w);
				found = true;
			}
		}
		if !found
		{
			c.write_str("1");
		}

		if version.major >= 5
		{
			first_opt! {options,
				LineStyle(d) =>
				{
					write!(c, " dt {}", d.to_int());
				}
			}
		}
		else
		{
			first_opt! {options,
				LineStyle(d) =>
				{
					write!(c, " lt {}", d.to_int());
				}
			}
		}
	}

	pub fn write_color_options(
		c: &mut dyn Writer, options: &[PlotOption<String>], default: Option<ColorType>,
	)
	{
		let mut col = default.as_ref();
		first_opt! {options,
			ColorOpt(ref s) =>
			{
				col = Some(s)
			}
		}
		if let Some(s) = col
		{
			write!(c, " lc {}", s.command());
		}
	}

	pub fn write_out_commands(
		&self, writer: &mut dyn Writer, auto_layout: bool, version: GnuplotVersion,
	)
	{
		let w = writer;
		if let Some(pos) = self.position
		{
			writeln!(w, "set origin {:.12e},{:.12e}", pos.x, pos.y);
		}
		else if !auto_layout
		{
			writeln!(w, "set origin");
		}
		if let Some(size) = self.size
		{
			writeln!(w, "set size {:.12e},{:.12e}", size.w, size.h);
		}
		else if !auto_layout
		{
			writeln!(w, "set size");
		}

		match self.aspect_ratio
		{
			Fix(r) =>
			{
				writeln!(w, "set size ratio {:.12e}", r);
			}
			Auto =>
			{
				writeln!(w, "set size noratio");
			}
		}
		self.margins.write_out_commands(w);

		match self.palette
		{
			Gray(gamma) =>
			{
				assert!(gamma > 0.0, "Gamma must be positive");
				writeln!(w, "set palette gray gamma {:.12e}", gamma);
			}
			Formula(r, g, b) =>
			{
				assert!(r >= -36 && r <= 36, "Invalid r formula!");
				assert!(g >= -36 && g <= 36, "Invalid g formula!");
				assert!(b >= -36 && b <= 36, "Invalid b formula!");
				writeln!(w, "set palette rgbformulae {},{},{}", r, g, b);
			}
			CubeHelix(start, rev, sat, gamma) =>
			{
				assert!(sat >= 0.0, "Saturation must be non-negative");
				assert!(gamma > 0.0, "Gamma must be positive");
				writeln!(
					w,
					"set palette cubehelix start {:.12e} cycles {:.12e} saturation {:.12e} gamma {:.12e}",
					start, rev, sat, gamma
				);
			}
			Custom(ref entries) =>
			{
				if entries.len() < 2
				{
					panic!("Need at least 2 elements in a custom palette");
				}
				write!(w, "set palette defined (");

				let mut first = true;
				let mut old_x = 0.0;
				for &(x, r, g, b) in entries
				{
					if first
					{
						old_x = x;
						first = false;
					}
					else
					{
						write!(w, ",");
					}
					assert!(x >= old_x, "The gray levels must be non-decreasing!");
					old_x = x;

					write!(w, "{:.12e} {:.12e} {:.12e} {:.12e}", x, r, g, b);
				}

				writeln!(w, ")");
			}
		}

		self.x_axis.write_out_commands(w, version);
		self.y_axis.write_out_commands(w, version);
		self.x2_axis.write_out_commands(w, version);
		self.y2_axis.write_out_commands(w, version);
		self.cb_axis.write_out_commands(w, version);
		self.title.write_out_commands(w);
		for label in &self.labels
		{
			label.write_out_commands(w);
		}
	}

	pub fn write_out_elements(
		&self, cmd: &str, data_directory: Option<&str>, writer: &mut dyn Writer,
		version: GnuplotVersion,
	)
	{
		if let Some(data_directory) = data_directory
		{
			for (i, e) in self.elems.iter().enumerate()
			{
				let filename = path::Path::new(data_directory).join(format!("{i}.bin"));
				let mut file = fs::File::create(&filename).unwrap();
				e.write_data(&mut file);
			}
		}

		write!(writer, "{}", cmd);

		let mut first = true;
		for (i, e) in self.elems.iter().enumerate()
		{
			if e.num_rows == 0
			{
				continue;
			}
			if !first
			{
				write!(writer, ",");
			}
			let source = if let Some(data_directory) = data_directory
			{
				escape(
					path::Path::new(data_directory)
						.join(format!("{i}.bin"))
						.to_str()
						.unwrap(),
				)
			}
			else
			{
				"-".into()
			};
			e.write_args(&source, writer, version);
			first = false;
		}

		writeln!(writer);

		if data_directory.is_none()
		{
			for e in self.elems.iter()
			{
				e.write_data(writer);
			}
		}
	}

	pub fn reset_state(&self, writer: &mut dyn Writer)
	{
		for label in &self.labels
		{
			label.reset_state(writer);
		}
	}
}

#[doc(hidden)]
pub trait AxesCommonPrivate
{
	fn get_common_data(&self) -> &AxesCommonData;
	fn get_common_data_mut(&mut self) -> &mut AxesCommonData;
}

pub trait AxesCommon: AxesCommonPrivate
{
	/// Set the position of the axes on the figure using grid coordinates.
	/// # Arguments
	/// * `nrow` - Number of rows in the grid. Must be greater than 0.
	/// * `ncol` - Number of columns in the grid. Must be greater than 0.
	/// * `pos` - Which grid cell to place this axes in, counting from top-left corner,
	///           going left and then down, starting at 0.
	fn set_pos_grid(&mut self, nrow: u32, ncol: u32, pos: u32) -> &mut Self
	{
		assert!(nrow > 0);
		assert!(ncol > 0);
		assert!(pos < nrow * ncol);
		let width = 1.0 / (ncol as f64);
		let height = 1.0 / (nrow as f64);
		let x = (pos % ncol) as f64 * width;
		let y = 1.0 - (1.0 + (pos / ncol) as f64) * height;

		self.get_common_data_mut().position = Some(Position { x, y });
		self.get_common_data_mut().size = Some(Size {
			w: width,
			h: height,
		});
		self
	}

	/// Set the position of the axes on the figure using screen coordinates.
	/// The coordinates refer to the bottom-left corner of the axes
	/// # Arguments
	/// * `x` - X position. Ranges from 0 to 1
	/// * `y` - Y position. Ranges from 0 to 1
	fn set_pos(&mut self, x: f64, y: f64) -> &mut Self
	{
		self.get_common_data_mut().position = Some(Position { x, y });
		self
	}

	/// Set the size of the axes
	/// # Arguments
	/// * `w` - Width. Ranges from 0 to 1
	/// * `h` - Height. Ranges from 0 to 1
	fn set_size(&mut self, w: f64, h: f64) -> &mut Self
	{
		self.get_common_data_mut().size = Some(Size { w, h });
		self
	}

	/// Set the aspect ratio of the axes
	/// # Arguments
	/// * `ratio` - The aspect ratio. Set to Auto to return the ratio to default
	fn set_aspect_ratio(&mut self, ratio: AutoOption<f64>) -> &mut Self
	{
		self.get_common_data_mut().aspect_ratio = ratio;
		self
	}

	/// Set the label for the X axis
	/// # Arguments
	/// * `text` - Text of the label. Pass an empty string to hide the label
	/// * `options` - Array of LabelOption controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_x_label<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut()
			.x_axis
			.label
			.set(text.into(), options.to_one_way_owned());
		self
	}

	/// Like `set_x_label`, but for the Y axis
	fn set_y_label<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut()
			.y_axis
			.label
			.set(text.into(), options.to_one_way_owned());
		self
	}

	/// Like `set_x_label`, but for the secondary X axis
	fn set_x2_label<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut()
			.x2_axis
			.label
			.set(text.into(), options.to_one_way_owned());
		self
	}

	/// Like `set_x_label`, but for the secondary Y axis
	fn set_y2_label<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut()
			.y2_axis
			.label
			.set(text.into(), options.to_one_way_owned());
		self
	}

	/// Like `set_x_label`, but for the color bar
	fn set_cb_label<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut()
			.cb_axis
			.label
			.set(text.into(), options.to_one_way_owned());
		self
	}

	/// Set the title for the axes
	/// # Arguments
	/// * `text` - Text of the title. Pass an empty string to hide the title
	/// * `options` - Array of LabelOption<&str> controlling the appearance of the title. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_title<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut()
			.title
			.set(text.into(), options.to_one_way_owned());
		self
	}

	/// Adds a label to the plot, with an optional marker.
	/// # Arguments
	/// * `text` - Text of the label
	/// * `x` - X coordinate of the label
	/// * `y` - Y coordinate of the label
	/// * `options` - Array of LabelOption<&str> controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	///      * `MarkerSymbol` - Specifies the symbol for the marker. Omit to hide the marker
	///      * `MarkerSize` - Specifies the size for the marker
	///      * `MarkerColor` - Specifies the color for the marker
	fn label<'l>(
		&'l mut self, text: &str, x: Coordinate, y: Coordinate, options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		{
			let labels = &mut self.get_common_data_mut().labels;
			let mut label = LabelData::new(Label(labels.len() as i32 + 1, x, y));
			label.set(text.into(), options.to_one_way_owned());
			labels.push(label);
		}
		self
	}

	/// Sets the properties of the ticks on the X axis.
	///
	/// # Arguments
	/// * `tick_placement` - Controls the placement of the ticks. Pass `None` to hide the ticks. Otherwise, the first tuple value controls the spacing
	///                      of the major ticks (in axes units), otherwise set it to `Auto` to let gnuplot decide the spacing automatically. The second
	///                      tuple value specifies the number of minor ticks. For logarithmic axes, non-zero values mean that the number of ticks usually
	///                      equals to `ceil(log_base) - 2`.
	/// * `tick_options` - Array of TickOption controlling the appearance of the ticks
	/// * `label_options` - Array of LabelOption<&str> controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_x_ticks<'l>(
		&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>,
		tick_options: &[TickOption<&str>], label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().x_axis.set_ticks(
			tick_placement,
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks` but for the Y axis.
	fn set_y_ticks<'l>(
		&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>,
		tick_options: &[TickOption<&str>], label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().y_axis.set_ticks(
			tick_placement,
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks` but for the secondary X axis.
	///
	/// Note that by default, these are hidden.
	fn set_x2_ticks<'l>(
		&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>,
		tick_options: &[TickOption<&str>], label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().y2_axis.set_ticks(
			tick_placement,
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks` but for the secondary Y axis.
	///
	/// Note that by default, these are hidden.
	fn set_y2_ticks<'l>(
		&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>,
		tick_options: &[TickOption<&str>], label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().y2_axis.set_ticks(
			tick_placement,
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks` but for the color bar axis.
	fn set_cb_ticks<'l>(
		&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>,
		tick_options: &[TickOption<&str>], label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().cb_axis.set_ticks(
			tick_placement,
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Sets ticks on the X axis with specified labels at specified positions.
	///
	/// # Arguments
	///
	/// * `ticks` - The locations and labels of the added ticks.
	///     The label can contain a single C printf style floating point formatting specifier which will be replaced by the
	///     location of the tic.
	/// * `tick_options` - Array of TickOption controlling the appearance of the ticks
	/// * `label_options` - Array of LabelOption<&str> controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_x_ticks_custom<
		'l,
		T: DataType,
		S: ToString,
		TickT: Borrow<Tick<T, S>>,
		TL: IntoIterator<Item = TickT>,
	>(
		&'l mut self, ticks: TL, tick_options: &[TickOption<&str>],
		label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().x_axis.set_ticks_custom(
			ticks.into_iter().map(|e| e.borrow().to_one_way_owned()),
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks_custom` but for the the Y axis.
	fn set_y_ticks_custom<
		'l,
		T: DataType,
		S: ToString,
		TickT: Borrow<Tick<T, S>>,
		TL: IntoIterator<Item = TickT>,
	>(
		&'l mut self, ticks: TL, tick_options: &[TickOption<&str>],
		label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().y_axis.set_ticks_custom(
			ticks.into_iter().map(|e| e.borrow().to_one_way_owned()),
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks_custom` but for the the secondary X axis.
	fn set_x2_ticks_custom<
		'l,
		T: DataType,
		S: ToString,
		TickT: Borrow<Tick<T, S>>,
		TL: IntoIterator<Item = TickT>,
	>(
		&'l mut self, ticks: TL, tick_options: &[TickOption<&str>],
		label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().x2_axis.set_ticks_custom(
			ticks.into_iter().map(|e| e.borrow().to_one_way_owned()),
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks_custom` but for the the secondary Y axis.
	fn set_y2_ticks_custom<
		'l,
		T: DataType,
		S: ToString,
		TickT: Borrow<Tick<T, S>>,
		TL: IntoIterator<Item = TickT>,
	>(
		&'l mut self, ticks: TL, tick_options: &[TickOption<&str>],
		label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().y2_axis.set_ticks_custom(
			ticks.into_iter().map(|e| e.borrow().to_one_way_owned()),
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Like `set_x_ticks_custom` but for the the color bar axis.
	fn set_cb_ticks_custom<
		'l,
		T: DataType,
		S: ToString,
		TickT: Borrow<Tick<T, S>>,
		TL: IntoIterator<Item = TickT>,
	>(
		&'l mut self, ticks: TL, tick_options: &[TickOption<&str>],
		label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.get_common_data_mut().cb_axis.set_ticks_custom(
			ticks.into_iter().map(|e| e.borrow().to_one_way_owned()),
			tick_options.to_one_way_owned(),
			label_options.to_one_way_owned(),
		);
		self
	}

	/// Set the range of values for the X axis.
	///
	/// # Arguments
	/// * `min` - Minimum X value
	/// * `max` - Maximum X value
	fn set_x_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self
	{
		self.get_common_data_mut().x_axis.set_range(min, max);
		self
	}

	/// Set the range of values for the Y axis.
	///
	/// # Arguments
	/// * `min` - Minimum Y value
	/// * `max` - Maximum Y value
	fn set_y_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self
	{
		self.get_common_data_mut().y_axis.set_range(min, max);
		self
	}

	/// Set the range of values for the secondary X axis.
	///
	/// # Arguments
	/// * `min` - Minimum X value
	/// * `max` - Maximum X value
	fn set_x2_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self
	{
		self.get_common_data_mut().x2_axis.set_range(min, max);
		self
	}

	/// Set the range of values for the secondary Y axis.
	///
	/// # Arguments
	/// * `min` - Minimum Y value
	/// * `max` - Maximum Y value
	fn set_y2_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self
	{
		self.get_common_data_mut().y2_axis.set_range(min, max);
		self
	}

	/// Sets X axis to reverse.
	/// # Arguments
	/// * `reverse` - Boolean, true to reverse axis, false will not reverse
	fn set_x_reverse(&mut self, reverse: bool) -> &mut Self
	{
		self.get_common_data_mut().x_axis.set_reverse(reverse);
		self
	}

	/// Sets Y axis to reverse.
	/// # Arguments
	/// * `reverse` - Boolean, true to reverse axis, false will not reverse
	fn set_y_reverse(&mut self, reverse: bool) -> &mut Self
	{
		self.get_common_data_mut().y_axis.set_reverse(reverse);
		self
	}

	/// Sets secondary X axis to reverse.
	/// # Arguments
	/// * `reverse` - Boolean, true to reverse axis, false will not reverse
	fn set_x2_reverse(&mut self, reverse: bool) -> &mut Self
	{
		self.get_common_data_mut().x2_axis.set_reverse(reverse);
		self
	}

	/// Sets secondary Y axis to reverse.
	/// # Arguments
	/// * `reverse` - Boolean, true to reverse axis, false will not reverse
	fn set_y2_reverse(&mut self, reverse: bool) -> &mut Self
	{
		self.get_common_data_mut().y2_axis.set_reverse(reverse);
		self
	}

	/// Set the range of values for the color bar axis.
	///
	/// # Arguments
	/// * `min` - Minimum Y value
	/// * `max` - Maximum Y value
	fn set_cb_range(&mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &mut Self
	{
		self.get_common_data_mut().cb_axis.set_range(min, max);
		self
	}

	/// Sets the X axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_x_log(&mut self, base: Option<f64>) -> &mut Self
	{
		self.get_common_data_mut().x_axis.set_log(base);
		self
	}

	/// Sets the Y axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_y_log(&mut self, base: Option<f64>) -> &mut Self
	{
		self.get_common_data_mut().y_axis.set_log(base);
		self
	}

	/// Sets the secondary X axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_x2_log(&mut self, base: Option<f64>) -> &mut Self
	{
		self.get_common_data_mut().x2_axis.set_log(base);
		self
	}

	/// Sets the secondary Y axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_y2_log(&mut self, base: Option<f64>) -> &mut Self
	{
		self.get_common_data_mut().y2_axis.set_log(base);
		self
	}

	/// Sets the color bar axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_cb_log(&mut self, base: Option<f64>) -> &mut Self
	{
		self.get_common_data_mut().cb_axis.set_log(base);
		self
	}

	/// Shows the grid on the X axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_x_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().x_axis.set_grid(show);
		self
	}

	/// Shows the minor grid on the X axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_x_minor_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().x_axis.set_minor_grid(show);
		self
	}

	/// Shows the grid on the Y axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_y_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().y_axis.set_grid(show);
		self
	}

	/// Shows the minor grid on the Y axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_y_minor_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().y_axis.set_minor_grid(show);
		self
	}

	/// Shows the grid on the secondary X axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_x2_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().x2_axis.set_grid(show);
		self
	}

	/// Shows the minor grid on the secondary X axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_x2_minor_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().x2_axis.set_minor_grid(show);
		self
	}

	/// Shows the grid on the secondary Y axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_y2_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().y2_axis.set_grid(show);
		self
	}

	/// Shows the minor grid on the secondary Y axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_y2_minor_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().y2_axis.set_minor_grid(show);
		self
	}

	/// Shows the grid on the color bar axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	fn set_cb_grid(&mut self, show: bool) -> &mut Self
	{
		self.get_common_data_mut().cb_axis.set_grid(show);
		self
	}

	/// Set the grid options.
	///
	/// # Arguments
	/// * `front` - Whether the grid should be in the front of the plot elements or behind them.
	/// * `options` - Styling options of the grid. Relevant options are:
	///      * `Color` - Specifies the color of the grid lines
	///      * `LineStyle` - Specifies the style of the grid lines
	///      * `LineWidth` - Specifies the width of the grid lines
	fn set_grid_options<'l>(&'l mut self, front: bool, options: &[PlotOption<&str>])
		-> &'l mut Self
	{
		self.get_common_data_mut().grid_front = front;
		self.get_common_data_mut().grid_options = options.to_one_way_owned();
		self
	}

	/// Set the minor grid options.
	///
	/// # Arguments
	/// * `options` - Styling options of the grid. Relevant options are:
	///      * `Color` - Specifies the color of the grid lines
	///      * `LineStyle` - Specifies the style of the grid lines
	///      * `LineWidth` - Specifies the width of the grid lines
	fn set_minor_grid_options<'l>(&'l mut self, options: &[PlotOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut().minor_grid_options = options.to_one_way_owned();
		self
	}

	/// Sets the X axis be time.
	///
	/// If true, the axis is interpreted as seconds from the Unix epoch. Use the `Format` TickOption to
	/// specify the formatting of the ticks (see strftime format spec for valid values).
	///
	/// # Arguments
	/// * `is_time` - Whether this axis is time or not.
	fn set_x_time(&mut self, is_time: bool) -> &mut Self
	{
		self.get_common_data_mut().x_axis.set_time(is_time);
		self
	}

	/// Sets the Y axis be time. Note that the range must be non-negative for this to be valid.
	///
	/// If true, the axis is interpreted as seconds from the Unix epoch. Use the `Format` TickOption to
	/// specify the formatting of the ticks (see strftime format spec for valid values).
	///
	/// # Arguments
	/// * `is_time` - Whether this axis is time or not.
	fn set_y_time(&mut self, is_time: bool) -> &mut Self
	{
		self.get_common_data_mut().y_axis.set_time(is_time);
		self
	}

	/// Sets the secondary X axis be time.
	///
	/// If true, the axis is interpreted as seconds from the Unix epoch. Use the `Format` TickOption to
	/// specify the formatting of the ticks (see strftime format spec for valid values).
	///
	/// # Arguments
	/// * `is_time` - Whether this axis is time or not.
	fn set_x2_time(&mut self, is_time: bool) -> &mut Self
	{
		self.get_common_data_mut().x2_axis.set_time(is_time);
		self
	}

	/// Sets the secondary Y axis be time. Note that the range must be non-negative for this to be valid.
	///
	/// If true, the axis is interpreted as seconds from the Unix epoch. Use the `Format` TickOption to
	/// specify the formatting of the ticks (see strftime format spec for valid values).
	///
	/// # Arguments
	/// * `is_time` - Whether this axis is time or not.
	fn set_y2_time(&mut self, is_time: bool) -> &mut Self
	{
		self.get_common_data_mut().y2_axis.set_time(is_time);
		self
	}

	/// Sets the color bar axis be time. Note that the range must be non-negative for this to be valid.
	///
	/// If true, the axis is interpreted as seconds from the Unix epoch. Use the `Format` TickOption to
	/// specify the formatting of the ticks (see strftime format spec for valid values).
	///
	/// # Arguments
	/// * `is_time` - Whether this axis is time or not.
	fn set_cb_time(&mut self, is_time: bool) -> &mut Self
	{
		self.get_common_data_mut().cb_axis.set_time(is_time);
		self
	}

	/// Sets the margins of the plot.
	///
	/// # Arguments
	///
	/// * `margins` - The values of margins to be overriden. Specified as a fraction of the
	///               full drawing area, ranging from 0 to 1
	fn set_margins(&mut self, margins: &[MarginSide]) -> &mut Self
	{
		{
			let m = &mut self.get_common_data_mut().margins;
			*m = Margins::new();
			for &s in margins.iter()
			{
				match s
				{
					MarginLeft(frac) => m.left = Some(frac),
					MarginRight(frac) => m.right = Some(frac),
					MarginTop(frac) => m.top = Some(frac),
					MarginBottom(frac) => m.bottom = Some(frac),
				};
			}
		}
		self
	}

	/// Sets the palette used for 3D surface and image plots
	///
	/// # Arguments
	/// * `palette` - What palette type to use
	fn set_palette(&mut self, palette: PaletteType<&[(f32, f32, f32, f32)]>) -> &mut Self
	{
		self.get_common_data_mut().palette = palette.to_one_way_owned();
		self
	}
}
