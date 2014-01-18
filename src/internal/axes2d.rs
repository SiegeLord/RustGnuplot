// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::io::Writer;

use axes_common::*;
use datatype::*;
use internal::coordinates::*;
use options::*;
use writer::*;

/// 2D axes that is used for drawing 2D plots
pub struct Axes2D
{
	priv common: AxesCommonData,
}

impl Axes2D
{
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
			c.write_i32(f);
			c.write_str( if front
			{
				" front "
			}
			else
			{
				" back "
			});

			AxesCommonData::write_color_options(c, options, Some("black"));
			AxesCommonData::write_line_options(c, options);

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
				AxesCommonData::write_color_options(c, options, Some("black"));
				AxesCommonData::write_line_options(c, options);
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

			AxesCommonData::write_color_options(c, options, Some("black"));
			AxesCommonData::write_line_options(c, options);

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
					c.write_i32(r as i32);
				}
			)

			first_opt!(legend_options,
				MaxCols(l) =>
				{
					c.write_str(" maxcols ");
					c.write_i32(l as i32);
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

impl AxesCommon for Axes2D
{
	fn get_common_data_mut<'l>(&'l mut self) -> &'l mut AxesCommonData
	{
		&mut self.common
	}

	fn get_common_data<'l>(&'l self) -> &'l AxesCommonData
	{
		&self.common
	}
}

pub fn new_axes2d() -> Axes2D
{
	Axes2D
	{
		common: AxesCommonData::new(),
	}
}

pub trait Axes2DPrivate
{
	fn plot2<T1: DataType, X1: Iterator<T1>, T2: DataType, X2: Iterator<T2>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, options: &[PlotOption]);
	fn plot3<T1: DataType, X1: Iterator<T1>, T2: DataType, X2: Iterator<T2>, T3: DataType, X3: Iterator<T3>>(&mut self, plot_type: PlotType, x1: X1, x2: X2, x3: X3, options: &[PlotOption]);
	fn write_out(&self, writer: &mut Writer);
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

	fn write_out(&self, writer: &mut Writer)
	{
		if self.common.elems.len() == 0
		{
			return;
		}

		self.common.write_out_commands(writer);

		self.common.write_out_elements("plot", writer);
	}
}
