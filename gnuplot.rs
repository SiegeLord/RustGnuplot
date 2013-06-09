#[link(name = "gnuplot",
       vers = "0.1",
       author = "SiegeLord",
       url = "https://github.com/SiegeLord/RustGnuplot")];

#[comment = "Rust gnuplot controller"];
#[license = "zlib"];
#[crate_type = "lib"];

use std::iterator::*;
use std::cast;
use std::str;
use std::float;
use std::io;
use std::run::{Process, ProcessOptions};

/// An enumeration of plot options you can supply to plotting commands, governing
/// things like line width, color and others
pub enum PlotOption<'self>
{
	/// Sets the symbol used for points. The characters are as follows:
	/// * ```.``` - dot
	/// * ```+``` - plus
	/// * ```x``` - cross
	/// * ```*``` - star
	/// * ```s``` - empty square
	/// * ```S``` - filled square
	/// * ```o``` - empty circle
	/// * ```O``` - filled circle
	/// * ```t``` - empty triangle
	/// * ```T``` - filled triangle
	/// * ```d``` - empty del (upside down triangle)
	/// * ```D``` - filled del (upside down triangle)
	/// * ```r``` - empty rhombus
	/// * ```R``` - filled rhombus
	PointSymbol(char),
	/// Sets the caption of the plot element. Set to empty to hide it from the legend.
	Caption(&'self str),
	/// Sets the width of lines.
	LineWidth(float),
	/// Sets the color of the plot element. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white).
	Color(&'self str),
	/// Sets the dash type. Note that not all gnuplot terminals support dashed lines. See [DashType](#enum-dashtype) for the available types.
	LineDash(DashType)
}

/// An enumeration of possible dash styles
pub enum DashType
{
	Solid,
	SmallDot,
	Dot,
	Dash,
	DotDash,
	DotDotDash
}

/// An enumeration of something that can either be fixed (e.g. the maximum of X values),
/// or automatically determined
pub enum AutoOption<T>
{
	/// Fixes the value to a specific value
	Fix(T),
	/// Lets the value scale automatically
	Auto
}

struct PlotElement
{
	args : ~[u8],
	data : ~[u8]
}

impl PlotElement
{
	fn new() -> PlotElement
	{
		PlotElement
		{
			args : ~[],
			data : ~[],
		}
	}
}

trait DataType
{
	fn get(&self) -> float;
}

macro_rules! impl_data_type
(
	($T:ty) =>
	(
		impl<'self> DataType for &'self $T
		{
			fn get(&self) -> float
			{
				self.to_float()
			}
		}
	)
)

macro_rules! impl_data_type_ref
(
	($T:ty) =>
	(
		impl DataType for $T
		{
			fn get(&self) -> float
			{
				self.to_float()
			}
		}
	)
)

impl_data_type!(u8)
impl_data_type!(u16)
impl_data_type!(u32)
impl_data_type!(u64)
impl_data_type!(uint)

impl_data_type!(i8)
impl_data_type!(i16)
impl_data_type!(i32)
impl_data_type!(i64)
impl_data_type!(int)

impl_data_type!(f32)
impl_data_type!(f64)
impl_data_type!(float)

impl_data_type_ref!(u8)
impl_data_type_ref!(u16)
impl_data_type_ref!(u32)
impl_data_type_ref!(u64)
impl_data_type_ref!(uint)

impl_data_type_ref!(i8)
impl_data_type_ref!(i16)
impl_data_type_ref!(i32)
impl_data_type_ref!(i64)
impl_data_type_ref!(int)

impl_data_type_ref!(f32)
impl_data_type_ref!(f64)
impl_data_type_ref!(float)

trait Writable
{
	priv fn write_data<T : DataType>(&mut self, v : T);
	priv fn write_str(&mut self, s : &str);
	priv fn write_int(&mut self, i : int);
	priv fn write_float(&mut self, f : float);
}

fn to_sci(v: float, writer : &fn(&str))
{
	let e = v.abs().log(10.0).floor();
	writer(float::to_str_digits(v / (10.0f).pow(e), 16) + "e" + e.to_str());
}

impl Writable for ~[u8]
{
	priv fn write_data<T : DataType>(&mut self, v : T)
	{
		let f = v.get();
		let i : u64 = unsafe { cast::transmute(f) };
		
		self.push((i >> 0) as u8);
		self.push((i >> 8) as u8);
		self.push((i >> 16) as u8);
		self.push((i >> 24) as u8);
		self.push((i >> 32) as u8);
		self.push((i >> 40) as u8);
		self.push((i >> 48) as u8);
		self.push((i >> 56) as u8);
	}

	priv fn write_str(&mut self, s : &str)
	{
		do str::byte_slice(s) |v| { self.push_all(v) }
	}
	
	priv fn write_int(&mut self, i : int)
	{
		self.write_str(i.to_str());
	}
	
	priv fn write_float(&mut self, f : float)
	{
		do to_sci(f) |s| { self.write_str(s) };
	}
}

enum PlotStyle
{
	Lines,
	Points
}

struct AxesCommon
{
	commands: ~[u8],
	elems : ~[PlotElement],
	grid_row : uint,
	grid_col : uint
}

impl AxesCommon
{
	fn new() -> AxesCommon
	{
		AxesCommon
		{
			commands: ~[],
			elems: ~[],
			grid_row: 0,
			grid_col: 0,
		}
	}
	
	fn write_common_commands(&mut self, elem_idx : uint, num_rows : int, num_cols : int, style : PlotStyle, options : &[PlotOption])
	{
		let args = &mut self.elems[elem_idx].args;
		args.write_str(" \"-\" binary endian=little record=");
		args.write_int(num_rows);
		args.write_str(" format=\"%float64\" using ");
		
		let mut col_idx : int = 1;
		while(col_idx < num_cols + 1)
		{
			args.write_int(col_idx);
			if(col_idx < num_cols)
			{
				args.write_str(":");
			}
			col_idx += 1;
		}
		
		args.write_str(" with ");
		let style_str = match style
		{
			Lines => "lines",
			Points => "points"
		};
		args.write_str(style_str);
		
		match style
		{
			Lines =>
			{
				for options.each() |o|
				{
					match *o
					{
						LineWidth(w) =>
						{
							args.write_str(" lw ");
							args.write_float(w);
							break;
						},
						_ => ()
					};
				}
				
				for options.each() |o|
				{
					match *o
					{
						LineDash(d) =>
						{
							args.write_str(" lt ");
							let ds : int = match d
							{
								Solid => 1,
								SmallDot => 0,
								Dash => 2,
								Dot => 3,
								DotDash => 4,
								DotDotDash => 5
							};
							args.write_int(ds);
							break;
						},
						_ => ()
					};
				}
			}
			Points =>
			{
				for options.each() |o|
				{
					match *o
					{
						PointSymbol(t) =>
						{
							let typ : int = match t
							{
								'.' => 0,
								'+' => 1,
								'x' => 2,
								'*' => 3,
								's' => 4,
								'S' => 5,
								'o' => 6,
								'O' => 7,
								't' => 8,
								'T' => 9,
								'd' => 10,
								'D' => 11,
								'r' => 12,
								'R' => 13,
								a => fail!("Invalid symbol %c", a)
							};
							args.write_str(" pt ");
							args.write_int(typ);
							break;
						},
						_ => ()
					};
				}
			}
		}
		
		for options.each() |o|
		{
			match *o
			{
				Color(s) =>
				{
					args.write_str(" lc rgb \"");
					args.write_str(s);
					args.write_str("\"");
					break;
				},
				_ => ()
			};
		}
		
		for options.each() |o|
		{
			match *o
			{
				Caption(s) =>
				{
					args.write_str(" t \"");
					args.write_str(s);
					args.write_str("\"");
					break;
				},
				_ => ()
			};
		}
	}
}

struct Axes2D
{
	common : AxesCommon
}

/// 2D axes that is used for drawing 2D plots
impl Axes2D
{
	/// Set the position of the axes on the figure using grid coordinates
	/// # Arguments
	/// * row - Row on the grid. Top-most row is 1
	/// * column - Column on the grid. Left-most column is 1
	pub fn set_pos_grid<'l>(&'l mut self, row : uint, col : uint) -> &'l mut Axes2D
	{
		self.common.grid_row = row;
		self.common.grid_col = col;
		self
	}
	
	/// Set the position of the axes on the figure using screen coordinates. 
	/// The coordinates refer to the bottom-left corner of the axes
	/// # Arguments
	/// * x - X position. Ranges from 0 to 1
	/// * y - Y position. Ranges from 0 to 1
	pub fn set_pos<'l>(&'l mut self, x : float, y : float) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set origin ");
			c.write_float(x);
			c.write_str(",");
			c.write_float(y);
			c.write_str("\n");
		}
		self
	}
	
	/// Set the size of the axes
	/// # Arguments
	/// * w - Width. Ranges from 0 to 1
	/// * h - Height. Ranges from 0 to 1
	pub fn set_size<'l>(&'l mut self, w : float, h : float) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set size ");
			c.write_float(w);
			c.write_str(",");
			c.write_float(h);
			c.write_str("\n");
		}
		self
	}
	
	/// Set the aspect ratio of the axes
	/// # Arguments
	/// * ratio - The aspect ratio. Set to Auto to return the ratio to default
	pub fn set_aspect_ratio<'l>(&'l mut self, ratio : AutoOption<float>) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			match ratio
			{
				Fix(r) => 
				{
					c.write_str("set size ratio ");
					c.write_float(r);
				},
				Auto =>
				{
					c.write_str("set size noratio");
				}
			}
			c.write_str("\n");
		}
		self
	}
	
	/// Set the label for the X axis
	/// # Arguments
	/// * text - Text of the label. Pass an empty string to hide the label
	pub fn set_x_label<'l>(&'l mut self, text : &str) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set xlabel \"");
			c.write_str(text);
			c.write_str("\"\n");
		}
		self
	}
	
	/// Set the label for the Y axis
	/// # Arguments
	/// * text - Text of the label. Pass an empty string to hide the label
	pub fn set_y_label<'l>(&'l mut self, text : &str) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set ylabel \"");
			c.write_str(text);
			c.write_str("\"\n");
		}
		self
	}
	
	/// Set the range of values for the X axis
	/// # Arguments
	/// * min - Minimum X value
	/// * max - Maximum X value
	pub fn set_x_range<'l>(&'l mut self, min : AutoOption<float>, max : AutoOption<float>) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set xrange [");
			match min
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str(":");
			match max
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str("]\n");
		}
		self
	}
	
	/// Set the range of values for the Y axis
	/// # Arguments
	/// * min - Minimum Y value
	/// * max - Maximum Y value
	pub fn set_y_range<'l>(&'l mut self, min : AutoOption<float>, max : AutoOption<float>) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set yrange [");
			match min
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str(":");
			match max
			{
				Fix(v) => c.write_float(v),
				Auto => c.write_str("*")
			}
			c.write_str("]\n");
		}
		self
	}
	
	/// Set the title for the axes
	/// # Arguments
	/// * text - Text of the title. Pass an empty string to hide the title
	pub fn set_title<'l>(&'l mut self, text : &str) -> &'l mut Axes2D
	{
		{
			let c = &mut self.common.commands;
			
			c.write_str("set title \"");
			c.write_str(text);
			c.write_str("\"\n");
		}
		self
	}
	
	/// Plot a 2D scatter-plot with lines connecting each data point
	/// # Arguments
	/// * x - Iterator for the x values
	/// * y - Iterator for the y values
	/// * options - Array of [PlotOption](#enum-plotoption) controlling the appearance of the plot element
	pub fn lines<'l, Tx : DataType, Ty : DataType, X : Iterator<Tx>, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Lines, x, y, options);
		self
	}
	
	/// Plot a 2D scatter-plot with a point standing in for each data point
	/// # Arguments
	/// * x - Iterator for the x values
	/// * y - Iterator for the y values
	/// * options - Array of [PlotOption](#enum-plotoption) controlling the appearance of the plot element
	pub fn points<'l, Tx : DataType, Ty : DataType, X : Iterator<Tx>, Y : Iterator<Ty>>(&'l mut self, x : X, y : Y, options : &[PlotOption]) -> &'l mut Axes2D
	{
		self.plot2(Points, x, y, options);
		self
	}
}

