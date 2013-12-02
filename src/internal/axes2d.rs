// Copyright (c) 2013 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;
use datatype::*;
use internal::coordinates::*;
use options::*;
use writer::*;

/// 2D axes that is used for drawing 2D plots
pub struct Axes2D
{
	priv common: AxesCommon,
}

impl Axes2D
{
	/// Set the position of the axes on the figure using grid coordinates
	/// # Arguments
	/// * `row` - Row on the grid. Top-most row is 1
	/// * `column` - Column on the grid. Left-most column is 1
	pub fn set_pos_grid<'l>(&'l mut self, row: u32, col: u32) -> &'l mut Axes2D
	{
		self.common.grid_row = row;
		self.common.grid_col = col;
		self
	}

	/// Set the position of the axes on the figure using screen coordinates.
	/// The coordinates refer to the bottom-left corner of the axes
	/// # Arguments
	/// * `x` - X position. Ranges from 0 to 1
	/// * `y` - Y position. Ranges from 0 to 1
	pub fn set_pos<'l>(&'l mut self, x: f64, y: f64) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

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
	pub fn set_size<'l>(&'l mut self, w: f64, h: f64) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

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
	pub fn set_aspect_ratio<'l>(&'l mut self, ratio: AutoOption<f64>) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

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
	pub fn set_x_label<'l>(&'l mut self, text: &str, options: &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(XLabel, text, options)
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
	pub fn set_y_label<'l>(&'l mut self, text: &str, options: &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(YLabel, text, options)
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
	pub fn set_title<'l>(&'l mut self, text: &str, options: &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(TitleLabel, text, options)
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
	pub fn label<'l>(&'l mut self, text: &str, x: Coordinate, y: Coordinate, options: &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(Label(x, y), text, options)
	}

	fn set_label_common<'l>(&'l mut self, label_type: LabelType, text: &str, options: &[LabelOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

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
		self
	}

	fn set_ticks_common<'l>(&'l mut self, tick_type: TickType, min: AutoOption<f64>, incr: Option<f64>, max: AutoOption<f64>, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

			let mut minor_intervals: u32 = 0;
			first_opt!(tick_options,
				MinorIntervals(i) =>
				{
					minor_intervals = i;
				}
			)

			c.write_str("set m");
			c.write_str(tick_type.to_str());
			c.write_str(" ");
			c.write_int(minor_intervals as i32);
			c.write_str("\n");

			c.write_str("set ");
			c.write_str(tick_type.to_str());

			incr.map(|incr|
			{
				if incr <= 0.0
				{
					fail!("'incr' must be positive, but is actually {}", incr);
				}
				c.write_str(" add ");
				match (min, max)
				{
					(Auto, Auto) =>
					{
						c.write_float(incr);
					},
					(Fix(min), Auto) =>
					{
						c.write_float(min);
						c.write_str(",");
						c.write_float(incr);
					},
					(Auto, Fix(max)) =>
					{
						/* A possible bug in gnuplot */
						c.write_float(incr);
						let _ = max;
					},
					(Fix(min), Fix(max)) =>
					{
						let (min, max) = if min > max
						{
							(max, min)
						}
						else
						{
							(min, max)
						};
						c.write_float(min);
						c.write_str(",");
						c.write_float(incr);
						c.write_str(",");
						c.write_float(max);
					}
				}
			});

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

			c.write_str("\n");
		}
		self
	}

	/// Sets the properties of the tics on the X axis. The first 3 arguments specify the range of the tics. The following combinations work for `(min, max)`:
	///
	/// * `Auto, Auto` - The tics span the entire axis range
	/// * `Fix, Auto` - The tics start at the specified location, and extend to positive infinity
	/// * `Fix, Fix` - The tics span a limited range
	///
	/// Pass `None` for `incr` to disable the automatically generated tics.
	///
	/// # Arguments
	/// * `min` - Sets the location of where the tics start
	/// * `incr` - Sets the spacing between the major tics.
	/// * `max` - Sets the location of where the tics end
	/// * `tick_options` - Array of TickOption controlling the appearance of the ticks
	/// * `label_options` - Array of LabelOption controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	pub fn set_x_tics<'l>(&'l mut self, min: AutoOption<f64>, incr: Option<f64>, max: AutoOption<f64>, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_ticks_common(XTics, min, incr, max, tick_options, label_options)
	}

	/// Like `set_x_tics` but for the Y axis.
	pub fn set_y_tics<'l>(&'l mut self, min: AutoOption<f64>, incr: Option<f64>, max: AutoOption<f64>, tick_options: &[TickOption], label_options: &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_ticks_common(YTics, min, incr, max, tick_options, label_options)
	}

	fn add_tics_common<'l, T: DataType>(&'l mut self, tick_type: TickType, minor: bool, tics: &[(&str, T)]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			c.write_str("set ");
			c.write_str(tick_type.to_str());
			c.write_str(" add (");

			let mut first = true;
			for tic in tics.iter()
			{
				let (ref label, ref pos) = *tic;
				if first
				{
					first = false;
				}
				else
				{
					c.write_str(",");
				}
				c.write_str("\"");
				c.write_str(*label);
				c.write_str("\" ");
				c.write_float(pos.get());
				c.write_str(" ");
				c.write_str(if minor { "1" } else { "0" });
			}
			c.write_str(")\n");
		}
		self
	}

	/// Adds major tics to the X axis with specified labels at specified positions.
	///
	/// # Arguments
	///
	/// * `tics` - Array of tuples specifying the locations and labels of the added tics.
	///     The label can contain a single C printf style floating point formatting specifier which will be replaced by the
	///     location of the tic.
	pub fn add_x_major_tics<'l, T: DataType>(&'l mut self, tics: &[(&str, T)]) -> &'l mut Axes2D
	{
		self.add_tics_common(XTics, false, tics)
	}

	/// Like `set_x_major_tics` but for the minor tics of the X axis.
	pub fn add_x_minor_tics<'l, T: DataType>(&'l mut self, tics: &[(&str, T)]) -> &'l mut Axes2D
	{
		self.add_tics_common(XTics, true, tics)
	}

	/// Like `set_x_major_tics` but for the major tics of the Y axis.
	pub fn add_y_major_tics<'l, T: DataType>(&'l mut self, tics: &[(&str, T)]) -> &'l mut Axes2D
	{
		self.add_tics_common(YTics, false, tics)
	}

	/// Like `set_x_major_tics` but for the minor tics of the Y axis.
	pub fn add_y_minor_tics<'l, T: DataType>(&'l mut self, tics: &[(&str, T)]) -> &'l mut Axes2D
	{
		self.add_tics_common(YTics, true, tics)
	}

	/// Sets the properties of the plot border
	///
	/// # Arguments
	///
	/// * `front` - Whether or not to draw the border above or below the plot contents
	/// * `locations` - Which locations of the border to draw
	/// * `options` - Array of PlotOption controlling the appearance of the border. Relevant options are:
	///      * `Color` - Specifies the color of the border
	///      * `LineStyle` - Specifies the style of the border
	///      * `LineWidth` - Specifies the width of the border
	pub fn set_border<'l>(&'l mut self, front: bool, locations: &[BorderLocation2D], options: &[PlotOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			c.write_str("set border ");
			let mut f: i32 = 0;
			for &l in locations.iter()
			{
				f |= (l as i32);
			}
			c.write_int(f);
			c.write_str( if front
			{
				" front "
			}
			else
			{
				" back "
			});

			AxesCommon::write_color_options(c, options, Some("black"));
			AxesCommon::write_line_options(c, options);

			c.write_str("\n");
		}
		self
	}

	fn set_axis_common<'l>(&'l mut self, axis: &str, show: bool, options: &[PlotOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			if show
			{
				c.write_str("set ");
				c.write_str(axis);
				c.write_str("zeroaxis ");
				AxesCommon::write_color_options(c, options, Some("black"));
				AxesCommon::write_line_options(c, options);
			}
			else
			{
				c.write_str("unset ");
				c.write_str(axis);
				c.write_str("zeroaxis ");
			}

			c.write_str("\n");
		}
		self
	}

	/// Sets the properties of x axis.
	///
	/// # Arguments
	///
	/// * `show` - Whether or not draw the axis
	/// * `options` - Array of PlotOption controlling the appearance of the border. Relevant options are:
	///      * `Color` - Specifies the color of the border
	///      * `LineStyle` - Specifies the style of the border
	///      * `LineWidth` - Specifies the width of the border
	pub fn set_x_axis<'l>(&'l mut self, show: bool, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.set_axis_common("x", show, options)
	}
	
	/// Like `set_x_axis` but for the y axis.
	pub fn set_y_axis<'l>(&'l mut self, show: bool, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.set_axis_common("y", show, options)
	}

	/// Adds an arrow to the plot. The arrow is drawn from `(x1, y1)` to `(x2, y2)` with the arrow point towards `(x2, y2)`.
	/// # Arguments
	/// * `x1` - X coordinate of the arrow start
	/// * `y1` - Y coordinate of the arrow start
	/// * `x2` - X coordinate of the arrow end
	/// * `y2` - Y coordinate of the arrow end
	/// * `options` - Array of PlotOption controlling the appearance of the arrowhead and arrow shaft. Relevant options are:
	///      * `ArrowType` - Specifies the style of the arrow head (or an option to omit it)
	///      * `ArrowSize` - Sets the size of the arrow head (in graph units)
	///      * `Color` - Specifies the color of the arrow
	///      * `LineStyle` - Specifies the style of the arrow shaft
	///      * `LineWidth` - Specifies the width of the arrow shaft
	pub fn arrow<'l>(&'l mut self, x1: Coordinate, y1: Coordinate, x2: Coordinate, y2: Coordinate, options: &[PlotOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			c.write_str("set arrow from ");
			x1.write(c);
			c.write_str(",");
			y1.write(c);
			c.write_str(" to ");
			x2.write(c);
			c.write_str(",");
			y2.write(c);

			first_opt!(options,
				ArrowType(s) =>
				{
					c.write_str(match(s)
					{
						Open => "",
						Closed => " empty",
						Filled => " filled",
						NoArrow => " nohead",
					});
				}
			)

			c.write_str(" size graph ");
			first_opt_default!(options,
				ArrowSize(s) =>
				{
					c.write_float(s);
				},
				_ =>
				{
					c.write_str("0.05");
				}
			)
			c.write_str(",12");

			AxesCommon::write_color_options(c, options, Some("black"));
			AxesCommon::write_line_options(c, options);

			c.write_str("\n");
		}
		self
	}

	/// Set the range of values for the X axis
	/// # Arguments
	/// * `min` - Minimum X value
	/// * `max` - Maximum X value
	pub fn set_x_range<'l>(&'l mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

			c.write_str("set xrange [");
			match min
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str(":");
			match max
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str("]\n");
		}
		self
	}

	/// Set the range of values for the Y axis
	/// # Arguments
	/// * `min` - Minimum Y value
	/// * `max` - Maximum Y value
	pub fn set_y_range<'l>(&'l mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;

			c.write_str("set yrange [");
			match min
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str(":");
			match max
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str("]\n");
		}
		self
	}

	/// Specifies the location and other properties of the legend
	/// # Arguments
	/// * `x` - X coordinate of the legend
	/// * `y` - Y coordinate of the legend
	/// * `label_options` - Array of LegendOption options
	/// * `text_options` - Array of LabelOption options specifying the appearance of the plot titles. Valid options are:
	///     * `Font`
	///     * `TextColor`
	///     * `TextAlign(AlignLeft)`
	///     * `TextAlign(AlignRight)`
	pub fn set_legend<'l>(&'l mut self, x: Coordinate, y: Coordinate, legend_options: &'l [LegendOption], text_options: &'l [LabelOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set key at");
			x.write(c);
			c.write_str(",");
			y.write(c);
			
			first_opt_default!(legend_options,
				Placement(h, v) =>
				{
					c.write_str(match h
					{
						AlignLeft => " left",
						AlignRight => " right",
						_ => " center"
					});
					c.write_str(match v
					{
						AlignTop => " top",
						AlignBottom => " bottom",
						_ => " center"
					});				},
				_ =>
				{
					c.write_str(" right top");
				}
			)
			
			first_opt_default!(legend_options,
				Horizontal =>
				{
					c.write_str(" horizontal");
				},
				_ =>
				{
					c.write_str(" vertical");
				}
			)
			
			first_opt_default!(legend_options,
				Reverse =>
				{
					c.write_str(" reverse");
				},
				_ =>
				{
					c.write_str(" noreverse");
				}
			)
			
			first_opt_default!(legend_options,
				Invert =>
				{
					c.write_str(" invert");
				},
				_ =>
				{
					c.write_str(" noinvert");
				}
			)
			
			first_opt!(legend_options,
				Title(s) =>
				{
					c.write_str(" title \"");
					c.write_str(s);
					c.write_str("\"");
				}
			)
			
			first_opt!(text_options,
				Font(f, s) =>
				{
					c.write_str(" font \"");
					c.write_str(f);
					c.write_str(",");
					c.write_str(s.to_str());
					c.write_str("\"");
				}
			)
			first_opt!(text_options,
				TextColor(s) =>
				{
					c.write_str(" textcolor rgb \"");
					c.write_str(s);
					c.write_str("\"");
				}
			)
			first_opt!(text_options,
				TextAlign(a) =>
				{
					c.write_str(match(a)
					{
						AlignLeft => " Left",
						AlignRight => " Right",
						_ => ""
					});
				}
			)
			
			first_opt!(legend_options,
				MaxRows(r) =>
				{
					c.write_str(" maxrows ");
					c.write_int(r as i32);
				}
			)
			
			first_opt!(legend_options,
				MaxCols(l) =>
				{
					c.write_str(" maxcols ");
					c.write_int(l as i32);
				}
			)
			
			c.write_str("\n");
		}
		self
	}

	/// Plot a 2D scatter-plot with lines connecting each data point
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn lines<'l, Tx: DataType, X: Iterator<Tx>, Ty: DataType, Y: Iterator<Ty>>(&'l mut self, x: X, y: Y, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Lines, x, y, options);
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `Color` - Sets the color
	pub fn points<'l, Tx: DataType, X: Iterator<Tx>, Ty: DataType, Y: Iterator<Ty>>(&'l mut self, x: X, y: Y, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Points, x, y, options);
		self
	}

	/// A combination of lines and points methods (drawn in that order).
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element
	pub fn lines_points<'l, Tx: DataType, X: Iterator<Tx>, Ty: DataType, Y: Iterator<Ty>>(&'l mut self, x: X, y: Y, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(LinesPoints, x, y, options);
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point and lines connecting each data point.
	/// Additionally, error bars are attached to each data point in the X direction.
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y valuess
	/// * `x_error` - Iterator for the error associated with the x value
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn x_error_lines<'l,
	                   Tx: DataType, X: Iterator<Tx>,
	                   Ty: DataType, Y: Iterator<Ty>,
	                   Txe: DataType, XE: Iterator<Txe>>(&'l mut self, x: X, y: Y, x_error: XE, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(XErrorLines, x, y, x_error, options);
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point and lines connecting each data point.
	/// Additionally, error bars are attached to each data point in the Y direction.
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `y_error` - Iterator for the error associated with the y values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn y_error_lines<'l,
	                   Tx: DataType, X: Iterator<Tx>,
	                   Ty: DataType, Y: Iterator<Ty>,
	                   Tye: DataType, YE: Iterator<Tye>>(&'l mut self, x: X, y: Y, y_error: YE, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(YErrorLines, x, y, y_error, options);
		self
	}

	/// Plot a 2D scatter-plot of two curves (bound by `y_lo` and `y_hi`) with a filled region between them.
	/// `FillRegion` plot option can be used to control what happens when the curves intersect. If set to Above, then the `y_lo < y_hi` region is filled.
	/// If set to Below, then the `y_lo > y_hi` region is filled. Otherwise both regions are filled.
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y_lo` - Iterator for the bottom y values
	/// * `y_hi` - Iterator for the top y values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `FillRegion` - Specifies the region between the two curves to fill
	///     * `Color` - Sets the color of the filled region
	///     * `FillAlpha` - Sets the transparency of the filled region
	pub fn fill_between<'l,
	                   Tx: DataType, X: Iterator<Tx>,
	                   Tyl: DataType, YL: Iterator<Tyl>,
	                   Tyh: DataType, YH: Iterator<Tyh>>(&'l mut self, x: X, y_lo: YL, y_hi: YH, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(FillBetween, x, y_lo, y_hi, options);
		self
	}

	/// Plot a 2D scatter-plot using boxes of automatic width. Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// # Arguments
	/// * `x` - Iterator for the x values (center of the box)
	/// * `y` - Iterator for the y values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn boxes<'l, Tx: DataType, X: Iterator<Tx>, Ty: DataType, Y: Iterator<Ty>>(&'l mut self, x: X, y: Y, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Boxes, x, y, options);
		self
	}

	/// Plot a 2D scatter-plot using boxes of set (per box) width.
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// # Arguments
	/// * `x` - Iterator for the x values (center of the box)
	/// * `y` - Iterator for the y values
	/// * `w` - Iterator for the box width values
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn boxes_set_width<'l,
	                      Tx: DataType,
	                      X: Iterator<Tx>,
	                      Ty: DataType,
	                      Y: Iterator<Ty>,
	                      Tw: DataType,
	                      W: Iterator<Tw>>(&'l mut self, x: X, y: Y, w: W, options: &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(Boxes, x, y, w, options);
		self
	}
}

struct Tics
{
	common_options: ~[u8],
	text_options: ~[u8],
}

pub fn new_axes2d() -> Axes2D
{
	Axes2D{common: AxesCommon::new()}
}

pub trait Axes2DPrivate
{
	fn plot2<T1: DataType, X1: Iterator<T1>, T2: DataType, X2: Iterator<T2>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, options: &[PlotOption]);
	fn plot3<T1: DataType, X1: Iterator<T1>, T2: DataType, X2: Iterator<T2>, T3: DataType, X3: Iterator<T3>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, x3: X3, options: &[PlotOption]);
	fn write_out(&self, writer: |data: &[u8]|);
	fn get_common<'l>(&'l self) -> &'l AxesCommon;
}

impl Axes2DPrivate for Axes2D
{
	fn plot2<T1: DataType, X1: Iterator<T1>,
			 T2: DataType, X2: Iterator<T2>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, options: &[PlotOption])
	{
		let l = self.common.elems.len();
		self.common.elems.push(PlotElement::new());
		let mut num_rows: i32 = 0;

		{
			let data = &mut self.common.elems[l].data;
			for (x1, x2) in x1.zip(x2)
			{
				data.write_data(x1);
				data.write_data(x2);
				num_rows += 1;
			}
		}

		self.common.write_common_commands(l, num_rows, 2, plot_type, options);
	}

	fn plot3<T1: DataType, X1: Iterator<T1>,
			 T2: DataType, X2: Iterator<T2>,
			 T3: DataType, X3: Iterator<T3>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, x3: X3, options: &[PlotOption])
	{
		let l = self.common.elems.len();
		self.common.elems.push(PlotElement::new());
		let mut num_rows: i32 = 0;

		{
			let data = &mut self.common.elems[l].data;
			for ((x1, x2), x3) in x1.zip(x2).zip(x3)
			{
				data.write_data(x1);
				data.write_data(x2);
				data.write_data(x3);
				num_rows += 1;
			}
		}

		self.common.write_common_commands(l, num_rows, 3, plot_type, options);
	}

	fn write_out(&self, writer: |data: &[u8]|)
	{
		if self.common.elems.len() == 0
		{
			return;
		}

		writer(self.common.commands);

		writer("plot".as_bytes());

		let mut first = true;
		for e in self.common.elems.iter()
		{
			if !first
			{
				writer(",".as_bytes());
			}
			writer(e.args);
			first = false;
		}

		writer("\n".as_bytes());

		for e in self.common.elems.iter()
		{
			writer(e.data);
		}
	}

	fn get_common<'l>(&'l self) -> &'l AxesCommon
	{
		&self.common
	}
}
