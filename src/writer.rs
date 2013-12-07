// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use datatype::*;

use std::f64;
use std::io::mem::MemWriter;
use std::io::Writer;

pub trait PlotWriter
{
	fn write_data<T: DataType>(&mut self, v: T);
	fn write_str(&mut self, s: &str);
	fn write_i32(&mut self, i: i32);
	fn write_float(&mut self, f: f64);
}

pub fn to_sci(v: f64, writer: &mut Writer)
{
	let e = v.abs();
	if(e > 0.0)
	{
		let e = e.log10().floor();
		write!(writer, "{}e{}", f64::to_str_digits(v / (10.0f64).pow(&e), 16), e);
	}
	else
	{
		write!(writer, "0.0");
	}
}

impl PlotWriter for MemWriter
{
	fn write_data<T: DataType>(&mut self, v: T)
	{
		self.write_le_f64(v.get());
	}

	fn write_str(&mut self, s: &str)
	{
		self.write(s.as_bytes());
	}
	
	fn write_i32(&mut self, i: i32)
	{
		let w = self as &mut Writer; 
		write!(w, "{}", i);
	}
	
	fn write_float(&mut self, f: f64)
	{
		to_sci(f, self as &mut Writer);
	}
}