impl Axes2D
{
	fn new() -> Axes2D
	{
		Axes2D
		{
			common : AxesCommon::new()
		}
	}
	
	fn plot2<Tx : DataType, Ty : DataType, X : Iterator<Tx>, Y : Iterator<Ty>>(&mut self, style : PlotStyle, x : X, y : Y, options : &[PlotOption])
	{
		let l = self.common.elems.len();
		self.common.elems.push(PlotElement::new());
		let mut num_rows : int = 0;
		
		{
			let data = &mut self.common.elems[l].data;
			for x.zip(y).advance |(x, y)|
			{
				data.write_data(x);
				data.write_data(y);
				num_rows += 1;
			}
		}
		
		self.common.write_common_commands(l, num_rows, 2, style, options);
	}
	
	fn write_out(&self, writer : &fn(data : &[u8]))
	{
		if self.common.elems.len() == 0
		{
			return;
		}
		
		writer(self.common.commands);

		str::byte_slice("plot", writer);
		
		let mut first = true;
		for self.common.elems.each() |e|
		{
			if !first
			{
				str::byte_slice(",",  writer)
			}
			writer(e.args);
			first = false;
		}
		
		str::byte_slice("\n", writer);
		
		for self.common.elems.each() |e|
		{
			writer(e.data);
		}
	}
}

