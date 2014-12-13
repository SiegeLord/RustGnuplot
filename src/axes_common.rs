// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::io::{MemWriter, Writer, IoResult};

use datatype::*;
use coordinates::*;
use options::*;
use writer::*;

pub use self::LabelType::*;
pub use self::PlotType::*;
pub use self::TickAxis::*;
pub use self::PlotType::*;
pub use self::DataSourceType::*;

pub struct ResetMemWriter
{
	buf: Vec<u8>,
}

impl ResetMemWriter {
	pub fn new() -> ResetMemWriter
	{
		ResetMemWriter{ buf: Vec::with_capacity(128) }
	}

	pub fn get_ref<'l>(&'l self) -> &'l [u8]
	{
		self.buf.as_slice()
	}

	pub fn truncate(&mut self, new_len: uint)
	{
		self.buf.truncate(new_len);
	}
}

impl Writer for ResetMemWriter
{
	fn write(&mut self, buf: &[u8]) -> IoResult<()>
	{
		self.buf.push_all(buf);
		Ok(())
	}
}

impl PlotWriter for ResetMemWriter
{
	fn write_data<T: DataType>(&mut self, v: T)
	{
		self.write_le_f64(v.get());
	}
}

pub struct PlotElement
{
	pub args: MemWriter,
	pub data: MemWriter
}

impl PlotElement
{
	pub fn new() -> PlotElement
	{
		PlotElement
		{
			args: MemWriter::new(),
			data: MemWriter::new(),
		}
	}
}

pub enum LabelType
{
	XLabel,
	YLabel,
	ZLabel,
	TitleLabel,
	Label(Coordinate, Coordinate),
	AxesTicks,
}

impl LabelType
{
	fn is_label(&self) -> bool
	{
		match *self
		{
			Label(..) => true,
			_ => false
		}
	}
}

