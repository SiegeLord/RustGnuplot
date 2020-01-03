// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use crate::error_types::*;

use self::AxesVariant::*;
use crate::axes2d::*;
use crate::axes3d::*;

use crate::options::{GnuplotVersion, MultiplotFillDirection, MultiplotFillOrder};
use crate::writer::Writer;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::{Child, Command, Stdio};
use std::str;

enum AxesVariant
{
	Axes2DType(Axes2D),
	Axes3DType(Axes3D),
	NewPage,
}

impl AxesVariant
{
	fn write_out(&self, writer: &mut dyn Writer, auto_layout: bool, version: GnuplotVersion)
	{
		match *self
		{
			Axes2DType(ref a) => a.write_out(writer, auto_layout, version),
			Axes3DType(ref a) => a.write_out(writer, auto_layout, version),
			NewPage =>
			{
				writeln!(writer, "unset multiplot");
				writeln!(writer, "set multiplot");
			}
		}
	}

	fn reset_state(&self, writer: &mut dyn Writer)
	{
		match *self
		{
			Axes2DType(ref a) => a.reset_state(writer),
			Axes3DType(ref a) => a.reset_state(writer),
			_ => (),
		}
	}
}

/// A struct that contains all the multiplot layout options
struct MultiplotOptions
{
	rows: usize,
	columns: usize,
	title: Option<String>,
	scale_x: Option<f32>,
	scale_y: Option<f32>,
	offset_x: Option<f32>,
	offset_y: Option<f32>,
	fill_order: Option<MultiplotFillOrder>,
	fill_direction: Option<MultiplotFillDirection>,
}

impl MultiplotOptions
{
	pub fn new() -> MultiplotOptions
	{
		MultiplotOptions {
			rows: 1,
			columns: 1,
			title: None,
			scale_x: None,
			scale_y: None,
			offset_x: None,
			offset_y: None,
			fill_order: None,
			fill_direction: None,
		}
	}
}

/// A figure that may contain multiple axes
pub struct Figure
{
	axes: Vec<AxesVariant>,
	terminal: String,
	enhanced_text: bool,
	output_file: String,
	post_commands: String,
	pre_commands: String,
	// RefCell so that we can echo to it
	gnuplot: RefCell<Option<Child>>,
	version: Option<GnuplotVersion>,
	multiplot_options: Option<MultiplotOptions>,
}

impl Default for GnuplotVersion
{
	fn default() -> GnuplotVersion
	{
		GnuplotVersion { major: 5, minor: 0 }
	}
}

impl Figure
{
	/// Creates a new figure
	pub fn new() -> Figure
	{
		Figure {
			axes: Vec::new(),
			terminal: "".into(),
			enhanced_text: true,
			output_file: "".into(),
			gnuplot: RefCell::new(None),
			post_commands: "".into(),
			pre_commands: "".into(),
			version: None,
			multiplot_options: None,
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
	/// * svg - Saves the figure as a SVG file
	/// * canvas - Saves the figure as an HTML5 canvas element
	///
	/// As of now you can hack the canvas size in by using "pngcairo size 600, 400" for `terminal`.
	/// Be prepared for that kludge to go away, though.
	pub fn set_terminal<'l>(&'l mut self, terminal: &str, output_file: &str) -> &'l mut Figure
	{
		self.terminal = terminal.into();
		self.output_file = output_file.into();
		self
	}

	/// Set or unset text enhancements
	pub fn set_enhanced_text<'l>(&'l mut self, enhanced: bool) -> &'l mut Figure
	{
		self.enhanced_text = enhanced;
		self
	}

