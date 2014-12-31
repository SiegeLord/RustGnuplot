// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;
use internal::axes2d::*;
use internal::axes3d::*;

use std::io::File;
use std::io::BufferedWriter;
use std::path::Path;
use std::io::process::Command;

pub use self::AxesVariant::*;

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
			Axes3DType(ref a) => a.write_out(writer)
		}
	}
	
	fn get_common_data<'l>(&'l self) -> &'l AxesCommonData
	{
		match *self
		{
			Axes2DType(ref a) => a.get_common_data(),
			Axes3DType(ref a) => a.get_common_data()
		}
	}
}

/// A figure that may contain multiple axes
pub struct Figure
{
	axes: Vec<AxesVariant>,
	terminal: String,
	output_file: String
}

impl Figure
{
	/// Creates a new figure
	pub fn new() -> Figure
	{
		Figure
		{
			axes: Vec::new(),
			terminal: "".to_string(),
			output_file: "".to_string()
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
	pub fn set_terminal<'l>(&'l mut self, terminal: &str, output_file: &str) -> &'l mut Figure
	{
		self.terminal = terminal.to_string();
		self.output_file = output_file.to_string();
		self
	}
	
	/// Creates a set of 2D axes
	pub fn axes2d(&mut self) -> &mut Axes2D
	{
		self.axes.push(Axes2DType(new_axes2d()));
		let l = self.axes.len();
		match self.axes.as_mut_slice()[l - 1]
		{
			Axes2DType(ref mut a) => a,
			_ => panic!()
		}
	}
	
	/// Creates a set of 3D axes
	pub fn axes3d(&mut self) -> &mut Axes3D
	{
		self.axes.push(Axes3DType(new_axes3d()));
		let l = self.axes.len();
		match self.axes.as_mut_slice()[l - 1]
		{
			Axes3DType(ref mut a) => a,
			_ => panic!()
		}
	}
	
	/// Launch a gnuplot process and display the figure on it
	pub fn show(&self) -> &Figure
	{
		if self.axes.len() == 0
		{
			return self;
		}
		
		let mut p = Command::new("gnuplot").arg("-p").spawn().ok().expect("Couldn't spawn gnuplot");
		self.echo(p.stdin.as_mut().unwrap());
		
		self
	}
	
	/// Echo the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * `writer` - A function pointer that will be called multiple times with the command text and data
	pub fn echo<'l, T: Writer>(&'l self, writer: &mut T) -> &'l Figure
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
		
		for e in self.axes.iter()
		{
			writeln!(w, "reset");

			let c = e.get_common_data();
			c.grid_pos.map(|pos|
			{
				let width = 1.0 / (c.grid_cols as f64);
				let height = 1.0 / (c.grid_rows as f64);
				let x = (pos % c.grid_cols) as f64 * width;
				let y = 1.0 - (1.0 + (pos / c.grid_cols) as f64) * height;
				
				writeln!(w, "set origin {:.12e},{:.12e}", x, y);
				writeln!(w, "set size {:.12e},{:.12e}", width, height);
			});
			e.write_out(w);
		}
		
		writeln!(w, "unset multiplot");
		self
	}
	
	/// Save to a file the the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * `filename` - Name of the file
	pub fn echo_to_file<'l>(&'l self, filename: &str) -> &'l Figure
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