pub fn write_out_label_options<T: PlotWriter + Writer>(label_type: LabelType, options: &[LabelOption], writer: &mut T)
{
	let w = writer as &mut Writer;

	match label_type
	{
		Label(ref x, ref y) =>
		{
			write!(w, " at {},{} front", x, y);
		}
		_ => ()
	}

	first_opt!(options,
		TextOffset(x, y) =>
		{
			write!(w, " offset character {:.12e},{:.12e}", x, y);
		}
	)

	first_opt!(options,
		TextColor(s) =>
		{
			write!(w, r#" tc rgb "{}""#, s);
		}
	)

	first_opt!(options,
		Font(f, s) =>
		{
			write!(w, r#" font "{},{}""#, f, s);
		}
	)

	first_opt!(options,
		Rotate(a) =>
		{
			write!(w, " rotate by {:.12e}", a);
		}
	)

	if label_type.is_label()
	{
		let mut have_point = false;
		first_opt!(options,
			MarkerSymbol(s) =>
			{
				write!(w, " point pt {}", char_to_symbol(s));
				have_point = true;
			}
		)

		if have_point
		{
			first_opt!(options,
				MarkerColor(s) =>
				{
					write!(w, r#" lc rgb "{}""#, s);
				}
			)

			first_opt!(options,
				MarkerSize(z) =>
				{
					write!(w, " ps {:.12e}", z);
				}
			)
		}

		first_opt!(options,
			TextAlign(a) =>
			{
				write!(w, "{}", match a
				{
					AlignLeft => " left",
					AlignRight => " right",
					_ => " center",
				});
			}
		)
	}
}

pub enum TickAxis
{
	XTickAxis,
	YTickAxis,
	ZTickAxis,
}

impl TickAxis
{
	pub fn to_axis_str(&self) -> &str
	{
		match *self
		{
			XTickAxis => "x",
			YTickAxis => "y",
			ZTickAxis => "z",
		}
	}

	pub fn to_tick_str(&self) -> &str
	{
		match *self
		{
			XTickAxis => "xtics",
			YTickAxis => "ytics",
			ZTickAxis => "ztics",
		}
	}

	pub fn to_range_str(&self) -> &str
	{
		match *self
		{
			XTickAxis => "xrange",
			YTickAxis => "yrange",
			ZTickAxis => "zrange",
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
	FillBetween,
	Boxes,
	Pm3D,
	Image,
}

impl PlotType
{
	fn is_line(&self) -> bool
	{
		match *self
		{
			Lines |
			LinesPoints |
			XErrorLines |
			Boxes |
			YErrorLines => true,
			_ => false
		}
	}

	fn is_points(&self) -> bool
	{
		match *self
		{
			Points |
			LinesPoints |
			XErrorLines |
			YErrorLines => true,
			_ => false
		}
	}

	fn is_fill(&self) -> bool
	{
		match *self
		{
			Boxes |
			FillBetween => true,
			_ => false
		}
	}
}

pub struct AxisData
{
	pub ticks_buf: ResetMemWriter,
	pub log_base: Option<f64>,
	pub mticks: i32,
	pub axis: TickAxis,
	pub min: AutoOption<f64>,
	pub max: AutoOption<f64>,
}

impl AxisData
{
	pub fn new(axis: TickAxis) -> AxisData
	{
		AxisData
		{
			ticks_buf: ResetMemWriter::new(),
			log_base: None,
			mticks: 0,
			axis: axis,
			min: Auto,
			max: Auto,
		}
	}
	
	pub fn write_out_commands(&self, w: &mut Writer)
	{
		let log = match self.log_base
		{
			Some(base) =>
			{
				w.write_str("set logscale ");
				w.write_str(self.axis.to_axis_str());
				write!(w, " {:.12e}", base);
				true
			},
			None =>
			{
				w.write_str("unset logscale ");
				w.write_str(self.axis.to_axis_str());
				false
			}
		};

		w.write_str("\n");
		if self.mticks > 0
		{
			write!(w, "set m{} ", self.axis.to_tick_str());
			if log
			{
				writeln!(w, "default");
			}
			else
			{
				writeln!(w, "{}", self.mticks as i32 + 1);
			}
		}
		else
		{
			writeln!(w, "unset m{}", self.axis.to_tick_str());
		}

		w.write_str("\n");
		w.write_str("set ");
		w.write_str(self.axis.to_range_str());
		w.write_str(" [");
		match self.min
		{
			Fix(v) => write!(w, "{:.12e}", v),
			Auto => w.write_str("*")
		};
		w.write_str(":");
		match self.max
		{
			Fix(v) => write!(w, "{:.12e}", v),
			Auto => w.write_str("*")
		};
		w.write_str("]\n");
		
		w.write(self.ticks_buf.get_ref());
	}
	
	pub fn set_ticks_custom<T: DataType, TL: Iterator<Tick<T>>>(&mut self, mut ticks: TL, tick_options: &[TickOption], label_options: &[LabelOption])
	{
		// Set to 0 so that we don't get any non-custom ticks
		self.mticks = 0;
		{
			let c = &mut self.ticks_buf;
			c.truncate(0);

			c.write_str("set ");
			c.write_str(self.axis.to_tick_str());
			c.write_str(" (");

			let mut first = true;
			for tick in ticks
			{
				if first
				{
					first = false;
				}
				else
				{
					c.write_str(",");
				}

				let a = Auto;
				let (ref pos, ref label, level) = match tick
				{
					Minor(ref pos) =>
					{
						(pos, &a, 1u)
					},
					Major(ref pos, ref label) =>
					{
						(pos, label, 0u)
					}
				};

				match **label
				{
					Fix(ref label) =>
					{
						c.write_str("\"");
						c.write_str(label.as_slice());
						c.write_str("\" ");
					},
					Auto => ()
				}
				write!(&mut *c, "{:.12e} {}", pos.get(), level);
			}
			c.write_str(")");
		}
		self.set_ticks_options(tick_options, label_options);
		self.ticks_buf.write_str("\n");
	}

	fn set_ticks_options(&mut self, tick_options: &[TickOption], label_options: &[LabelOption])
	{
		let c = &mut self.ticks_buf;
		write_out_label_options(AxesTicks, label_options, c);

		first_opt!(tick_options,
			OnAxis(b) =>
			{
				c.write_str(match b
				{
					true => " axis",
					false => " border",
				});
			}
		)

		first_opt!(tick_options,
			Mirror(b) =>
			{
				c.write_str(match b
				{
					true => " mirror",
					false => " nomirror",
				});
			}
		)

		first_opt!(tick_options,
			Inward(b) =>
			{
				c.write_str(match b
				{
					true => " in",
					false => " out",
				});
			}
		)

		let mut minor_scale = 0.5;
		let mut major_scale = 0.5;

		first_opt!(tick_options,
			MinorScale(s) =>
			{
				minor_scale = s;
			}
		)

		first_opt!(tick_options,
			MajorScale(s) =>
			{
				major_scale = s;
			}
		)

		write!(&mut *c, " scale {:.12e},{:.12e}", minor_scale, major_scale);
	}

	pub fn set_ticks(&mut self, tick_placement: Option<(AutoOption<f64>, u32)>, tick_options: &[TickOption], label_options: &[LabelOption])
	{
		self.ticks_buf.truncate(0);
			
		self.mticks = match tick_placement
		{
			Some((incr, mticks)) =>
			{
				{
					let c = &mut self.ticks_buf;
					c.write_str("set ");
					c.write_str(self.axis.to_tick_str());

					match incr
					{
						Auto =>
						{
							c.write_str(" autofreq");
						},
						Fix(incr) =>
						{
							if incr <= 0.0
							{
								panic!("'incr' must be positive, but is actually {}", incr);
							}
							c.write_str(" ");
							write!(&mut *c, " {:.12e}", incr);
						}
					}
				}

				self.set_ticks_options(tick_options, label_options);
				mticks as i32
			},
			None =>
			{
				write!(&mut self.ticks_buf, "unset {0}", self.axis.to_tick_str());
				0
			}
		};
		self.ticks_buf.write_str("\n");
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
}

pub struct AxesCommonData
{
	pub commands: MemWriter,
	pub elems: Vec<PlotElement>,
	pub grid_rows: u32,
	pub grid_cols: u32,
	pub grid_pos: Option<u32>,
	pub x_axis: AxisData,
	pub y_axis: AxisData,
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
		a => panic!("Invalid symbol {}", a)
	}
}

enum DataSourceType
{
	Record,
	Array,
	SizedArray(f64, f64, f64, f64),
}

impl AxesCommonData
{
	pub fn new() -> AxesCommonData
	{
		AxesCommonData
		{
			commands: MemWriter::new(),
			elems: Vec::new(),
			grid_rows: 0,
			grid_cols: 0,
			grid_pos: None,
			x_axis: AxisData::new(XTickAxis),
			y_axis: AxisData::new(YTickAxis),
		}
	}

	pub fn write_line_options(c: &mut Writer, options: &[PlotOption])
	{
		let mut found = false;
		c.write_str(" lw ");
		first_opt!(options,
			LineWidth(w) =>
			{
				write!(c, "{:.12e}", w);
				found = true;
			}
		)
		if !found
		{
			c.write_str("1");
		}

		c.write_str(" lt ");
		let mut found = false;
		first_opt!(options,
			LineStyle(d) =>
			{
				write!(c, "{}", d.to_int());
				found = true;
			}
		)
		if !found
		{
			c.write_str("1");
		}
	}

	pub fn write_color_options<'l>(c: &mut Writer, options: &[PlotOption<'l>], default: Option<&'l str>)
	{
		let mut col = default;
		first_opt!(options,
			Color(s) =>
			{
				col = Some(s)
			}
		)
		match col
		{
			Some(s) =>
			{
				write!(c, r#" lc rgb "{}""#, s);
			},
			None => ()
		}
	}
	
	pub fn plot2<T1: DataType, X1: Iterator<T1>,
	             T2: DataType, X2: Iterator<T2>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, options: &[PlotOption])
	{
		let l = self.elems.len();
		self.elems.push(PlotElement::new());
		let mut num_rows = 0;

		{
			let data = &mut self.elems.as_mut_slice()[l].data;
			for (x1, x2) in x1.zip(x2)
			{
				data.write_data(x1);
				data.write_data(x2);
				num_rows += 1;
			}
		}

		self.write_common_commands(l, num_rows, 2, plot_type, Record, false, options);
	}

	pub fn plot3<T1: DataType, X1: Iterator<T1>,
			     T2: DataType, X2: Iterator<T2>,
			     T3: DataType, X3: Iterator<T3>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, x3: X3, options: &[PlotOption])
	{
		let l = self.elems.len();
		self.elems.push(PlotElement::new());
		let mut num_rows = 0;

		{
			let data = &mut self.elems.as_mut_slice()[l].data;
			for ((x1, x2), x3) in x1.zip(x2).zip(x3)
			{
				data.write_data(x1);
				data.write_data(x2);
				data.write_data(x3);
				num_rows += 1;
			}
		}

		self.write_common_commands(l, num_rows, 3, plot_type, Record, false, options);
	}

	pub fn plot_matrix<T: DataType, X: Iterator<T>>(&mut self, plot_type: PlotType, is_3d: bool, mut mat: X, num_rows: uint, num_cols: uint,
	                                                dimensions: Option<(f64, f64, f64, f64)>, options: &[PlotOption])
	{
		let l = self.elems.len();
		self.elems.push(PlotElement::new());
		
		{
			let mut count = 0;
			let data = &mut self.elems.as_mut_slice()[l].data;
			for x in mat
			{
				data.write_data(x);
				count += 1;
			}
			
			if count < num_rows * num_cols
			{
				for _ in range(0, num_rows * num_cols - count)
				{
					use std::f64;
					data.write_data(f64::NAN);
				}
			}
		}
		
		let source_type = match dimensions
		{
			Some((x1, y1, x2, y2)) => SizedArray(x1, y1, x2, y2),
			None => Array
		};
		self.write_common_commands(l, num_rows, num_cols, plot_type, source_type, is_3d, options);
	}

	fn write_common_commands(&mut self, elem_idx: uint, num_rows: uint, num_cols: uint, plot_type: PlotType,
	                         source_type: DataSourceType, is_3d: bool, options: &[PlotOption])
	{
		let args = &mut self.elems.as_mut_slice()[elem_idx].args as &mut Writer;
		match source_type
		{
			Record => 
			{
				write!(args, r#" "-" binary endian=little record={} format="%float64" using "#, num_rows);
			
				let mut col_idx = 1;
				while col_idx < num_cols + 1
				{
					write!(args, "{}", col_idx);
					if col_idx < num_cols
					{
						args.write_str(":");
					}
					col_idx += 1;
				}
			},
			_ =>
			{
				write!(args, r#" "-" binary endian=little array=({},{}) format="%float64" "#, num_cols, num_rows);
				
				match source_type
				{
					SizedArray(x1, y1, x2, y2) =>
					{
						let (x1, x2) = if x1 > x2
						{
							(x2, x1)
						}
						else
						{
							(x1, x2)
						};
						
						let (y1, y2) = if y1 > y2
						{
							(y2, y1)
						}
						else
						{
							(y1, y2)
						};
						write!(args, "origin=({:.12e},{:.12e}", x1, y1);
						if is_3d
						{
							write!(args, ",0");
						}
						write!(args, ") ");
						if num_cols > 1
						{
							write!(args, "dx={:.12e} ", (x2 - x1) / (num_cols as f64 - 1.0));
						}
						else
						{
							write!(args, "dx=1 ");
						}
						if num_rows > 1
						{
							write!(args, "dy={:.12e} ", (y2 - y1) / (num_rows as f64 - 1.0));
						}
						else
						{
							write!(args, "dy=1 ");
						}
					},
					_ => ()
				}
			}
		}

		args.write_str(" with ");
		let type_str = match plot_type
		{
			Lines => "lines",
			Points => "points",
			LinesPoints => "linespoints",
			XErrorLines => "xerrorlines",
			YErrorLines => "yerrorlines",
			FillBetween => "filledcurves",
			Boxes => "boxes",
			Pm3D => "pm3d",
			Image => "image",
		};
		args.write_str(type_str);

		if plot_type.is_fill()
		{
			match plot_type
			{
				FillBetween =>
				{
					let mut found = false;
					first_opt!(options,
						FillRegion(d) =>
						{
							found = true;
							args.write_str(match d
							{
								Above => " above",
								Below => " below",
								Between => " closed",
							});
						}
					)
					if !found
					{
						args.write_str(" closed");
					}
				},
				_ => ()
			}

			args.write_str(" fill transparent solid ");

			first_opt!(options,
				FillAlpha(a) =>
				{
					write!(args, "{:.12e}", a);
				}
			)

			if plot_type.is_line()
			{
				args.write_str(" border");
				first_opt!(options,
					BorderColor(s) =>
					{
						write!(args, r#" rgb "{}""#, s);
					}
				)
			}
			else
			{
				args.write_str(" noborder");
			}
		}

		if plot_type.is_line()
		{
			AxesCommonData::write_line_options(args, options);
		}

		if plot_type.is_points()
		{
			first_opt!(options,
				PointSymbol(s) =>
				{
					write!(args, " pt {}", char_to_symbol(s));
				}
			)

			first_opt!(options,
				PointSize(z) =>
				{
					write!(args, " ps {}", z);
				}
			)
		}

		AxesCommonData::write_color_options(args, options, None);

		args.write_str(" t \"");
		first_opt!(options,
			Caption(s) =>
			{
				args.write_str(s);
			}
		)
		args.write_str("\"");
	}

	pub fn write_out_commands(&self, writer: &mut Writer)
	{
		writer.write(self.commands.get_ref());
		self.x_axis.write_out_commands(writer);
		self.y_axis.write_out_commands(writer);
	}

	pub fn write_out_elements(&self, cmd: &str, writer: &mut Writer)
	{
		write!(writer, "{}", cmd);

		let mut first = true;
		for e in self.elems.iter()
		{
			if !first
			{
				write!(writer, ",");
			}
			writer.write(e.args.get_ref());
			first = false;
		}

		write!(writer, "\n");

		for e in self.elems.iter()
		{
			writer.write(e.data.get_ref());
		}
	}

	pub fn set_label_common(&mut self, label_type: LabelType, text: &str, options: &[LabelOption])
	{
		let c = &mut self.commands;

		c.write_str("set ");

		let label_str = match label_type
		{
			XLabel => "xlabel",
			YLabel => "ylabel",
			ZLabel => "zlabel",
			TitleLabel => "title",
			Label(..) => "label",
			_ => panic!("Invalid label type")
		};
		c.write_str(label_str);

		c.write_str(" \"");
		c.write_str(text);
		c.write_str("\"");

		write_out_label_options(label_type, options, c);

		c.write_str("\n");
	}
}

#[doc(hidden)]
pub trait AxesCommonPrivate
{
	fn get_common_data<'l>(&'l self) -> &'l AxesCommonData;
	fn get_common_data_mut<'l>(&'l mut self) -> &'l mut AxesCommonData;
}

pub trait AxesCommon : AxesCommonPrivate
{
	/// Set the position of the axes on the figure using grid coordinates.
	/// # Arguments
	/// * `nrow` - Number of rows in the grid. Must be greater than 0.
	/// * `ncol` - Number of columns in the grid. Must be greater than 0.
	/// * `pos` - Which grid cell to place this axes in, counting from top-left corner,
	///           going left and then down, starting at 0.
	fn set_pos_grid<'l>(&'l mut self, nrow: u32, ncol: u32, pos: u32) -> &'l mut Self
	{
		assert!(nrow > 0);
		assert!(ncol > 0);
		assert!(pos < nrow * ncol);
		{
			let c = self.get_common_data_mut();
			c.grid_rows = nrow;
			c.grid_cols = ncol;
			c.grid_pos = Some(pos);
		}
		self
	}

	/// Set the position of the axes on the figure using screen coordinates.
	/// The coordinates refer to the bottom-left corner of the axes
	/// # Arguments
	/// * `x` - X position. Ranges from 0 to 1
	/// * `y` - Y position. Ranges from 0 to 1
	fn set_pos<'l>(&'l mut self, x: f64, y: f64) -> &'l mut Self
	{
		self.get_common_data_mut().grid_pos = None;
		writeln!(&mut self.get_common_data_mut().commands, "set origin {:.12e},{:.12e}", x, y);
		self
	}

	/// Set the size of the axes
	/// # Arguments
	/// * `w` - Width. Ranges from 0 to 1
	/// * `h` - Height. Ranges from 0 to 1
	fn set_size<'l>(&'l mut self, w: f64, h: f64) -> &'l mut Self
	{
		writeln!(&mut self.get_common_data_mut().commands, "set size {:.12e},{:.12e}", w, h);
		self
	}

	/// Set the aspect ratio of the axes
	/// # Arguments
	/// * `ratio` - The aspect ratio. Set to Auto to return the ratio to default
	fn set_aspect_ratio<'l>(&'l mut self, ratio: AutoOption<f64>) -> &'l mut Self
	{
		{
			let c = &mut self.get_common_data_mut().commands as &mut Writer;

			match ratio
			{
				Fix(r) =>
				{
					writeln!(c, "set size ratio {:.12e}", r);
				},
				Auto =>
				{
					writeln!(c, "set size noratio");
				}
			}
		}
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
	fn set_x_label<'l>(&'l mut self, text: &str, options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_label_common(XLabel, text, options);
		self
	}

	/// Set the label for the Y axis
	/// # Arguments
	/// * `text` - Text of the label. Pass an empty string to hide the label
	/// * `options` - Array of LabelOption controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_y_label<'l>(&'l mut self, text: &str, options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_label_common(YLabel, text, options);
		self
	}

	/// Set the title for the axes
	/// # Arguments
	/// * `text` - Text of the title. Pass an empty string to hide the title
	/// * `options` - Array of LabelOption controlling the appearance of the title. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_title<'l>(&'l mut self, text: &str, options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_label_common(TitleLabel, text, options);
		self
	}

	/// Adds a label to the plot, with an optional marker.
	/// # Arguments
	/// * `text` - Text of the label
	/// * `x` - X coordinate of the label
	/// * `y` - Y coordinate of the label
	/// * `options` - Array of LabelOption controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	///      * `MarkerSymbol` - Specifies the symbol for the marker. Omit to hide the marker
	///      * `MarkerSize` - Specifies the size for the marker
	///      * `MarkerColor` - Specifies the color for the marker
	fn label<'l>(&'l mut self, text: &str, x: Coordinate, y: Coordinate, options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_label_common(Label(x, y), text, options);
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
	/// * `label_options` - Array of LabelOption controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_x_ticks<'l>(&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().x_axis.set_ticks(tick_placement, tick_options, label_options);
		self
	}

	/// Like `set_x_ticks` but for the Y axis.
	fn set_y_ticks<'l>(&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().y_axis.set_ticks(tick_placement, tick_options, label_options);
		self
	}

	/// Sets ticks on the X axis with specified labels at specified positions.
	///
	/// # Arguments
	///
	/// * `ticks` - Iterator specifying the locations and labels of the added ticks.
	///     The label can contain a single C printf style floating point formatting specifier which will be replaced by the
	///     location of the tic.
	/// * `tick_options` - Array of TickOption controlling the appearance of the ticks
	/// * `label_options` - Array of LabelOption controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_x_ticks_custom<'l, T: DataType, TL: Iterator<Tick<T>>>(&'l mut self, ticks: TL, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().x_axis.set_ticks_custom(ticks, tick_options, label_options);
		self
	}

	/// Like `set_x_ticks_custom` but for the the Y axis.
	fn set_y_ticks_custom<'l, T: DataType, TL: Iterator<Tick<T>>>(&'l mut self, ticks: TL, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().y_axis.set_ticks_custom(ticks, tick_options, label_options);
		self
	}

	/// Set the range of values for the X axis.
	///
	/// # Arguments
	/// * `min` - Minimum X value
	/// * `max` - Maximum X value
	fn set_x_range<'l>(&'l mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &'l mut Self
	{
		self.get_common_data_mut().x_axis.set_range(min, max);
		self
	}

	/// Set the range of values for the Y axis.
	///
	/// # Arguments
	/// * `min` - Minimum Y value
	/// * `max` - Maximum Y value
	fn set_y_range<'l>(&'l mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &'l mut Self
	{
		self.get_common_data_mut().y_axis.set_range(min, max);
		self
	}

	/// Sets the X axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_x_log<'l>(&'l mut self, base: Option<f64>) -> &'l mut Self
	{
		self.get_common_data_mut().x_axis.set_log(base);
		self
	}

	/// Sets the Y axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	fn set_y_log<'l>(&'l mut self, base: Option<f64>) -> &'l mut Self
	{
		self.get_common_data_mut().y_axis.set_log(base);
		self
	}
}
