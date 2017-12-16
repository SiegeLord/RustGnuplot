// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.


use axes_common::*;
use datatype::*;
use options::*;
use std::io::Write;
use util::OneWayOwned;
use writer::Writer;

/// 3D axes that is used for drawing 3D plots
pub struct Axes3D
{
	common: AxesCommonData,
	z_axis: AxisData,
	contour_base: bool,
	contour_surface: bool,
	contour_auto: AutoOption<u32>,
	contour_levels: Option<Vec<f64>>,
	contour_style: ContourStyle,
	contour_label: AutoOption<String>,
}

impl Axes3D
{
	pub(crate) fn new() -> Axes3D
	{
		Axes3D {
			common: AxesCommonData::new(),
			z_axis: AxisData::new(ZTickAxis),
			contour_base: false,
			contour_surface: false,
			contour_auto: Auto,
			contour_levels: None,
			contour_style: Linear,
			contour_label: Auto,
		}
	}

	/// Draws a 3D surface from a rectangular array of data by connecting the individual datapoints with polygons.
	///
	/// #Arguments:
	/// * `mat` - Row-major 2D array signifying the Z coordinate of the datapoints. The X and Y coordinates of the datapoints are determined automatically,
	///           and optionally scaled using the `dimensions` argument.
	/// * `num_rows` - Number of rows in the data array
	/// * `num_cols` - Number of columns in the data array
	/// * `dimensions` - Optional X and Y coordinates of the first and last data points (with the rest of the coordinates spaced evenly between).
	///                  By default this will be `(0, 0)` and `(num_rows - 1, num_cols - 1)`.
	/// * `options` - Array of PlotOption controlling the appearance of the surface. Relevant options are:
	///     * `Caption` - Specifies the caption for this dataset. Use an empty string to hide it (default).
	pub fn surface<'l, T: DataType, X: IntoIterator<Item = T>>(&'l mut self, mat: X, num_rows: usize, num_cols: usize, dimensions: Option<(f64, f64, f64, f64)>, options: &[PlotOption<&str>])
		-> &'l mut Self
	{
		self.common.elems.push(PlotElement::new_plot_matrix(
			Pm3D,
			true,
			mat,
			num_rows,
			num_cols,
			dimensions,
			options.to_one_way_owned(),
		));
		self
	}

	/// Sets the 3D view.
	///
	/// #Arguments:
	/// * `pitch` - Pitch, in degrees. Value of 0 is looking straight down on the XY plane, Z pointing out of the screen.
	/// * `yaw` - Yaw, in degrees. Value of 0 is looking at the XZ plane, Y point into the screen.
	pub fn set_view<'l>(&'l mut self, pitch: f64, yaw: f64) -> &'l mut Self
	{
		writeln!(&mut self.common.commands, "set view {:.12e},{:.12e}", pitch, yaw);
		self
	}

	/// Sets the view to be a map. Useful for images and contour plots.
	pub fn set_view_map<'l>(&'l mut self) -> &'l mut Self
	{
		writeln!(&mut self.common.commands, "set view map");
		self
	}

	/// Set the label for the Z axis
	///
	/// # Arguments
	/// * `text` - Text of the label. Pass an empty string to hide the label
	/// * `options` - Array of LabelOption controlling the appearance of the label. Relevant options are:
	///      * `Offset` - Specifies the offset of the label
	///      * `Font` - Specifies the font of the label
	///      * `TextColor` - Specifies the color of the label
	///      * `Rotate` - Specifies the rotation of the label
	///      * `Align` - Specifies how to align the label
	pub fn set_z_label<'l>(&'l mut self, text: &str, options: &[LabelOption<&str>]) -> &'l mut Self
	{
		self.get_common_data_mut().set_label_common(ZLabel, text, options);
		self
	}

	/// Like `set_x_ticks` but for the Z axis.
	pub fn set_z_ticks<'l>(
		&'l mut self, tick_placement: Option<(AutoOption<f64>, u32)>, tick_options: &[TickOption<&str>],
		label_options: &[LabelOption<&str>],
	) -> &'l mut Self
	{
		self.z_axis
			.set_ticks(tick_placement, tick_options.to_one_way_owned(), label_options.to_one_way_owned());
		self
	}

	/// Like `set_x_ticks_custom` but for the the Y axis.
	pub fn set_z_ticks_custom<'l, T: DataType, TL: IntoIterator<Item = Tick<T>>>(&'l mut self, ticks: TL, tick_options: &[TickOption<&str>], label_options: &[LabelOption<&str>])
		-> &'l mut Self
	{
		self.z_axis
			.set_ticks_custom(ticks, tick_options.to_one_way_owned(), label_options.to_one_way_owned());
		self
	}

	/// Set the range of values for the Z axis
	///
	/// # Arguments
	/// * `min` - Minimum Z value
	/// * `max` - Maximum Z value
	pub fn set_z_range<'l>(&'l mut self, min: AutoOption<f64>, max: AutoOption<f64>) -> &'l mut Self
	{
		self.z_axis.set_range(min, max);
		self
	}

	/// Sets z axis to reverse.
	pub fn set_z_reverse<'l>(&'l mut self, reverse: bool) -> &'l mut Self
	{
		self.z_axis.set_reverse(reverse);
		self
	}


	/// Sets the Z axis be logarithmic. Note that the range must be non-negative for this to be valid.
	///
	/// # Arguments
	/// * `base` - If Some, then specifies base of the logarithm, if None makes the axis not be logarithmic
	pub fn set_z_log<'l>(&'l mut self, base: Option<f64>) -> &'l mut Self
	{
		self.z_axis.set_log(base);
		self
	}

	/// Shows the grid on the Z axis.
	///
	/// # Arguments
	/// * `show` - Whether to show the grid.
	pub fn set_z_grid<'l>(&'l mut self, show: bool) -> &'l mut Self
	{
		self.z_axis.set_grid(show);
		self
	}

	/// Show contours (lines of equal Z value) at automatically determined levels.
	///
	/// # Arguments
	/// * `base` - Show contours on the base of the plot (XY plane)
	/// * `surface` - Show the contours on the surface itself
	/// * `style` - Style of the contours
	/// * `label` - Auto sets the label automatically and enables the legend, Fix() allows you specify a format string (using C style formatting),
	///             otherwise an empty string disables the legend and labels.
	/// * `levels` - Auto picks some default number of levels, otherwise you can pass a set nominal number instead. The number is nominal as
	///              contours are placed at nice values of Z, and thus there may be fewer of them than this number.
	pub fn show_contours<'l>(&'l mut self, base: bool, surface: bool, style: ContourStyle, label: AutoOption<&str>, levels: AutoOption<u32>)
		-> &'l mut Self
	{
		self.contour_base = base;
		self.contour_surface = surface;
		self.contour_style = style;
		self.contour_auto = levels;
		self.contour_levels = None;
		self.contour_label = label.map(|l| l.to_string());
		self
	}

	/// Show contours (lines of equal Z value) at specific levels.
	///
	/// # Arguments
	/// * `base` - Show contours on the base of the plot (XY plane)
	/// * `surface` - Show the contours on the surface itself
	/// * `style` - Style of the contours
	/// * `label` - Auto sets the label automatically and enables the legend, Fix() allows you specify a format string (using C style formatting),
	///             otherwise an empty string disables the legend and labels.
	/// * `levels` - A set of levels.
	pub fn show_contours_custom<'l, T: DataType, TC: IntoIterator<Item = T>>(&'l mut self, base: bool, surface: bool, style: ContourStyle, label: AutoOption<&str>, levels: TC)
		-> &'l mut Self
	{
		self.contour_base = base;
		self.contour_surface = surface;
		self.contour_style = style;
		self.contour_auto = Auto;
		self.contour_levels = Some(levels.into_iter().map(|l| l.get()).collect());
		self.contour_label = label.map(|l| l.to_string());
		self
	}
}

