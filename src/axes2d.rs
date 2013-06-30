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
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the label
	pub fn set_x_label<'l>(&'l mut self, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(XLabel, text, options)
	}
	
	/// Set the label for the Y axis
	/// # Arguments
	/// * `text` - Text of the label. Pass an empty string to hide the label
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the label
	pub fn set_y_label<'l>(&'l mut self, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(YLabel, text, options)
	}

	/// Set the title for the axes
	/// # Arguments
	/// * `text` - Text of the title. Pass an empty string to hide the title
	/// * `options` - Array of [LabelOption](options.html#enum-labeloption) controlling the appearance of the title
	pub fn set_title<'l>(&'l mut self, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(Title, text, options)
	}
	
	pub fn label<'l>(&'l mut self, text : &str, x : Coordinate, y : Coordinate, options : &[LabelOption]) -> &'l mut Axes2D
	{
		self.set_label_common(Label(x, y), text, options)
	}
	
	fn set_label_common<'l>(&'l mut self, label_type : LabelType, text : &str, options : &[LabelOption]) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			let label_str = match label_type
			{
				XLabel => "xlabel",
				YLabel => "ylabel",
				Title => "title",
				Label(*) => "label",
				/* _ => fail!("Invalid label type") */
			};

			c.write_str("set ");
			c.write_str(label_str);
			c.write_str(" \"");
			c.write_str(text);
			c.write_str("\"");
			
			match label_type
			{
				Label(x, y) => 
				{
					c.write_str(" at ");
					x.write(c);
					c.write_str(",");
					y.write(c);
					c.write_str(" front");
				}
				_ => ()
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					Offset(x, y) =>
					{
						c.write_str(" offset character ");
						c.write_float(x);	
						c.write_str(",");
						c.write_float(y);
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					TextColor(s) =>
					{
						c.write_str(" tc rgb \"");
						c.write_str(s);
						c.write_str("\"");
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					Font(f, s) =>
					{
						c.write_str(" font \"");
						c.write_str(f);
						c.write_str(",");
						c.write_str(s.to_str());
						c.write_str("\"");
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					Rotate(a) =>
					{
						c.write_str(" rotate by ");
						c.write_float(a);
						break;
					},
					_ => ()
				};
			}
			
			if label_type.is_label()
			{
				let mut have_point = false;
				for options.iter().advance |o|
				{
					match *o
					{
						MarkerSymbol(s) =>
						{
							c.write_str(" point pt ");
							c.write_int(char_to_symbol(s));
							have_point = true;
							break;
						},
						_ => ()
					};
				}
				
				if have_point
				{
					for options.iter().advance |o|
					{
						match *o
						{
							MarkerColor(s) =>
							{
								c.write_str(" lc rgb \"");
								c.write_str(s);
								c.write_str("\"");
								break;
							},
							_ => ()
						};
					}
					
					for options.iter().advance |o|
					{
						match *o
						{
							MarkerSize(z) =>
							{
								c.write_str(" ps ");
								c.write_float(z);
								c.write_str("");
								break;
							},
							_ => ()
						};
					}
				}
				
				for options.iter().advance |o|
				{
					match *o
					{
						Align(a) =>
						{
							c.write_str(match(a)
							{
								AlignLeft => " left",
								AlignRight => " right",
								AlignCenter => " center",
							});
							break;
						},
						_ => ()
					};
				}
			}
			
			c.write_str("\n");
		}
		self
	}
	
	pub fn arrow<'l>(&'l mut self, x1 : Coordinate, y1 : Coordinate, x2 : Coordinate, y2 : Coordinate, options : &[ArrowOption]) -> &'l mut Axes2D
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
			
			for options.iter().advance |o|
			{
				match *o
				{
					HeadType(s) =>
					{
						c.write_str(match(s)
						{
							Open => "",
							Closed => " empty",
							Filled => " filled",
							NoArrow => " nohead",
						});
						break;
					},
					_ => ()
				};
			}
			
			c.write_str(" size graph ");
			let mut found_size = false;
			for options.iter().advance |o|
			{
				match *o
				{
					HeadSize(s) =>
					{
						c.write_float(s);
						found_size = true;
						break;
					},
					_ => ()
				};
			}
			if !found_size
			{
				c.write_str("0.05");
			}
			c.write_str(",12");
			
			for options.iter().advance |o|
			{
				match *o
				{
					ArrowColor(s) =>
					{
						c.write_str(" lc rgb \"");
						c.write_str(s);
						c.write_str("\"");
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					ShaftWidth(w) =>
					{
						c.write_str(" lw ");
						c.write_float(w);
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					ShaftType(t) =>
					{
						c.write_str(" lt ");
						c.write_int(t.to_int());
						break;
					},
					_ => ()
				};
			}
			
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element
	pub fn lines<'l, Tx : DataType, X : Iterator<Tx>, Ty : DataType, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Lines, x, y, options);
		self
	}
	
	/// Plot a 2D scatter-plot with a point standing in for each data point
	/// # Arguments
	/// * `x` - Iterator for the x values
	/// * `y` - Iterator for the y values
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element
	pub fn points<'l, Tx : DataType, X : Iterator<Tx>, Ty : DataType, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Points, x, y, options);
		self
	}
	
	/// Plot a 2D scatter-plot with a point standing in for each data point and lines connecting each data point
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element
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
	/// * `options` - Array of [PlotOption](options.html#enum-plotoption) controlling the appearance of the plot element
	pub fn fill_between<'l, 
	                   Tx : DataType, X : Iterator<Tx>,
	                   Tyl : DataType, YL : Iterator<Tyl>,
	                   Tyh : DataType, YH : Iterator<Tyh>>(&'l mut self, x : X, y_lo : YL, y_hi : YH, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot3(FillBetween, x, y_lo, y_hi, options);
		self
	}
}

mod private
{
	use axes_common::*;
	use options::*;
	use datatype::*;
	use writer::*;
	
	struct Axes2D
	{
		common : AxesCommon
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
