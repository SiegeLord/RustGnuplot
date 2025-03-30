// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::iter;

use crate::axes_common::*;
use crate::coordinates::*;
use crate::datatype::*;
use crate::options::*;
use crate::util::{escape, OneWayOwned};
use crate::writer::Writer;
use crate::ColorType;

struct LegendData
{
	x: Coordinate,
	y: Coordinate,
	legend_options: Vec<LegendOption<String>>,
	text_options: Vec<LabelOption<String>>,
}

impl LegendData
{
	fn write_out(&self, writer: &mut dyn Writer)
	{
		let w = writer;
		write!(w, "set key at {},{}", self.x, self.y);

		first_opt_default! {self.legend_options,
			Placement(h, v) =>
			{
				w.write_str(match h
				{
					AlignLeft => " left",
					AlignRight => " right",
					_ => " center"
				});
				w.write_str(match v
				{
					AlignTop => " top",
					AlignBottom => " bottom",
					_ => " center"
				});
			},
			_ =>
			{
				w.write_str(" right top");
			}
		}

		first_opt_default! {self.legend_options,
			Horizontal =>
			{
				w.write_str(" horizontal");
			},
			_ =>
			{
				w.write_str(" vertical");
			}
		}

		first_opt_default! {self.legend_options,
			Reverse =>
			{
				w.write_str(" reverse");
			},
			_ =>
			{
				w.write_str(" noreverse");
			}
		}

		first_opt_default! {self.legend_options,
			Invert =>
			{
				w.write_str(" invert");
			},
			_ =>
			{
				w.write_str(" noinvert");
			}
		}

		first_opt! {self.legend_options,
			Title(ref s) =>
			{
				w.write_str(" title \"");
				w.write_str(&escape(s));
				w.write_str("\"");
			}
		}

		first_opt! {self.text_options,
			Font(ref f, s) =>
			{
				w.write_str(" font \"");
				w.write_str(&escape(f));
				w.write_str(",");
				w.write_str(&s.to_string()[..]);
				w.write_str("\"");
			}
		}
		first_opt! {self.text_options,
			TextColor(ref s) =>
			{
				write!(w, " textcolor {} ", s.command());
			}
		}
		first_opt! {self.text_options,
			TextAlign(a) =>
			{
				w.write_str(match a
				{
					AlignLeft => " Left",
					AlignRight => " Right",
					_ => ""
				});
			}
		}

		first_opt! {self.legend_options,
			MaxRows(r) =>
			{
				write!(w, " maxrows {}", r as i32);
			}
		}

		first_opt! {self.legend_options,
			MaxCols(l) =>
			{
				write!(w, " maxcols {}", l as i32);
			}
		}

		w.write_str("\n");
	}

	fn reset_state(&self, writer: &mut dyn Writer)
	{
		writer.write_str("unset key\n");
	}
}

struct ArrowData
{
	x1: Coordinate,
	y1: Coordinate,
	x2: Coordinate,
	y2: Coordinate,
	plot_options: Vec<PlotOption<String>>,
	tag: i32,
}

impl ArrowData
{
	fn write_out(&self, writer: &mut dyn Writer)
	{
		let w = writer;
		write!(
			w,
			"set arrow {} from {},{} to {},{}",
			self.tag, self.x1, self.y1, self.x2, self.y2
		);

		first_opt! {self.plot_options,
			ArrowType(s) =>
			{
				w.write_str(match s
				{
					Open => "",
					Closed => " empty",
					Filled => " filled",
					NoArrow => " nohead",
				});
			}
		}

		w.write_str(" size graph ");
		first_opt_default! {self.plot_options,
			ArrowSize(z) =>
			{
				write!(w, "{:.12e}", z);
			},
			_ =>
			{
				w.write_str("0.05");
			}
		}
		w.write_str(",12");

		AxesCommonData::write_color_options(w, &self.plot_options, false, Some(ColorType::Black));
		AxesCommonData::write_line_options(
			w,
			&self.plot_options,
			GnuplotVersion { major: 0, minor: 0 },
		);

		w.write_str("\n");
	}