	/// Sets commands to send to gnuplot after all the plotting commands.
	pub fn set_post_commands<'l>(&'l mut self, post_commands: &str) -> &'l mut Figure
	{
		self.post_commands = post_commands.into();
		self
	}

	/// Sets commands to send to gnuplot before any plotting commands.
	pub fn set_pre_commands<'l>(&'l mut self, pre_commands: &str) -> &'l mut Figure
	{
		self.pre_commands = pre_commands.into();
		self
	}

	/// Sets the Gnuplot version.
	///
	/// By default, we assume version 5.0. If `show` is called, it will attempt
	/// to parse Gnuplot's version string as well.
	pub fn set_gnuplot_version(&mut self, version: Option<GnuplotVersion>) -> &mut Figure
	{
		self.version = version;
		self
	}

	/// Returns the Gnuplot version.
	pub fn get_gnuplot_version(&self) -> GnuplotVersion
	{
		self.version.unwrap_or_default()
	}

	/// Define the layout for the multiple plots
	/// # Arguments
	/// * `rows` - Number of rows
	/// * `columns` - Number of columns
	pub fn set_multiplot_layout<'l>(&'l mut self, rows: usize, columns: usize) -> &'l mut Self
	{
		let multiplot_options = self
			.multiplot_options
			.get_or_insert(MultiplotOptions::new());
		multiplot_options.rows = rows;
		multiplot_options.columns = columns;

		self
	}

	/// Set the multiplot title
	/// # Arguments
	/// * `title` - Name of the file
	pub fn set_title<'l>(&'l mut self, title: &str) -> &'l mut Self
	{
		let multiplot_options = self
			.multiplot_options
			.get_or_insert(MultiplotOptions::new());
		multiplot_options.title = Some(title.into());

		self
	}

	/// Applies a horizontal and vertical scale to each plot
	/// # Arguments
	/// * `scale_x` - Horizonal scale applied to each plot
	/// * `scale_y` - Vertical scale applied to each plot
	pub fn set_scale<'l>(&'l mut self, scale_x: f32, scale_y: f32) -> &'l mut Self
	{
		let multiplot_options = self
			.multiplot_options
			.get_or_insert(MultiplotOptions::new());
		multiplot_options.scale_x = Some(scale_x);
		multiplot_options.scale_y = Some(scale_y);

		self
	}

	/// Applies a horizontal and vertical offset to each plot
	/// # Arguments
	/// * `offset_x` - Horizontal offset applied to each plot
	/// * `offset_y` - Horizontal offset applied to each plot
	pub fn set_offset<'l>(&'l mut self, offset_x: f32, offset_y: f32) -> &'l mut Self
	{
		let multiplot_options = self
			.multiplot_options
			.get_or_insert(MultiplotOptions::new());
		multiplot_options.offset_x = Some(offset_x);
		multiplot_options.offset_y = Some(offset_y);

		self
	}

	/// Defines the order in which plots fill the layout. Default is RowsFirst and Downwards.
	/// # Arguments
	/// * `order` - Options: RowsFirst, ColumnsFirst
	/// * `direction` - Options: Downwards, Upwards
	pub fn set_multiplot_fill_order<'l>(
		&'l mut self, order: MultiplotFillOrder, direction: MultiplotFillDirection,
	) -> &'l mut Self
	{
		let multiplot_options = self
			.multiplot_options
			.get_or_insert(MultiplotOptions::new());
		multiplot_options.fill_order = Some(order);
		multiplot_options.fill_direction = Some(direction);

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

	/// Creates a new page.
	///
	/// Some terminals support multiple pages or frames, e.g. to create an
	/// animation. Call this function between sets of plots to indicate that a
	/// new page should be started. Note that this is implicit before any
	/// `axes2d`/`axes3d` calls, so make sure to call this only between pages
	/// (not once before every page).
	pub fn new_page(&mut self) -> &mut Figure
	{
		self.axes.push(NewPage);
		self
	}

	/// Launch a gnuplot process, if it hasn't been spawned already by a call to
	/// this function, and display the figure on it.
	pub fn show(&mut self) -> Result<&mut Figure, GnuplotInitError>
	{
		if self.axes.len() == 0
		{
			return Ok(self);
		}

		if self.version.is_none()
		{
			let output = Command::new("gnuplot").arg("--version").output()?;

			if let Ok(version_string) = str::from_utf8(&output.stdout)
			{
				let parts: Vec<_> = version_string.split(|c| c == ' ' || c == '.').collect();
				if parts.len() > 2 && parts[0] == "gnuplot"
				{
					if let (Ok(major), Ok(minor)) = (
						i32::from_str_radix(parts[1], 10),
						i32::from_str_radix(parts[2], 10),
					)
					{
						self.version = Some(GnuplotVersion {
							major: major,
							minor: minor,
						});
					}
				}
			}
		}

		if self.gnuplot.borrow().is_none()
		{
			*self.gnuplot.borrow_mut() = Some(
				Command::new("gnuplot")
					.arg("-p")
					.stdin(Stdio::piped())
					.spawn()
					.expect(
						"Couldn't spawn gnuplot. Make sure it is installed and available in PATH.",
					),
			);
		}

		self.gnuplot.borrow_mut().as_mut().map(|p| {
			let stdin = p.stdin.as_mut().expect("No stdin!?");
			self.echo(stdin);
			stdin.flush();
		});

		Ok(self)
	}

	/// Save the figure to a png file.
	///
	/// # Arguments
	/// * `file_path` - Path to the output file (png)
	/// * `width_px` - output image width (in pixels)
	/// * `height_px` - output image height (in pixels)
	pub fn save_to_png(
		&mut self, file_path: &str, width_px: u32, height_px: u32,
	) -> Result<(), GnuplotInitError>
	{
		let former_term = self.terminal.clone();
		let former_output_file = self.output_file.clone();
		self.terminal = format!("pngcairo size {},{}", width_px, height_px);
		self.output_file = file_path.into();
		self.show()?.close();
		self.terminal = former_term;
		self.output_file = former_output_file;

		Ok(())
	}

	/// Save the figure to a svg file.
	///
	/// # Arguments
	/// * `file_path` - Path to the output file (svg)
	/// * `width_px` - output image width (in pixels)
	/// * `height_px` - output image height (in pixels)
	pub fn save_to_svg(
		&mut self, file_path: &str, width_px: u32, height_px: u32,
	) -> Result<(), GnuplotInitError>
	{
		let former_term = self.terminal.clone();
		let former_output_file = self.output_file.clone();
		self.terminal = format!("svg size {},{}", width_px, height_px);
		self.output_file = file_path.into();
		self.show()?.close();
		self.terminal = former_term;
		self.output_file = former_output_file;

		Ok(())
	}

	/// Save the figure to a pdf file.
	///
	/// # Arguments
	/// * `file_path` - Path to the output file (pdf)
	/// * `width_in` - output image width (in inches)
	/// * `height_in` - output image height (in inches)
	pub fn save_to_pdf(
		&mut self, file_path: &str, width_in: u32, height_in: u32,
	) -> Result<(), GnuplotInitError>
	{
		let former_term = self.terminal.clone();
		let former_output_file = self.output_file.clone();
		self.terminal = format!("pdfcairo size {},{}", width_in, height_in);
		self.output_file = file_path.into();
		self.show()?.close();
		self.terminal = former_term;
		self.output_file = former_output_file;

		Ok(())
	}

	/// Save the figure to an eps file
	///
	/// # Arguments
	/// * `file_path` - Path to the output file (eps)
	/// * `width_in` - output image width (in inches)
	/// * `height_in` - output image height (in inches)
	pub fn save_to_eps(
		&mut self, file_path: &str, width_in: u32, height_in: u32,
	) -> Result<(), GnuplotInitError>
	{
		let former_term = self.terminal.clone();
		let former_output_file = self.output_file.clone();
		self.terminal = format!("epscairo size {},{}", width_in, height_in);
		self.output_file = file_path.into();
		self.show()?.close();
		self.terminal = former_term;
		self.output_file = former_output_file;

		Ok(())
	}

	/// Save the figure to a HTML5 canvas file
	///
	/// # Arguments
	/// * `file_path` - Path to the output file (canvas)
	/// * `width_px` - output image width (in pixels)
	/// * `height_px` - output image height (in pixels)
	pub fn save_to_canvas(
		&mut self, file_path: &str, width_px: u32, height_px: u32,
	) -> Result<(), GnuplotInitError>
	{
		let former_term = self.terminal.clone();
		let former_output_file = self.output_file.clone();
		self.terminal = format!("canvas size {},{}", width_px, height_px);
		self.output_file = file_path.into();
		self.show()?.close();
		self.terminal = former_term;
		self.output_file = former_output_file;

		Ok(())
	}

	/// Closes the gnuplot process.
	///
	/// This can be useful if you're your plot output is a file and you need to
	/// that it was written.
	pub fn close(&mut self) -> &mut Figure
	{
		if self.gnuplot.borrow().is_none()
		{
			return self;
		}

		{
			let mut gnuplot = self.gnuplot.borrow_mut();

			gnuplot.as_mut().map(|p| {
				{
					let stdin = p.stdin.as_mut().expect("No stdin!?");
					writeln!(stdin, "quit");
				}
				p.wait();
			});
			*gnuplot = None;
		}

		self
	}

	/// Clears all axes on this figure.
	pub fn clear_axes(&mut self) -> &mut Figure
	{
		self.axes.clear();
		self
	}

	/// Echo the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * `writer` - A function pointer that will be called multiple times with the command text and data
	pub fn echo<'l, T: Writer>(&'l self, writer: &mut T) -> &'l Figure
	{
		let w = writer as &mut dyn Writer;
		writeln!(w, "{}", &self.pre_commands);

		if self.axes.len() == 0
		{
			return self;
		}

		writeln!(w, "set encoding utf8");
		if self.terminal.len() > 0
		{
			writeln!(w, "set terminal {}", self.terminal);
		}

		if self.output_file.len() > 0
		{
			writeln!(w, "set output \"{}\"", self.output_file);
		}

		writeln!(w, "set termoption dashed");
		writeln!(
			w,
			"set termoption {}",
			if self.enhanced_text
			{
				"enhanced"
			}
			else
			{
				"noenhanced"
			}
		);

		let mut multiplot_options_string = "".to_string();
		if let Some(m) = &self.multiplot_options
		{
			let fill_order = match m.fill_order
			{
				None => "",
				Some(fo) => match fo
				{
					MultiplotFillOrder::RowsFirst => " rowsfirst",
					MultiplotFillOrder::ColumnsFirst => " columnsfirst",
				},
			};

			let fill_direction = match m.fill_direction
			{
				None => "",
				Some(fd) => match fd
				{
					MultiplotFillDirection::Downwards => " downwards",
					MultiplotFillDirection::Upwards => " upwards",
				},
			};

			let title = m
				.title
				.as_ref()
				.map_or("".to_string(), |t| format!(" title \"{}\"", t));
			let scale = m.scale_x.map_or("".to_string(), |s| {
				format!(" scale {},{}", s, m.scale_y.unwrap())
			});
			let offset = m.offset_x.map_or("".to_string(), |o| {
				format!(" offset {},{}", o, m.offset_y.unwrap())
			});

			multiplot_options_string = format!(
				" layout {},{}{}{}{}{}{}",
				m.rows, m.columns, fill_order, fill_direction, title, scale, offset
			);
		}

		writeln!(w, "set multiplot{}", multiplot_options_string);

		// TODO: Maybe add an option for this (who seriously prefers them in the back though?)
		writeln!(w, "set tics front");

		let mut prev_e: Option<&AxesVariant> = None;
		for e in self.axes.iter()
		{
			if let Some(prev_e) = prev_e
			{
				prev_e.reset_state(w);
			}
			e.write_out(
				w,
				self.multiplot_options.is_some(),
				self.get_gnuplot_version(),
			);
			prev_e = Some(e);
		}

		writeln!(w, "unset multiplot");
		writeln!(w, "{}", &self.post_commands);
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

#[test]
fn flush_test()
{
	use std::fs;
	use tempfile::TempDir;

	let tmp_path = TempDir::new().unwrap().into_path();
	let file_path = tmp_path.join("plot.png");
	let mut fg = Figure::new();
	fg.axes2d().boxes(0..5, 0..5, &[]);
	fg.set_terminal("pngcairo", &*file_path.to_string_lossy());
	fg.show().unwrap().close();
	fs::read(file_path).unwrap();
	fs::remove_dir_all(&tmp_path);
}
