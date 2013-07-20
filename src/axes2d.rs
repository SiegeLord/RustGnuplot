use axes_common::*;
use axes2d::private::*;
use coordinates::*;
use datatype::*;
use options::*;
use writer::*;

/// 2D axes that is used for drawing 2D plots
impl Axes2D
{
	/// Set the position of the axes on the figure using grid coordinates
	/// # Arguments
	/// * `row` - Row on the grid. Top-most row is 1
	/// * `column` - Column on the grid. Left-most column is 1
	pub fn set_pos_grid<'l>(&'l mut self, row : uint, col : uint) -> &'l mut Axes2D
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
	pub fn set_pos<'l>(&'l mut self, x : float, y : float) -> &'l mut Axes2D
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
	pub fn set_size<'l>(&'l mut self, w : float, h : float) -> &'l mut Axes2D
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
	pub fn set_aspect_ratio<'l>(&'l mut self, ratio : AutoOption<float>) -> &'l mut Axes2D
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
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	pub fn set_x_label<'l>(&'l mut self, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(XLabel, text, options)
	}
	
	/// Set the label for the Y axis
	/// # Arguments
	/// * `text` - Text of the label. Pass an empty string to hide the label
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	pub fn set_y_label<'l>(&'l mut self, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(YLabel, text, options)
	}

	/// Set the title for the axes
	/// # Arguments
	/// * `text` - Text of the title. Pass an empty string to hide the title
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the title. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	pub fn set_title<'l>(&'l mut self, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(Title, text, options)
	}
	
	/// Adds a label to the plot, with an optional marker.
	/// # Arguments
	/// * `text` - Text of the label
	/// * `x` - X coordinate of the label, specified using the [Coordinate](coordinates.html) type
	/// * `y` - Y coordinate of the label, specified using the [Coordinate](coordinates.html) type
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	///      * `MarkerSymbol` - Specifies the symbol for the marker. Omit to hide the marker
	///      * `MarkerSize` - Specifies the size for the marker
	///      * `MarkerColor` - Specifies the color for the marker
	pub fn label<'l>(&'l mut self, text : &str, x : Coordinate, y : Coordinate, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(Label(x, y), text, options)
	}
	
	fn set_label_common<'l>(&'l mut self, label_type : LabelType, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set ");
			
			let label_str = match label_type
			{
				XLabel => "xlabel",
				YLabel => "ylabel",
				Title => "title",
				Label(*) => "label",
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
	
	fn set_ticks_common<'l>(&'l mut self, tick_type : TickType, min : AutoOption<float>, incr : float, max : AutoOption<float>, tick_options : &[TickOption], label_options : &[LabelOption]) -> &'l mut Axes2D
	{
		if incr <= 0.0
		{
			fail!("'incr' must be positive, but is actually %f", incr);
		}
		
		{
			let c = &mut self.common.commands;
			
			let mut minor_intervals : uint = 0;
			first_opt!(tick_options,
				MinorIntervals(i) =>
				{
					minor_intervals = i;
				}
			)

			c.write_str("set m");
			c.write_str(tick_type.to_str());
			c.write_str(" ");
			c.write_int(minor_intervals as int);
			c.write_str("\n");
			
			c.write_str("set ");
			c.write_str(tick_type.to_str());
			
			c.write_str(" ");
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
					c.write_float(min);
					c.write_str(",");
					c.write_float(incr);
					c.write_str(",");
					c.write_float(max);
				}
			}
			
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
	/// # Arguments
	/// * `min` - Sets the location of where the tics start
	/// * `incr` - Sets the spacing between the major tics
	/// * `max` - Sets the location of where the tics end
	/// * `tick_options` - Array of [TickOption](options.html#enum-tickoption) controlling the appearance of the ticks
	/// * `label_options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the tick labels. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	pub fn set_x_tics<'l>(&'l mut self, min : AutoOption<float>, incr : float, max : AutoOption<float>, tick_options : &[TickOption], label_options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_ticks_common(XTics, min, incr, max, tick_options, label_options)
	}
	
	/// Like `set_x_tics` but for the Y axis.
	pub fn set_y_tics<'l>(&'l mut self, min : AutoOption<float>, incr : float, max : AutoOption<float>, tick_options : &[TickOption], label_options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_ticks_common(YTics, min, incr, max, tick_options, label_options)
	}
	
	/// Adds an arrow to the plot. The arrow is drawn from `(x1, y1)` to `(x2, y2)` with the arrow point towards `(x2, y2)`.
	/// # Arguments
	/// * `x1` - X coordinate of the arrow start, specified using the [Coordinate](coordinates.html) type
	/// * `y1` - Y coordinate of the arrow start, specified using the [Coordinate](coordinates.html) type
	/// * `x2` - X coordinate of the arrow end, specified using the [Coordinate](coordinates.html) type
	/// * `y2` - Y coordinate of the arrow end, specified using the [Coordinate](coordinates.html) type
	/// * `options` - Array of [PlotOption](options.html#enum-arrowoption) controlling the appearance of the arrowhead and arrow shaft. Relevant options are:
	///      * `ArrowType` - Specifies the style of the arrow head (or an option to omit it)
	///      * `ArrowSize` - Sets the size of the arrow head (in graph units)
	///      * `Color` - Specifies the color of the arrow
	///      * `LineStyle` - Specifies the style of the arrow shaft
	///      * `LineWidth` - Specifies the width of the arrow shaft
	pub fn arrow<'l>(&'l mut self, x1 : Coordinate, y1 : Coordinate, x2 : Coordinate, y2 : Coordinate, options : &[PlotOption]) -> &'l mut Axes2D
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
			let mut found_size = false;
			first_opt!(options,
				ArrowSize(s) =>
				{
					c.write_float(s);
					found_size = true;
				}
			)
			if !found_size
			{
				c.write_str("0.05");
			}
			c.write_str(",12");
			
			first_opt!(options,
				Color(s) =>
				{
					c.write_str(" lc rgb \"");
					c.write_str(s);
					c.write_str("\"");
				}
			)
			
			first_opt!(options,
				LineWidth(w) =>
				{
					c.write_str(" lw ");
					c.write_float(w);
				}
			)
			
			first_opt!(options,
				LineStyle(t) =>
				{
					c.write_str(" lt ");
					c.write_int(t.to_int());
				}
			)
			
			c.write_str("\n");
		}
		self
	}
	
	/// Set the range of values for the X axis
	/// # Arguments
	/// * `min` - Minimum X value
	/// * `max` - Maximum X value
	pub fn set_x_range<'l>(&'l mut self, min : AutoOption<float>, max : AutoOption<float>) -> &'l mut Axes2D
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
	pub fn set_y_range<'l>(&'l mut self, min : AutoOption<float>, max : AutoOption<float>) -> &'l mut Axes2D
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
	
	/// Plot a 2D scatter-plot with lines connecting each data point
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn lines<'l, Tx : DataType, X : Iterator<Tx>, Ty : DataType, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Lines, x, y, options);
		self
	}
	
	/// Plot a 2D scatter-plot with a point standing in for each data point
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `Color` - Sets the color
	pub fn points<'l, Tx : DataType, X : Iterator<Tx>, Ty : DataType, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Points, x, y, options);
		self
	}
	
	/// A combination of [lines](#method-lines) and [points](#method-points) methods (drawn in that order).
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element
	pub fn lines_points<'l, Tx : DataType, X : Iterator<Tx>, Ty : DataType, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn x_error_lines<'l, 
	                   Tx : DataType, X : Iterator<Tx>,
	                   Ty : DataType, Y : Iterator<Ty>,
	                   Txe : DataType, XE : Iterator<Txe>>(&'l mut self, x : X, y : Y, x_error : XE, options : &[PlotOption]) -> &'l mut Axes2D
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `PointSymbol` - Sets symbol for each point
	///     * `PointSize` - Sets the size of each point
	///     * `LineWidth` - Sets the width of the line
	///     * `LineStyle` - Sets the style of the line
	///     * `Color` - Sets the color
	pub fn y_error_lines<'l, 
	                   Tx : DataType, X : Iterator<Tx>,
	                   Ty : DataType, Y : Iterator<Ty>,
	                   Tye : DataType, YE : Iterator<Tye>>(&'l mut self, x : X, y : Y, y_error : YE, options : &[PlotOption]) -> &'l mut Axes2D
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `FillRegion` - Specifies the region between the two curves to fill
	///     * `Color` - Sets the color of the filled region
	///     * `FillAlpha` - Sets the transparency of the filled region
	pub fn fill_between<'l, 
	                   Tx : DataType, X : Iterator<Tx>,
	                   Tyl : DataType, YL : Iterator<Tyl>,
	                   Tyh : DataType, YH : Iterator<Tyh>>(&'l mut self, x : X, y_lo : YL, y_hi : YH, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(FillBetween, x, y_lo, y_hi, options);
		self
	}
	
	/// Plot a 2D scatter-plot using boxes of automatic width. Box widths are set so that there are no gaps between successive boxes (i.e. each box may have a different width).
	/// Boxes start at the x-axis and go towards the y value of the datapoint.
	/// # Arguments
	/// * `x` - Iterator for the x values (center of the box)
	/// * `y` - Iterator for the y values
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn boxes<'l, Tx : DataType, X : Iterator<Tx>, Ty : DataType, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element. The relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	///     * `LineWidth` - Sets the width of the border
	///     * `LineStyle` - Sets the style of the border
	///     * `BorderColor` - Sets the color of the border
	///     * `Color` - Sets the color of the box fill
	///     * `FillAlpha` - Sets the transparency of the box fill
	pub fn boxes_set_width<'l,
	                      Tx : DataType,
	                      X : Iterator<Tx>,
	                      Ty : DataType,
	                      Y : Iterator<Ty>,
	                      Tw : DataType,
	                      W : Iterator<Tw>>(&'l mut self, x : X, y : Y, w : W, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(Boxes, x, y, w, options);
		self
	}
}

mod private
{
	use axes_common::*;
	use options::*;
	use datatype::*;
	use writer::*;
	
	struct Tics
	{
		common_options : ~[u8],
		text_options : ~[u8],
	}
	
	struct Axes2D
	{
		common : AxesCommon,
	}
	
	impl Axes2D
	{
		pub fn new() -> Axes2D
		{
			Axes2D
			{
				common : AxesCommon::new()
			}
		}
		
		pub fn plot2<T1 : DataType, X1 : Iterator<T1>,
				 T2 : DataType, X2 : Iterator<T2>>(&mut self, plot_type : PlotType, x1 : X1, x2 : X2, options : &[PlotOption])
		{
			let l = self.common.elems.len();
			self.common.elems.push(PlotElement::new());
			let mut num_rows : int = 0;
			
			{
				let data = &mut self.common.elems[l].data;
				for x1.zip(x2).advance |(x1, x2)|
				{
					data.write_data(x1);
					data.write_data(x2);
					num_rows += 1;
				}
			}
			
			self.common.write_common_commands(l, num_rows, 2, plot_type, options);
		}
		
		pub fn plot3<T1 : DataType, X1 : Iterator<T1>,
				 T2 : DataType, X2 : Iterator<T2>,
				 T3 : DataType, X3 : Iterator<T3>>(&mut self, plot_type : PlotType, x1 : X1, x2 : X2, x3 : X3, options : &[PlotOption])
		{
			let l = self.common.elems.len();
			self.common.elems.push(PlotElement::new());
			let mut num_rows : int = 0;
			
			{
				let data = &mut self.common.elems[l].data;
				for x1.zip(x2).zip(x3).advance |((x1, x2), x3)|
				{
					data.write_data(x1);
					data.write_data(x2);
					data.write_data(x3);
					num_rows += 1;
				}
			}
			
			self.common.write_common_commands(l, num_rows, 3, plot_type, options);
		}
		
		pub fn write_out(&self, writer : &fn(data : &[u8]))
		{
			if self.common.elems.len() == 0
			{
				return;
			}
			
			writer(self.common.commands);

			writer("plot".as_bytes());
			
			let mut first = true;
			for self.common.elems.iter().advance |e|
			{
				if !first
				{
					writer(",".as_bytes());
				}
				writer(e.args);
				first = false;
			}
			
			writer("\n".as_bytes());
			
			for self.common.elems.iter().advance |e|
			{
				writer(e.data);
			}
		}
	}
}
