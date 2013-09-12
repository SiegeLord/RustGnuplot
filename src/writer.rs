// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use datatype::*;

use std::cast;
use std::float;

pub trait PlotWriter
{
	fn write_data<T: DataType>(&mut self, v: T);
	fn write_str(&mut self, s: &str);
	fn write_int(&mut self, i: int);
	fn write_float(&mut self, f: float);
}

pub fn to_sci(v: float, writer: &fn(&str))
{
	let e = v.abs();
	if(e > 0.0)
	{
		let e = e.log10().floor();
		writer(float::to_str_digits(v / (10.0f).pow(&e), 16));
		writer("e");
		writer(e.to_str());
	}
	else
	{
		writer("0.0");
	}
}

impl PlotWriter for ~[u8]
{
	fn write_data<T: DataType>(&mut self, v: T)
	{
		let f = v.get();
		let i: u64 = unsafe { cast::transmute(f) };
		
		self.push((i >> 0) as u8);
		self.push((i >> 8) as u8);
		self.push((i >> 16) as u8);
		self.push((i >> 24) as u8);
		self.push((i >> 32) as u8);
		self.push((i >> 40) as u8);
		self.push((i >> 48) as u8);
		self.push((i >> 56) as u8);
	}

	fn write_str(&mut self, s: &str)
	{
		self.push_all(s.as_bytes());
	}
	
	fn write_int(&mut self, i: int)
	{
		self.write_str(i.to_str());
	}
	
	fn write_float(&mut self, f: float)
	{
		do to_sci(f) |s| { self.write_str(s) };
	}
}
