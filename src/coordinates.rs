// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

/// Specifies how to interpret the coordinate passed to a plotting command
pub enum Coordinate
{
	/// Coordinates are done relative to a graph (i.e. an axis set). (0, 0) is the bottom left corner and (1, 1) is the top right corner.
	/// You'd use this to place labels and other objects so that they remain in the same place relative to the graph no matter what you have plotted.
	Graph(float),
	/// Coordinates match those on the axes. You'd use this to place labels and other objects relative to regions of interest in the graph (e.g. labeling the peak of a function)
	Axis(float)
}

mod private
{
	use super::*;
	use writer::*;

	impl Coordinate
	{
		pub fn write<T : PlotWriter>(&self, writer : &mut T)
		{
			let (name, x) = match *self
			{
				Graph(x) => (" graph ", x),
				Axis(x) => (" first ", x),
			};
			writer.write_str(name);
			writer.write_float(x);
		}
	}
}
