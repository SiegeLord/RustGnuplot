// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;
use internal::axes2d::*;
use internal::axes3d::*;
use writer::*;

use std::rt::io::File;
use std::rt::io::buffered::BufferedWriter;
use std::path::Path;
use std::run::{Process, ProcessOptions};

enum AxesVariant
{
	Axes2DType(Axes2D),
	Axes3DType(Axes3D)
}

impl AxesVariant
{
	fn write_out(&self, writer: &fn(data: &[u8]))
	{
		match *self
		{
			Axes2DType(ref a) => a.write_out(writer),
			Axes3DType(_) => ()
		}
	}
	
	fn get_common<'l>(&'l self) -> &'l AxesCommon
	{
		match *self
		{
			Axes2DType(ref a) => a.get_common(),
			Axes3DType(ref a) => a.get_common()
		}
	}
}

/// A figure that may contain multiple axes
pub struct Figure<'self>
{
	priv axes: ~[AxesVariant],
	priv num_rows: u32,
	priv num_cols: u32,
	priv terminal: &'self str,
	priv output_file: &'self str
}

impl<'self> Figure<'self>
{
	/// Creates a new figure
	pub fn new() -> Figure
	{
		Figure
		{
			axes: ~[],
			num_rows: 0,
			num_cols: 0,
			terminal: "",
			output_file: ""
		}
	}
	
	/// Sets the terminal for gnuplot to use, as well as the file to output the figure to.
	/// Terminals that spawn a GUI don't need an output file, so pass an empty string for those.
	///
	/// There are a quite a number of terminals, here are some commonly used ones:
	///
	/// * wxt - Interactive GUI
	/// * pdfcairo - Saves the figure as a PDF file
	/// * epscairo - Saves the figure as a EPS file
	/// * pngcairo - Saves the figure as a PNG file
	pub fn set_terminal<'l>(&'l mut self, terminal: &'self str, output_file: &'self str) -> &'l mut Figure<'self>
	{
		self.terminal = terminal;
		self.output_file = output_file;
		self
	}
	
	/// Sets the dimensions of the grid that you can use to
	/// place multiple axes on
	/// # Arguments
	/// * `rows` - Number of rows. Set to 0 to disable the grid
	/// * `cols` - Number of columns. Set to 0 to disable the grid
	pub fn set_grid<'l>(&'l mut self, rows: u32, cols: u32) -> &'l mut Figure<'self>
	{
		self.num_rows = rows;
		self.num_cols = cols;
		self
	}
	
	
	/// Creates a set of 2D axes
	pub fn axes2d<'l>(&'l mut self) -> &'l mut Axes2D
	{
		self.axes.push(Axes2DType(new_axes2d()));
		match self.axes[self.axes.len() - 1]
		{
			Axes2DType(ref mut a) => a,
			_ => fail!()
		}
	}
	
	/// Creates a set of 3D axes
	pub fn axes3d<'l>(&'l mut self) -> &'l mut Axes3D
	{
		self.axes.push(Axes3DType(new_axes3d()));
		match self.axes[self.axes.len() - 1]
		{
			Axes3DType(ref mut a) => a,
			_ => fail!()
		}
	}
	
	/// Launch a gnuplot process and display the figure on it
	pub fn show<'l>(&'l self) -> &'l Figure<'l>
	{
		if self.axes.len() == 0
		{
			return self;
		}
		
		let mut p = Process::new("gnuplot", [~"-p"], ProcessOptions::new());
		let input = p.input();
		
		do self.echo |v|
		{
			input.write(v);
		};
		
		self
	}
	
	/// Echo the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * `writer` - A function pointer that will be called multiple times with the command text and data
	pub fn echo<'l>(&'l self, writer: &fn(data: &[u8])) -> &'l Figure<'l>
	{
		if self.axes.len() == 0
		{
			return self;
		}
		
		if self.terminal.len() > 0
		{
			writer("set terminal ".as_bytes());
			writer(self.terminal.as_bytes());
			writer("\n".as_bytes());
		}
		
		if self.output_file.len() > 0
		{
			writer("set output \"".as_bytes());
			writer(self.output_file.as_bytes());
			writer("\"\n".as_bytes());
		}
		
		writer("set termoption dashed\n".as_bytes());
		writer("set termoption enhanced\n".as_bytes());
		writer("set multiplot\n".as_bytes());
		
		let do_layout = self.num_rows > 0 && self.num_cols > 0;
		
		let (w, h) = if do_layout
		{
			(1.0 / (self.num_cols as f64), 1.0 / (self.num_rows as f64))
		}
		else
		{
			(0.0, 0.0)
		};
		
		for e in self.axes.iter()
		{
			if do_layout
			{
				let c = e.get_common();
				let x = (c.grid_col as f64 - 1.0) * w;
				let y = (self.num_rows as f64 - c.grid_row as f64) * h;
				
				writer("set origin ".as_bytes());
				do to_sci(x) |s| { writer(s.as_bytes()) };
				writer(",".as_bytes());
				do to_sci(y) |s| { writer(s.as_bytes()) };
				writer("\n".as_bytes());
				
				writer("set size ".as_bytes());
				do to_sci(w) |s| { writer(s.as_bytes()) };
				writer(",".as_bytes());
				do to_sci(h) |s| { writer(s.as_bytes()) };
				writer("\n".as_bytes());
			}
			e.write_out(|s| writer(s));
		}
		
		writer("unset multiplot\n".as_bytes());
		self
	}
	
	/// Save to a file the the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * `filename` - Name of the file
	pub fn echo_to_file<'l>(&'l self, filename: &str) -> &'l Figure<'l>
	{
		if self.axes.len() == 0
		{
			return self;
		}
		
		let mut file = BufferedWriter::new(File::create(&Path::new(filename)).unwrap());
		do self.echo |v|
		{
			file.write(v);
		};
		self
	}
}
