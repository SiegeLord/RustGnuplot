// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.


pub use self::Coordinate::*;
use std::fmt;

/// Specifies how to interpret the coordinate passed to a plotting command
#[derive(Copy, Clone)]
pub enum Coordinate
{
	/// Coordinates are done relative to a graph (i.e. an axis set). (0, 0) is the bottom left corner and (1, 1) is the top right corner.
	/// You'd use this to place labels and other objects so that they remain in the same place relative to the graph no matter what you have plotted.
	Graph(f64),
	/// Coordinates match those on the axes. You'd use this to place labels and other objects relative to regions of interest in the graph (e.g. labeling the peak of a function)
	Axis(f64),
}

impl fmt::Display for Coordinate
{
	fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result
	{
		let (name, x) = match *self
		{
			Graph(x) => (" graph ", x),
			Axis(x) => (" first ", x),
		};
		write!(buf, "{}{:.16e}", name, x)
	}
}