	fn reset_state(&self, writer: &mut dyn Writer)
	{
		writeln!(writer, "unset arrow {}", self.tag);
	}
}

struct BorderOptions
{
	front: bool,
	locations: Vec<BorderLocation2D>,
	options: Vec<PlotOption<String>>,
}

impl BorderOptions
{
	fn new() -> BorderOptions
	{
		BorderOptions {
			front: true,
			locations: vec![Bottom, Left, Top, Right],
			options: vec![],
		}
	}

	fn write_out(&self, writer: &mut dyn Writer, version: GnuplotVersion)
	{
		writer.write_str("set border ");
		let mut f: i32 = 0;
		for &l in self.locations.iter()
		{
			f |= l as i32;
		}
		write!(writer, "{}", f);
		writer.write_str(if self.front { " front " } else { " back " });

		AxesCommonData::write_color_options(writer, &self.options, false, Some(ColorType::Black));
		AxesCommonData::write_line_options(writer, &self.options, version);

		writer.write_str("\n");
	}
}

/// 2D axes that is used for drawing 2D plots
pub struct Axes2D
{
	common: AxesCommonData,
	border_options: BorderOptions,
	arrows: Vec<ArrowData>,
	legend: Option<LegendData>,
}

impl Axes2D
{
	pub(crate) fn new() -> Axes2D
	{
		Axes2D {
			common: AxesCommonData::new(),
			border_options: BorderOptions::new(),
			arrows: vec![],
			legend: None,
		}
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
	pub fn set_border<'l>(
		&'l mut self, front: bool, locations: &[BorderLocation2D], options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		self.border_options.front = front;
		self.border_options.locations = locations.into();
		self.border_options.options = options.to_one_way_owned();
		self
	}

