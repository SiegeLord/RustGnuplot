// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use datatype::*;

pub trait PlotWriter
{
	fn write_data<T: DataType>(&mut self, v: T);
}
