// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;
use datatype::*;
use options::*;
use writer::PlotWriter;

pub struct Axes3D
{
	priv common: AxesCommonData
}

impl Axes3D
{
	/// Draws a 3D surface
	pub fn surface<'l, T: DataType, X: Iterator<T>>(&'l mut self, mat: X, num_rows: i32, num_cols: i32, options: &[PlotOption]) -> &'l mut Axes3D
	{
		self.common.plot_matrix(Pm3D, mat, num_rows, num_cols, options);
		self
	}

	/// Sets the 3D view.
	///
	/// #Arguments:
	/// * `pitch` - Pitch, in degrees. Value of 0 is looking straight down on the XY plane, Z pointing out of the screen.
	/// * `yaw` - Yaw, in degrees. Value of 0 is looking at the XZ plane, Y point into the screen.
	pub fn set_view<'l>(&'l mut self, pitch: f64, yaw: f64) -> &'l mut Axes3D
	{
		{
			let c = &mut self.common.commands;
			c.write_str("set view ");
			c.write_float(pitch);
			c.write_str(",");
			c.write_float(yaw);
			c.write_str("\n");
		}
		self
	}

	/// Sets the view to be a map. Useful for images and contour plots.
	pub fn set_view_map<'l>(&'l mut self) -> &'l mut Axes3D
	{
		writeln!(&mut self.common.commands, "set view map");
		self
	}
}

pub fn new_axes3d() -> Axes3D
{
	Axes3D{common: AxesCommonData::new()}
}

impl AxesCommonPrivate for Axes3D
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

impl AxesCommon for Axes3D {}

pub trait Axes3DPrivate
{
	fn write_out(&self, writer: &mut Writer);
}

impl Axes3DPrivate for Axes3D
{
	fn write_out(&self, writer: &mut Writer)
	{
		if self.common.elems.len() == 0
		{
			return;
		}

		self.common.write_out_commands(writer);
		self.common.write_out_elements("splot", writer);
	}
}
