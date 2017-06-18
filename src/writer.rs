// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.


use std::io::{self, Write};
use std::mem::transmute;

pub trait Writer: Write
{
	fn write_str(&mut self, s: &str) -> Result<(), io::Error>
	{
		self.write_all(s.as_bytes())
	}

	fn write_le_f64(&mut self, v: f64) -> Result<(), io::Error>
	{
		let vb: [u8; 8] = unsafe {
			let v: u64 = transmute(v);
			transmute(v.to_le())
		};
		self.write_all(&vb)
	}
}

impl<T: Write> Writer for T {}
