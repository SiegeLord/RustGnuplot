// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;
use internal::axes2d::*;
use internal::axes3d::*;
use writer::*;

use std::io::File;
use std::io::buffered::BufferedWriter;
use std::path::Path;
use std::run::{Process, ProcessOptions};

enum AxesVariant
{
	Axes2DType(Axes2D),
	Axes3DType(Axes3D)
}

impl AxesVariant
{
	fn write_out(&self, writer: &mut Writer)
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
pub struct Figure<'l>
{
	priv axes: ~[AxesVariant],
	priv num_rows: u32,
	priv num_cols: u32,
	priv terminal: &'l str,
	priv output_file: &'l str
}

impl<'m> Figure<'m>
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
	pub fn set_terminal<'l>(&'l mut self, terminal: &'m str, output_file: &'m str) -> &'l mut Figure<'m>
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
	pub fn set_grid<'l>(&'l mut self, rows: u32, cols: u32) -> &'l mut Figure<'m>
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
		let mut input = p.input();
		
		self.echo(&mut input);
		
		self
	}
	
	/// Echo the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * `writer` - A function pointer that will be called multiple times with the command text and data
	pub fn echo<'l, T: Writer>(&'l self, writer: &mut T) -> &'l Figure<'l>
	{
		let w = writer as &mut Writer;
		
		if self.axes.len() == 0
		{
			return self;
		}
		
		if self.terminal.len() > 0
		{
			writeln!(w, "set terminal {}", self.terminal);
		}
		
		if self.output_file.len() > 0
		{
			writeln!(w, "set output \"{}\"", self.output_file);
		}
		
		writeln!(w, "set termoption dashed");
		writeln!(w, "set termoption enhanced");
		writeln!(w, "set multiplot");
		// TODO: Maybe add an option for this (who seriously prefers them in the back though?)
		writeln!(w, "set tics front");
		
		let do_layout = self.num_rows > 0 && self.num_cols > 0;
		
		let (width, height) = if do_layout
		{
			(1.0 / (self.num_cols as f64), 1.0 / (self.num_rows as f64))
		}
		else
		{
			(0.0, 0.0)
		};
		
		for e in self.axes.iter()
		{
			writeln!(w, "reset");
			if do_layout
			{
				let c = e.get_common();
				let x = (c.grid_col as f64 - 1.0) * width;
				let y = (self.num_rows as f64 - c.grid_row as f64) * height;
				
				write!(w, "set origin ");
				to_sci(x, w);
				write!(w, ",");
				to_sci(y, w);
				write!(w, "\n");
				
				write!(w, "set size ");
				to_sci(width, w);
				write!(w, ",");
				to_sci(height, w);
				write!(w, "\n");
			}
			e.write_out(w);
		}
		
		writeln!(w, "unset multiplot");
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
		self.echo(&mut file);
		file.flush();
		self
	}
}
