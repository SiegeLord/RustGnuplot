// Copyright (c) 2022 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use crate::axes2d::{ArrowData, BorderOptions, LegendData};
use crate::axes_common::*;
use crate::datatype::*;
use crate::options::*;
use crate::util::OneWayOwned;
use crate::writer::Writer;

/// Polar axes that is used for drawing polar plots
pub struct AxesPolar
{
	common: AxesCommonData,
	border_options: BorderOptions,
	arrows: Vec<ArrowData>,
	legend: Option<LegendData>,
}

impl AxesPolar
{
	pub(crate) fn new() -> AxesPolar
	{
		AxesPolar {
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
		self.common.elems.push(PlotElement::new_plot2(
			Lines,
			x,
			y,
			options.to_one_way_owned(),
		));
		self
	}

	pub(crate) fn write_out(
		&self, writer: &mut dyn Writer, auto_layout: bool, version: GnuplotVersion,
	)
	{
		writeln!(writer, "set polar");
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
		self.common.write_out_elements("plot", writer, version);
		writeln!(writer, "unset polar");
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

impl AxesCommonPrivate for AxesPolar
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

impl AxesCommon for AxesPolar {}