	/// Sets the properties of x axis.
	///
	/// # Arguments
	///
	/// * `show` - Whether or not draw the axis
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the axis. Relevant options are:
	///      * `Color` - Specifies the color of the border
	///      * `LineStyle` - Specifies the style of the border
	///      * `LineWidth` - Specifies the width of the border
	pub fn set_x_axis<'l>(&'l mut self, show: bool, options: &[PlotOption<&str>]) -> &'l mut Self
	{
		self.common.x_axis.show = show;
		self.common.x_axis.options = options.to_one_way_owned();
		self
	}

	/// Like `set_x_axis` but for the y axis.
	pub fn set_y_axis<'l>(&'l mut self, show: bool, options: &[PlotOption<&str>]) -> &'l mut Self
	{
		self.common.y_axis.show = show;
		self.common.y_axis.options = options.to_one_way_owned();
		self
	}

	/// Adds an arrow to the plot. The arrow is drawn from `(x1, y1)` to `(x2, y2)` with the arrow point towards `(x2, y2)`.
	/// # Arguments
	/// * `x1` - X coordinate of the arrow start
	/// * `y1` - Y coordinate of the arrow start
	/// * `x2` - X coordinate of the arrow end
	/// * `y2` - Y coordinate of the arrow end
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the arrowhead and arrow shaft. Relevant options are:
	///      * `ArrowType` - Specifies the style of the arrow head (or an option to omit it)
	///      * `ArrowSize` - Sets the size of the arrow head (in graph units)
	///      * `Color` - Specifies the color of the arrow
	///      * `LineStyle` - Specifies the style of the arrow shaft
	///      * `LineWidth` - Specifies the width of the arrow shaft
	pub fn arrow<'l>(
		&'l mut self, x1: Coordinate, y1: Coordinate, x2: Coordinate, y2: Coordinate,
		options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		self.arrows.push(ArrowData {
			x1,
			y1,
			x2,
			y2,
			tag: self.arrows.len() as i32 + 1,
			plot_options: options.to_one_way_owned(),
		});
		self
	}

	/// Specifies the location and other properties of the legend
	/// # Arguments
	/// * `x` - X coordinate of the legend
	/// * `y` - Y coordinate of the legend
	/// * `legend_options` - Array of LegendOption options
	/// * `text_options` - Array of LabelOption options specifying the appearance of the plot titles. Valid options are:
	///     * `Font`
	///     * `TextColor`
	///     * `TextAlign(AlignLeft)`
	///     * `TextAlign(AlignRight)`
	pub fn set_legend<'l>(
		&'l mut self, x: Coordinate, y: Coordinate, legend_options: &[LegendOption<&str>],
		text_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.legend = Some(LegendData {
			x,
			y,
			legend_options: legend_options.to_one_way_owned(),
			text_options: text_options.to_one_way_owned(),
		});
		self
	}

	/// Plot a 2D scatter-plot with lines connecting each data point
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn lines<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
	>(
		&'l mut self, x: X, y: Y, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y);
		self.common.elems.push(PlotElement::new_plot(
			Lines, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `Color` - Sets the color
	pub fn points<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
	>(
		&'l mut self, x: X, y: Y, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y);
		self.common.elems.push(PlotElement::new_plot(
			Points, data, num_rows, num_cols, options,
		));
		self
	}

	/// A combination of lines and points methods (drawn in that order).
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element
	pub fn lines_points<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
	>(
		&'l mut self, x: X, y: Y, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y);
		self.common.elems.push(PlotElement::new_plot(
			LinesPoints,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point.
	/// Additionally, error bars are attached to each data point in the X direction.
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `x_error` - Errors associated with the x value
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `Color` - Sets the color
	pub fn x_error_bars<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Txe: DataType,
		XE: IntoIterator<Item = Txe>,
	>(
		&'l mut self, x: X, y: Y, x_error: XE, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, x_error);
		self.common.elems.push(PlotElement::new_plot(
			XErrorBars, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point.
	/// Additionally, error bars are attached to each data point in the Y direction.
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `y_error` - Errors associated with the y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `Color` - Sets the color
	pub fn y_error_bars<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tye: DataType,
		YE: IntoIterator<Item = Tye>,
	>(
		&'l mut self, x: X, y: Y, y_error: YE, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, y_error);
		self.common.elems.push(PlotElement::new_plot(
			YErrorBars, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point.
	/// Additionally, error bars are attached to each data point in the X and Y directions.
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `x_error` - Errors associated with the x value
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `Color` - Sets the color
	pub fn xy_error_bars<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Txe: DataType,
		XE: IntoIterator<Item = Txe>,
		Tye: DataType,
		YE: IntoIterator<Item = Tye>,
	>(
		&'l mut self, x: X, y: Y, x_error: XE, y_error: YE, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, x_error, y_error);
		self.common.elems.push(PlotElement::new_plot(
			XYErrorBars,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point and lines connecting each data point.
	/// Additionally, error bars are attached to each data point in the X direction.
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `x_error` - Errors associated with the x value
	/// * `options` - Array of PlotOption controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn x_error_lines<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Txe: DataType,
		XE: IntoIterator<Item = Txe>,
	>(
		&'l mut self, x: X, y: Y, x_error: XE, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, x_error);
		self.common.elems.push(PlotElement::new_plot(
			XErrorLines,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D scatter-plot with a point standing in for each data point and lines connecting each data point.
	/// Additionally, error bars are attached to each data point in the Y direction.
	/// # Arguments
	/// * `x` - x values
	/// * `y` - y values
	/// * `y_error` - Errors associated with the y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn y_error_lines<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tye: DataType,
		YE: IntoIterator<Item = Tye>,
	>(
		&'l mut self, x: X, y: Y, y_error: YE, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, y_error);
		self.common.elems.push(PlotElement::new_plot(
			YErrorLines,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D scatter-plot of two curves (bound by `y_lo` and `y_hi`) with a filled region between them.
	/// `FillRegion` plot option can be used to control what happens when the curves intersect. If set to Above, then the `y_lo < y_hi` region is filled.
	/// If set to Below, then the `y_lo > y_hi` region is filled. Otherwise both regions are filled.
	/// # Arguments
	/// * `x` - x values
	/// * `y_lo` - Bottom y values
	/// * `y_hi` - Top y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `FillRegion` - Specifies the region between the two curves to fill
	///     * `Color` - Sets the color of the filled region
	///     * `FillAlpha` - Sets the transparency of the filled region
	pub fn fill_between<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Tyl: DataType,
		YL: IntoIterator<Item = Tyl>,
		Tyh: DataType,
		YH: IntoIterator<Item = Tyh>,
	>(
		&'l mut self, x: X, y_lo: YL, y_hi: YH, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y_lo, y_hi);
		self.common.elems.push(PlotElement::new_plot(
			FillBetween,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	// Plot a polygon given coordinates of its vertices.
	//
	// # Arguments
	// * `x` - x coordinates of the vertices
	// * `y` - y coordinates of the vertices
	// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	//	   * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	//	   * `FillAlpha` - Sets the transparency of the filled region
	//	   * `Color` - Sets the color of the filled region (and the border, unless `BorderColor` is set)
	//	   * `BorderColor` - Sets the color of the border
	//	   * `FillPattern` - Sets the fill pattern
	pub fn polygon<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
	>(
		&'l mut self, x: X, y: Y, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y);
		self.common.elems.push(PlotElement::new_plot(
			Polygons, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot a 2D scatter-plot using boxes of automatic width. Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `y` - y values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn boxes<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
	>(
		&'l mut self, x: X, y: Y, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y);
		self.common.elems.push(PlotElement::new_plot(
			Boxes, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot a 2D scatter-plot using boxes of set (per box) width.
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `y` - y values
	/// * `w` - Box width values
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn boxes_set_width<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tw: DataType,
		W: IntoIterator<Item = Tw>,
	>(
		&'l mut self, x: X, y: Y, w: W, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, w);
		self.common.elems.push(PlotElement::new_plot(
			Boxes, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot a 2D box-plot with error bars using boxes of automatic width.
	/// Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// Each box has an error bar from y - y_delta to y + y_delta.
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `y` - y values
	/// * `y_delta` - errors in y (error bars are plotted from y - y_delta to y + y_delta)
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn box_error_delta<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tye: DataType,
		YE: IntoIterator<Item = Tye>,
	>(
		&'l mut self, x: X, y: Y, y_error: YE, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, y_error);
		self.common.elems.push(PlotElement::new_plot(
			BoxErrorBars,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D box-plot with error bars using boxes of specified width.
	/// Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// Each box has an error bar from y - y_delta to y + y_delta.
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `y` - y values
	/// * `y_delta` - errors in y (error bars are plotted from y - y_delta to y + y_delta)
	/// * `x_delta` - errors in x (interpreted as box width)
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn box_error_delta_set_width<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tye: DataType,
		YE: IntoIterator<Item = Tye>,
		Tw: DataType,
		W: IntoIterator<Item = Tw>,
	>(
		&'l mut self, x: X, y: Y, y_error: YE, x_delta: W, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, y_error, x_delta);
		self.common.elems.push(PlotElement::new_plot(
			BoxErrorBars,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D box-plot with error bars using boxes of automatic width.
	/// Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// Each box has an error bar from y - y_low to y + y_high.
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `y` - y values
	/// * `y_low` - minimum of error bar
	/// * `y_high` - maximum of error bar
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn box_error_low_high<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tyl: DataType,
		YL: IntoIterator<Item = Tyl>,
		Tyh: DataType,
		YH: IntoIterator<Item = Tyh>,
	>(
		&'l mut self, x: X, y: Y, y_low: YL, y_high: YH, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		// The way to get boxerrorbars to interpret low and high y values is to use a dummy negative value for
		// xdelta (box width). If you supply four values rather than five, the fourth is interpreted as width.
		let dummy_width = iter::repeat(-1.0);
		let (data, num_rows, num_cols) = generate_data!(options, x, y, y_low, y_high, dummy_width);
		self.common.elems.push(PlotElement::new_plot(
			BoxErrorBars,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D box-plot with error bars using boxes of specified width.
	/// Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// Each box has an error bar from y - y_low to y + y_high.
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `y` - y values
	/// * `y_low` - minimum of error bar
	/// * `y_high` - maximum of error bar
	/// * `x_delta` - errors in x (interpreted as box width)
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn box_error_low_high_set_width<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		Tyl: DataType,
		YL: IntoIterator<Item = Tyl>,
		Tyh: DataType,
		YH: IntoIterator<Item = Tyh>,
		Tw: DataType,
		W: IntoIterator<Item = Tw>,
	>(
		&'l mut self, x: X, y: Y, y_low: YL, y_high: YH, x_delta: W, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, y_low, y_high, x_delta);
		self.common.elems.push(PlotElement::new_plot(
			BoxErrorBars,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D box-and-whisker plot using boxes of automatic width.
	///
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `box_min` - minimum box y value
	/// * `whisker_min` - minimum whisker y value
	/// * `whisker_max` - maximum whisker y value
	/// * `box_max` - maximum box y value
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	///     * `WhiskerBars` - Sets the width of the whisker bars
	pub fn box_and_whisker<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		TBoxMin: DataType,
		BoxMin: IntoIterator<Item = TBoxMin>,
		TWhiskerMin: DataType,
		WhiskerMin: IntoIterator<Item = TWhiskerMin>,
		TWhiskerMax: DataType,
		WhiskerMax: IntoIterator<Item = TWhiskerMax>,
		TBoxMax: DataType,
		BoxMax: IntoIterator<Item = TBoxMax>,
	>(
		&'l mut self, x: X, box_min: BoxMin, whisker_min: WhiskerMin, whisker_max: WhiskerMax,
		box_max: BoxMax, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) =
			generate_data!(options, x, box_min, whisker_min, whisker_max, box_max);
		self.common.elems.push(PlotElement::new_plot(
			BoxAndWhisker,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot a 2D box-and-whisker plot using boxes of set width.
	///
	/// # Arguments
	/// * `x` - x values (center of the box)
	/// * `box_min` - minimum box y value
	/// * `whisker_min` - minimum whisker y value
	/// * `whisker_max` - maximum whisker y value
	/// * `box_max` - maximum box y value
	/// * `box_width` - width of the box (in x axis units)
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	///     * `WhiskerBars` - Sets the width of the whisker bars
	pub fn box_and_whisker_set_width<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		TBoxMin: DataType,
		BoxMin: IntoIterator<Item = TBoxMin>,
		TWhiskerMin: DataType,
		WhiskerMin: IntoIterator<Item = TWhiskerMin>,
		TWhiskerMax: DataType,
		WhiskerMax: IntoIterator<Item = TWhiskerMax>,
		TBoxMax: DataType,
		BoxMax: IntoIterator<Item = TBoxMax>,
		TBoxWidth: DataType,
		BoxWidth: IntoIterator<Item = TBoxWidth>,
	>(
		&'l mut self, x: X, box_min: BoxMin, whisker_min: WhiskerMin, whisker_max: WhiskerMax,
		box_max: BoxMax, box_width: BoxWidth, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(
			options,
			x,
			box_min,
			whisker_min,
			whisker_max,
			box_max,
			box_width
		);
		self.common.elems.push(PlotElement::new_plot(
			BoxAndWhisker,
			data,
			num_rows,
			num_cols,
			options,
		));
		self
	}

	/// Plot 2D rectangular boxes - usually used for error bars - using specified by width (x_delta) and height (y_delta).
	///
	/// # Arguments
	/// * `x` - x values (horizontal center of the box)
	/// * `y` - y values (vertical center of the box)
	/// * `x_delta` - Error in x (horizontal half-width of the box)
	/// * `y_delta` - Error in y (vertical half-width of the box)
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn box_xy_error_delta<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		TXDelta: DataType,
		XDelta: IntoIterator<Item = TXDelta>,
		TYDelta: DataType,
		YDelta: IntoIterator<Item = TYDelta>,
	>(
		&'l mut self, x: X, y: Y, x_delta: XDelta, y_delta: YDelta, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) = generate_data!(options, x, y, x_delta, y_delta);
		self.common.elems.push(PlotElement::new_plot(
			BoxXYError, data, num_rows, num_cols, options,
		));
		self
	}

	/// Plot 2D rectangular boxes - usually used for error bars - using specified low and high limits for x and y.
	///
	/// # Arguments
	/// * `x` - x values (horizontal center of the box)
	/// * `y` - y values (vertical center of the box)
	/// * `x_low` - Horizontal lower limit of the box
	/// * `x_high` - Horizontal upper limit of the box
	/// * `y_low` - Vertical lower limit of the box
	/// * `y_high` - Vertical upper limit of the box
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn box_xy_error_low_high<
		'l,
		Tx: DataType,
		X: IntoIterator<Item = Tx>,
		Ty: DataType,
		Y: IntoIterator<Item = Ty>,
		TXLow: DataType,
		XLow: IntoIterator<Item = TXLow>,
		TXHigh: DataType,
		XHigh: IntoIterator<Item = TXHigh>,
		TYLow: DataType,
		YLow: IntoIterator<Item = TYLow>,
		TYHigh: DataType,
		YHigh: IntoIterator<Item = TYHigh>,
	>(
		&'l mut self, x: X, y: Y, x_low: XLow, x_high: XHigh, y_low: YLow, y_high: YHigh,
		options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		let (data, num_rows, num_cols) =
			generate_data!(options, x, y, x_low, x_high, y_low, y_high);
		self.common.elems.push(PlotElement::new_plot(
			BoxXYError, data, num_rows, num_cols, options,
		));
		self
	}

	/// Draws an image from a rectangular array of data by connecting the individual datapoints with polygons.
	///
	/// #Arguments:
	/// * `mat` - Row-major 2D array signifying the value of the datapoints. The X and Y coordinates of the datapoints are determined automatically,
	///           and optionally scaled using the `dimensions` argument.
	/// * `num_rows` - Number of rows in the data array
	/// * `num_cols` - Number of columns in the data array
	/// * `dimensions` - Optional X and Y coordinates of the first and last data points (with the rest of the coordinates spaced evenly between).
	///                  By default this will be `(0, 0)` and `(num_rows - 1, num_cols - 1)`.
	/// * `options` - Array of PlotOption<&str> controlling the appearance of the surface. Relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	pub fn image<'l, T: DataType, X: IntoIterator<Item = T>>(
		&'l mut self, mat: X, num_rows: usize, num_cols: usize,
		dimensions: Option<(f64, f64, f64, f64)>, options: &[PlotOption<&str>],
	) -> &'l mut Self
	{
		self.common.elems.push(PlotElement::new_plot_matrix(
			Image,
			false,
			mat,
			num_rows,
			num_cols,
			dimensions,
			options.to_one_way_owned(),
		));
		self
	}

	pub(crate) fn write_out(
		&self, data_directory: Option<&str>, writer: &mut dyn Writer, auto_layout: bool,
		version: GnuplotVersion,
	)
	{
		self.common.write_out_commands(writer, auto_layout, version);
		self.border_options.write_out(writer, version);
		let mut grid_axes = vec![];
		if self.common.x_axis.grid
		{
			grid_axes.push(self.common.x_axis.axis);
		}
		if self.common.y_axis.grid
		{
			grid_axes.push(self.common.y_axis.axis);
		}
		if self.common.cb_axis.grid
		{
			grid_axes.push(self.common.cb_axis.axis);
		}
		self.common.write_grid_options(writer, &grid_axes, version);
		for arrow in &self.arrows
		{
			arrow.write_out(writer);
		}
		if let Some(l) = self.legend.as_ref()
		{
			l.write_out(writer)
		};
		self.common
			.write_out_elements("plot", data_directory, writer, version);
	}

	pub(crate) fn reset_state(&self, writer: &mut dyn Writer)
	{
		self.common.reset_state(writer);
		for arrow in &self.arrows
		{
			arrow.reset_state(writer);
		}
		if let Some(l) = self.legend.as_ref()
		{
			l.reset_state(writer)
		};
	}
}

impl AxesCommonPrivate for Axes2D
{
	fn get_common_data(&self) -> &AxesCommonData
	{
		&self.common
	}

	fn get_common_data_mut(&mut self) -> &mut AxesCommonData
	{
		&mut self.common
	}
}

impl AxesCommon for Axes2D {}
