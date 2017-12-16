// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.


use self::AxesVariant::*;
use axes2d::*;
use axes3d::*;

use axes_common::*;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::{Child, Command, Stdio};
use writer::Writer;

enum AxesVariant
{
	Axes2DType(Axes2D),
	Axes3DType(Axes3D),
}

impl AxesVariant
{
	fn write_out(&self, writer: &mut Writer)
	{
		match *self
		{
			Axes2DType(ref a) => a.write_out(writer),
			Axes3DType(ref a) => a.write_out(writer),
		}
	}

	fn get_common_data(&self) -> &AxesCommonData
	{
		match *self
		{
			Axes2DType(ref a) => a.get_common_data(),
			Axes3DType(ref a) => a.get_common_data(),
		}
	}
}

/// A figure that may contain multiple axes
pub struct Figure
{
	axes: Vec<AxesVariant>,
	terminal: String,
	output_file: String,
	// RefCell so that we can echo to it
	gnuplot: RefCell<Option<Child>>,
}

impl Figure
{
	/// Creates a new figure
	pub fn new() -> Figure
	{
		Figure { axes: Vec::new(), terminal: "".to_string(), output_file: "".to_string(), gnuplot: RefCell::new(None) }
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
	///
	/// As of now you can hack the canvas size in by using "pngcairo size 600, 400" for `terminal`.
	/// Be prepared for that kludge to go away, though.
	pub fn set_terminal<'l>(&'l mut self, terminal: &str, output_file: &str) -> &'l mut Figure
	{
		self.terminal = terminal.to_string();
		self.output_file = output_file.to_string();
		self
	}

	/// Creates a set of 2D axes
	pub fn axes2d(&mut self) -> &mut Axes2D
	{
		self.axes.push(Axes2DType(Axes2D::new()));
		let l = self.axes.len();
		match *&mut self.axes[l - 1]
		{
			Axes2DType(ref mut a) => a,
			_ => unreachable!(),
		}
	}

	/// Creates a set of 3D axes
	pub fn axes3d(&mut self) -> &mut Axes3D
	{
		self.axes.push(Axes3DType(Axes3D::new()));
		let l = self.axes.len();
		match *&mut self.axes[l - 1]
		{
			Axes3DType(ref mut a) => a,
			_ => unreachable!(),
		}
	}

	/// Launch a gnuplot process, if it hasn't been spawned already by a call to
	/// this function, and display the figure on it.
	pub fn show(&mut self) -> &Figure
	{
		if self.axes.len() == 0
		{
			return self;
		}

		if self.gnuplot.borrow().is_none()
		{
			*self.gnuplot.borrow_mut() = Some(
				Command::new("gnuplot")
					.arg("-p")
					.stdin(Stdio::piped())
					.spawn()
					.ok()
					.expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH."),
			);
		}

		self.gnuplot.borrow_mut().as_mut().map(|p| { self.echo(p.stdin.as_mut().expect("No stdin!?")); });

		self
	}

	/// Clears all axes on this figure.
	pub fn clear_axes(&mut self) -> &Figure
	{
		self.axes.clear();
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
		if self.axes.len() > 1
		{
			writeln!(w, "set multiplot");
		}
		// TODO: Maybe add an option for this (who seriously prefers them in the back though?)
		writeln!(w, "set tics front");

		for e in self.axes.iter()
		{
			writeln!(w, "reset");

			let c = e.get_common_data();
			c.grid_pos.map(|pos| {
				let width = 1.0 / (c.grid_cols as f64);
				let height = 1.0 / (c.grid_rows as f64);
				let x = (pos % c.grid_cols) as f64 * width;
				let y = 1.0 - (1.0 + (pos / c.grid_cols) as f64) * height;

				writeln!(w, "set origin {:.12e},{:.12e}", x, y);
				writeln!(w, "set size {:.12e},{:.12e}", width, height);
			});
			e.write_out(w);
		}

		if self.axes.len() > 1
		{
			writeln!(w, "unset multiplot");
		}
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

		let mut file = BufWriter::new(File::create(filename).unwrap());
		self.echo(&mut file);
		file.flush();
		self
	}
}
