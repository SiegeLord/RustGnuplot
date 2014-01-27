// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use datatype::*;

use std::io::MemWriter;
use std::io::Writer;

pub trait PlotWriter
{
	fn write_data<T: DataType>(&mut self, v: T);
}

impl PlotWriter for MemWriter
{
	fn write_data<T: DataType>(&mut self, v: T)
	{
		self.write_le_f64(v.get());
	}
}