impl AxesCommonPrivate for Axes3D
{
	fn get_common_data_mut(&mut self) -> &mut AxesCommonData
	{
		&mut self.common
	}

	fn get_common_data(&self) -> &AxesCommonData
	{
		&self.common
	}
}

impl AxesCommon for Axes3D {}

pub(crate) trait Axes3DPrivate
{
	fn write_out(&self, writer: &mut Writer);
}

impl Axes3DPrivate for Axes3D
{
	fn write_out(&self, w: &mut Writer)
	{
		fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T
		{
			if val < min
			{
				min
			}
			else if val > max
			{
				max
			}
			else
			{
				val
			}
		}

		if self.common.elems.len() == 0
		{
			return;
		}

		if self.contour_base || self.contour_surface
		{
			write!(w, "set contour ");
			write!(
				w,
				"{}",
				match (self.contour_base, self.contour_surface)
				{
					(true, false) => "base",
					(false, true) => "surface",
					(true, true) => "both",
					_ => unreachable!(),
				}
			);
			write!(w, "\n");

			match self.contour_label
			{
				Auto => writeln!(w, "set clabel"),
				Fix(ref s) => if s.len() == 0
				{
					writeln!(w, "unset clabel")
				}
				else
				{
					writeln!(w, r#"set clabel "{}""#, s)
				},
			};

			fn set_cntrparam<F: FnOnce(&mut Writer)>(w: &mut Writer, wr: F)
			{
				write!(w, "set cntrparam ");
				wr(w);
				write!(w, "\n");
			}

			set_cntrparam(w, |w| {
				write!(
					w,
					"{}",
					match self.contour_style
					{
						Linear => "linear ",
						Cubic(..) => "cubicspline",
						Spline(..) => "bspline",
					}
				);
			});

			set_cntrparam(w, |w| {
				let pt = match self.contour_style
				{
					Cubic(pt) => Some(pt),
					Spline(pt, _) => Some(pt),
					_ => None,
				};

				pt.map(|pt| {
					write!(w, "points {}", clamp(pt, 2, 100));
				});
			});

			set_cntrparam(w, |w| {
				let ord = match self.contour_style
				{
					Spline(_, ord) => Some(ord),
					_ => None,
				};

				ord.map(|ord| {
					write!(w, "order {}", clamp(ord, 2, 10));
				});
			});

			set_cntrparam(w, |w| {
				write!(w, "levels ");
				match self.contour_levels
				{
					Some(ref ls) =>
					{
						write!(w, "discrete ");
						let mut left = ls.len();
						for &l in ls.iter()
						{
							write!(w, "{:.12e}", l);
							if left > 1
							{
								write!(w, ",");
							}
							left -= 1;
						}
					}
					None =>
					{
						match self.contour_auto
						{
							Auto => write!(w, "auto "),
							Fix(f) => write!(w, "{}", f),
						};
					}
				};
			});
		}

		self.common.write_out_commands(w);
		self.z_axis.write_out_commands(w);
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
		if self.z_axis.grid
		{
			grid_axes.push(self.z_axis.axis);
		}
		self.common.write_grid_options(w, &grid_axes);
		self.common.write_out_elements("splot", w);
	}
}
