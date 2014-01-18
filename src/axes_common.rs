// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::io::{MemWriter, SeekSet, Writer};

use datatype::*;
use internal::coordinates::*;
use options::*;
use writer::*;

pub struct PlotElement
{
	args: MemWriter,
	data: MemWriter
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
	let w = writer;

	match label_type
	{
		Label(x, y) =>
		{
			w.write_str(" at ");
			x.write(w);
			w.write_str(",");
			y.write(w);
			w.write_str(" front");
		}
		_ => ()
	}

	first_opt!(options,
		TextOffset(x, y) =>
		{
			w.write_str(" offset character ");
			w.write_float(x);
			w.write_str(",");
			w.write_float(y);
		}
	)

	first_opt!(options,
		TextColor(s) =>
		{
			w.write_str(" tc rgb \"");
			w.write_str(s);
			w.write_str("\"");
		}
	)

	first_opt!(options,
		Font(f, s) =>
		{
			w.write_str(" font \"");
			w.write_str(f);
			w.write_str(",");
			w.write_str(s.to_str());
			w.write_str("\"");
		}
	)

	first_opt!(options,
		Rotate(a) =>
		{
			w.write_str(" rotate by ");
			w.write_float(a);
		}
	)

	if label_type.is_label()
	{
		let mut have_point = false;
		first_opt!(options,
			MarkerSymbol(s) =>
			{
				w.write_str(" point pt ");
				w.write_i32(char_to_symbol(s));
				have_point = true;
			}
		)

		if have_point
		{
			first_opt!(options,
				MarkerColor(s) =>
				{
					w.write_str(" lc rgb \"");
					w.write_str(s);
					w.write_str("\"");
				}
			)

			first_opt!(options,
				MarkerSize(z) =>
				{
					w.write_str(" ps ");
					w.write_float(z);
					w.write_str("");
				}
			)
		}

		first_opt!(options,
			TextAlign(a) =>
			{
				w.write_str(match(a)
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
	XTicks,
	YTicks,
}

impl TickAxis
{
	pub fn to_str(&self) -> &str
	{
		match *self
		{
			XTicks => "xtics",
			YTicks => "ytics",
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

pub struct AxesCommonData
{
	commands: MemWriter,
	elems: ~[PlotElement],
	grid_row: u32,
	grid_col: u32,
	x_ticks: MemWriter,
	y_ticks: MemWriter,
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
		a => fail!("Invalid symbol {}", a)
	}
}

impl AxesCommonData
{
	pub fn new() -> AxesCommonData
	{
		AxesCommonData
		{
			commands: MemWriter::new(),
			elems: ~[],
			grid_row: 0,
			grid_col: 0,
			x_ticks: MemWriter::new(),
			y_ticks: MemWriter::new()
		}
	}

	pub fn write_line_options(c: &mut MemWriter, options: &[PlotOption])
	{
		let mut found = false;
		c.write_str(" lw ");
		first_opt!(options,
			LineWidth(w) =>
			{
				c.write_float(w);
				found = true;
			}
		)
		if !found
		{
			c.write_float(1.0);
		}

		c.write_str(" lt ");
		let mut found = false;
		first_opt!(options,
			LineStyle(d) =>
			{
				c.write_i32(d.to_int());
				found = true;
			}
		)
		if !found
		{
			c.write_i32(1);
		}
	}

	pub fn write_color_options<'l>(c: &mut MemWriter, options: &[PlotOption<'l>], default: Option<&'l str>)
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
				c.write_str(" lc rgb \"");
				c.write_str(s);
				c.write_str("\"");
			},
			None => ()
		}
	}

	pub fn write_common_commands(&mut self, elem_idx: uint, num_rows: i32, num_cols: i32, plot_type: PlotType, options: &[PlotOption])
	{
		let args = &mut self.elems[elem_idx].args;
		args.write_str(" \"-\" binary endian=little record=");
		args.write_i32(num_rows);
		args.write_str(" format=\"%float64\" using ");

		let mut col_idx: i32 = 1;
		while(col_idx < num_cols + 1)
		{
			args.write_i32(col_idx);
			if(col_idx < num_cols)
			{
				args.write_str(":");
			}
			col_idx += 1;
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
					args.write_float(a);
				}
			)

			if plot_type.is_line()
			{
				args.write_str(" border");
				first_opt!(options,
					BorderColor(s) =>
					{
						args.write_str(" rgb \"");
						args.write_str(s);
						args.write_str("\"");
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
					args.write_str(" pt ");
					args.write_i32(char_to_symbol(s));
				}
			)

			first_opt!(options,
				PointSize(z) =>
				{
					args.write_str(" ps ");
					args.write_float(z);
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
		writer.write(self.x_ticks.get_ref());
		writer.write(self.y_ticks.get_ref());
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

	fn set_label_common(&mut self, label_type: LabelType, text: &str, options: &[LabelOption])
	{
		let c = &mut self.commands;

		c.write_str("set ");

		let label_str = match label_type
		{
			XLabel => "xlabel",
			YLabel => "ylabel",
			TitleLabel => "title",
			Label(..) => "label",
			_ => fail!("Invalid label type")
		};
		c.write_str(label_str);

		c.write_str(" \"");
		c.write_str(text);
		c.write_str("\"");

		write_out_label_options(label_type, options, c);

		c.write_str("\n");
	}

	fn set_ticks_custom_common<T: DataType, TL: Iterator<Tick<T>>>(&mut self, tick_axis: TickAxis, mut ticks: TL, tick_options: &[TickOption], label_options: &[LabelOption])
	{
		let c = match tick_axis
		{
			XTicks => &mut self.x_ticks,
			YTicks => &mut self.y_ticks
		};
		c.seek(0, SeekSet);

		c.write_str("set ");
		c.write_str(tick_axis.to_str());
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

			let (ref pos, ref label, level) = match tick
			{
				Minor(ref pos) =>
				{
					(pos, &Auto, 1)
				},
				Major(ref pos, ref label) =>
				{
					(pos, label, 0)
				}
			};

			match **label
			{
				Fix(ref label) =>
				{
					c.write_str("\"");
					c.write_str(*label);
					c.write_str("\" ");
				},
				Auto => ()
			}
			c.write_float(pos.get());
			c.write_str(" ");
			c.write_i32(level);
		}
		c.write_str(")");
		AxesCommonData::set_ticks_options(c, tick_options, label_options);
		c.write_str("\n");
	}

	fn set_ticks_options(c: &mut MemWriter, tick_options: &[TickOption], label_options: &[LabelOption])
	{
		write_out_label_options(AxesTicks, label_options, c);

		first_opt!(tick_options,
			OnAxis(b) =>
			{
				c.write_str(match(b)
				{
					true => " axis",
					false => " border",
				});
			}
		)

		first_opt!(tick_options,
			Mirror(b) =>
			{
				c.write_str(match(b)
				{
					true => " mirror",
					false => " nomirror",
				});
			}
		)

		first_opt!(tick_options,
			Inward(b) =>
			{
				c.write_str(match(b)
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

		c.write_str(" scale ");
		c.write_float(minor_scale);
		c.write_str(",");
		c.write_float(major_scale);
	}

	fn set_ticks_common(&mut self, tick_axis: TickAxis, incr: AutoOption<f64>, minor_intervals: u32, tick_options: &[TickOption], label_options: &[LabelOption])
	{
		let c = match tick_axis
		{
			XTicks => &mut self.x_ticks,
			YTicks => &mut self.y_ticks
		};
		c.seek(0, SeekSet);

		c.write_str("set m");
		c.write_str(tick_axis.to_str());
		c.write_str(" ");
		c.write_i32(minor_intervals as i32);
		c.write_str("\n");

		c.write_str("set ");
		c.write_str(tick_axis.to_str());

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
					fail!("'incr' must be positive, but is actually {}", incr);
				}
				c.write_str(" ");
				c.write_float(incr);
			}
		}

		AxesCommonData::set_ticks_options(c, tick_options, label_options);
		c.write_str("\n");
	}
}

pub trait AxesCommonPrivate
{
	fn get_common_data<'l>(&'l self) -> &'l AxesCommonData;
	fn get_common_data_mut<'l>(&'l mut self) -> &'l mut AxesCommonData;
}

pub trait AxesCommon : AxesCommonPrivate
{
	/// Set the position of the axes on the figure using grid coordinates
	/// # Arguments
	/// * `row` - Row on the grid. Top-most row is 1
	/// * `column` - Column on the grid. Left-most column is 1
	fn set_pos_grid<'l>(&'l mut self, row: u32, col: u32) -> &'l mut Self
	{
		{
			let c = self.get_common_data_mut();
			c.grid_row = row;
			c.grid_col = col;
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
		{
			let c = &mut self.get_common_data_mut().commands;

			c.write_str("set origin ");
			c.write_float(x);
			c.write_str(",");
			c.write_float(y);
			c.write_str("\n");
		}
		self
	}

	/// Set the size of the axes
	/// # Arguments
	/// * `w` - Width. Ranges from 0 to 1
	/// * `h` - Height. Ranges from 0 to 1
	fn set_size<'l>(&'l mut self, w: f64, h: f64) -> &'l mut Self
	{
		{
			let c = &mut self.get_common_data_mut().commands;

			c.write_str("set size ");
			c.write_float(w);
			c.write_str(",");
			c.write_float(h);
			c.write_str("\n");
		}
		self
	}

	/// Set the aspect ratio of the axes
	/// # Arguments
	/// * `ratio` - The aspect ratio. Set to Auto to return the ratio to default
	fn set_aspect_ratio<'l>(&'l mut self, ratio: AutoOption<f64>) -> &'l mut Self
	{
		{
			let c = &mut self.get_common_data_mut().commands;

			match ratio
			{
				Fix(r) =>
				{
					c.write_str("set size ratio ");
					c.write_float(r);
				},
				Auto =>
				{
					c.write_str("set size noratio");
				}
			}
			c.write_str("\n");
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
	/// * `incr` - Sets the spacing between the major ticks. Pass `Auto` to let gnuplot decide the spacing automatically.
	/// * `minor_intervals` - Number of sub-intervals between minor ticks.
	/// * `tick_options` - Array of TickOption controlling the appearance of the ticks
	/// * `label_options` - Array of LabelOption controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	fn set_x_ticks<'l>(&'l mut self, incr: AutoOption<f64>, minor_intervals: u32, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_ticks_common(XTicks, incr, minor_intervals, tick_options, label_options);
		self
	}

	/// Like `set_x_ticks` but for the Y axis.
	fn set_y_ticks<'l>(&'l mut self, incr: AutoOption<f64>, minor_intervals: u32, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_ticks_common(YTicks, incr, minor_intervals, tick_options, label_options);
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
		self.get_common_data_mut().set_ticks_custom_common(XTicks, ticks, tick_options, label_options);
		self
	}

	/// Like `set_x_ticks_custom` but for the the Y axis.
	fn set_y_ticks_custom<'l, T: DataType, TL: Iterator<Tick<T>>>(&'l mut self, ticks: TL, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Self
	{
		self.get_common_data_mut().set_ticks_custom_common(YTicks, ticks, tick_options, label_options);
		self
	}
}