struct Axes3D
{
	common : AxesCommon
}

impl Axes3D
{
	fn new() -> Axes3D
	{
		Axes3D
		{
			common : AxesCommon::new()
		}
	}
}

enum AxesVariant
{
	Axes2DType(Axes2D),
	Axes3DType(Axes3D)
}

impl AxesVariant
{
	fn write_out(&self, writer : &fn(data : &[u8]))
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
			Axes2DType(ref a) => &'l a.common,
			Axes3DType(ref a) => &'l a.common
		}
	}
}

struct Figure<'self>
{
	axes: ~[AxesVariant],
	num_rows: uint,
	num_cols: uint,
	terminal: &'self str,
	output_file: &'self str
}

/// A figure that may contain multiple axes
///
/// # Example
/// ~~~
/// let x = [0, 1, 2];
/// let y = [3, 4, 5];
/// let mut fg = Figure::new();
/// {
///	   let ax = fg.axes2d();
///    ax.lines(x.iter(), y.iter(), [Caption("A line"), Color("black")]);
/// }
/// fg.show();
/// ~~~
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
	/// * wxt - Interactive GUI
	/// * pdfcairo - Saves the figure as a PDF file
	/// * epscairo - Saves the figure as a EPS file
	/// * pngcairo - Saves the figure as a PNG file
	pub fn set_terminal<'l>(&'l mut self, terminal : &'self str, output_file : &'self str) -> &'l mut Figure<'self>
	{
		self.terminal = terminal;
		self.output_file = output_file;
		self
	}
	
	/// Sets the dimensions of the grid that you can use to
	/// place multiple axes on
	/// # Arguments
	/// * rows - Number of rows. Set to 0 to disable the grid
	/// * cols - Number of columns. Set to 0 to disable the grid
	pub fn set_grid<'l>(&'l mut self, rows : uint, cols : uint) -> &'l mut Figure<'self>
	{
		self.num_rows = rows;
		self.num_cols = cols;
		self
	}
	
	
	/// Creates a set of 2D axes
	pub fn axes2d<'l>(&'l mut self) -> &'l mut Axes2D
	{
		self.axes.push(Axes2DType(Axes2D::new()));
		match self.axes[self.axes.len() - 1]
		{
			Axes2DType(ref mut a) => a,
			_ => fail!()
		}
	}
	
	/// Creates a set of 3D axes
	pub fn axes3d<'l>(&'l mut self) -> &'l mut Axes3D
	{
		self.axes.push(Axes3DType(Axes3D::new()));
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
	/// * writer - A function pointer that will be called multiple times with the command text and data
	pub fn echo<'l>(&'l self, writer : &fn(data : &[u8])) -> &'l Figure<'l>
	{
		if self.axes.len() == 0
		{
			return self;
		}
		
		str::byte_slice("set terminal ", writer);
		str::byte_slice(self.terminal, writer);
		str::byte_slice("\n", writer);
		
		if self.output_file.len() > 0
		{
			str::byte_slice("set output \"", writer);
			str::byte_slice(self.output_file, writer);
			str::byte_slice("\"\n", writer);
		}
		
		str::byte_slice("set termoption dashed\n", writer);
		str::byte_slice("set termoption enhanced\n", writer);
		str::byte_slice("set multiplot\n", writer);
		
		let do_layout = self.num_rows > 0 && self.num_cols > 0;
		
		let (w, h) = if do_layout
		{
			(1.0 / (self.num_cols as float), 1.0 / (self.num_rows as float))
		}
		else
		{
			(0.0, 0.0)
		};
		
		for self.axes.each() |e|
		{
			if do_layout
			{
				let c = e.get_common();
				let x = (c.grid_col as float - 1.0) * w;
				let y = (self.num_rows as float - c.grid_row as float) * h;
				
				str::byte_slice("set origin ", writer);
				do to_sci(x) |s| { str::byte_slice(s, writer) };
				str::byte_slice(",", writer);
				do to_sci(y) |s| { str::byte_slice(s, writer) };
				str::byte_slice("\n", writer);
				
				str::byte_slice("set size ", writer);
				do to_sci(w) |s| { str::byte_slice(s, writer) };
				str::byte_slice(",", writer);
				do to_sci(h) |s| { str::byte_slice(s, writer) };
				str::byte_slice("\n", writer);
			}
			e.write_out(writer);
		}
		
		str::byte_slice("unset multiplot\n", writer);
		self
	}
	
	/// Save to a file the the commands that if piped to a gnuplot process would display the figure
	/// # Arguments
	/// * filename - Name of the file
	pub fn echo_to_file<'l>(&'l self, filename : &str) -> &'l Figure<'l>
	{
		if self.axes.len() == 0
		{
			return self;
		}
		
		let file = io::file_writer(&Path(filename), [io::Create]).get();
		do self.echo |v|
		{
			file.write(v);
		};
		self
	}
}
