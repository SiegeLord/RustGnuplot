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
use std::u64;
use std::io;
use std::run::{Process, ProcessOptions};

enum PlotOption<'self>
{
	PointSymbol(char),
	Caption(&'self str),
	LineWidth(float),
	Color(&'self str)
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
	fn write_data<T : DataType>(&mut self, v : T);
	fn write_str(&mut self, s : &str);
}

impl Writable for ~[u8]
{
	fn write_data<T : DataType>(&mut self, v : T)
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

	fn write_str(&mut self, s : &str)
	{
		do str::byte_slice(s) |v| { self.push_all(v) }
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
	cell_row : uint,
	cell_col : uint
}

impl AxesCommon
{
	fn new() -> AxesCommon
	{
		AxesCommon
		{
			commands: ~[],
			elems: ~[],
			cell_row: 0,
			cell_col: 0,
		}
	}
}

pub struct Axes2D
{
	common : AxesCommon
}

impl Axes2D
{
	pub fn set_cell(&mut self, row : uint, col : uint)
	{
		self.common.cell_row = row;
		self.common.cell_col = col;
	}
	
	pub fn set_x_label(&mut self, text : &str)
	{
		let c = &mut self.common.commands;
		
		c.write_str("set xlabel \"");
		c.write_str(text);
		c.write_str("\"\n");
	}
	
	pub fn set_y_label(&mut self, text : &str)
	{
		let c = &mut self.common.commands;
		
		c.write_str("set ylabel \"");
		c.write_str(text);
		c.write_str("\"\n");
	}
	
	pub fn set_title(&mut self, text : &str)
	{
		let c = &mut self.common.commands;
		
		c.write_str("set title \"");
		c.write_str(text);
		c.write_str("\"\n");
	}
	
	pub fn lines<Tx : DataType, Ty : DataType, X : Iterator<Tx>, Y : Iterator<Ty>>(&mut self, x : X, y : Y, options : &[PlotOption])
	{
		self.plot2(Lines, x, y, options);
	}
	
	pub fn points<Tx : DataType, Ty : DataType, X : Iterator<Tx>, Y : Iterator<Ty>>(&mut self, x : X, y : Y, options : &[PlotOption])
	{
		self.plot2(Points, x, y, options);
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
	
	fn plot2<Tx : DataType, Ty : DataType, X : Iterator<Tx>, Y : Iterator<Ty>>(&mut self, style : PlotStyle, mut x : X, mut y : Y, options : &[PlotOption])
	{
		let l = self.common.elems.len();
		self.common.elems.push(PlotElement::new());
		
		let args = &mut self.common.elems[l].args;
		let data = &mut self.common.elems[l].data;
		
		let mut length : u64 = 0;
		
		loop
		{
			let x_val = match x.next()
			{
				Some(a) => a,
				None => break
			};
			
			let y_val = match y.next()
			{
				Some(a) => a,
				None => break
			};
			
			data.write_data(x_val);
			data.write_data(y_val);
			
			length += 1;
		}
		
		args.write_str(" \"-\" binary endian=little record=");
		args.write_str(u64::to_str(length));
		args.write_str(" format=\"%float64\" using 1:2 with ");
		
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
							args.write_str(w.to_str());
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
							let typ : i8 = match t
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
							args.write_str(typ.to_str());
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

pub struct Axes3D
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

pub struct Figure
{
	priv axes: ~[AxesVariant],
	priv gnuplot: Option<Process>,
	priv num_rows: uint,
	priv num_cols: uint
}

impl Figure
{
	pub fn new() -> Figure
	{
		Figure
		{
			axes: ~[],
			gnuplot: None,
			num_rows: 0,
			num_cols: 0
		}
	}
	
	pub fn layout(&mut self, rows : uint, cols : uint)
	{
		self.num_rows = rows;
		self.num_cols = cols;
	}
	
	pub fn axes2d<'l>(&'l mut self) -> &'l mut Axes2D
	{
		self.axes.push(Axes2DType(Axes2D::new()));
		match self.axes[self.axes.len() - 1]
		{
			Axes2DType(ref mut a) => a,
			_ => fail!()
		}
	}
	
	pub fn axes3d<'l>(&'l mut self) -> &'l mut Axes3D
	{
		self.axes.push(Axes3DType(Axes3D::new()));
		match self.axes[self.axes.len() - 1]
		{
			Axes3DType(ref mut a) => a,
			_ => fail!()
		}
	}
	
	pub fn show(&mut self)
	{
		if(self.gnuplot.is_none())
		{
			self.gnuplot = Some(Process::new("gnuplot", [~"-p"], ProcessOptions::new()));
		}
		let input = self.gnuplot.get_mut_ref().input();
		
		do self.echo |v|
		{
			input.write(v);
		}
	}
	
	pub fn echo(&self, writer : &fn(data : &[u8]))
	{
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
				let x = (c.cell_col as float - 1.0) * w;
				let y = (self.num_rows as float - c.cell_row as float) * h;
				
				str::byte_slice("set origin ", writer);
				str::byte_slice(x.to_str(), writer);
				str::byte_slice(",", writer);
				str::byte_slice(y.to_str(), writer);
				str::byte_slice("\n", writer);
				
				str::byte_slice("set size ", writer);
				str::byte_slice(w.to_str(), writer);
				str::byte_slice(",", writer);
				str::byte_slice(h.to_str(), writer);
				str::byte_slice("\n", writer);
			}
			e.write_out(writer);
		}
		
		str::byte_slice("unset multiplot\n", writer);
	}
	
	pub fn echo_to_file(&self, filename : &str)
	{
		let file = io::file_writer(&Path(filename), [io::Create]).get();
		do self.echo |v|
		{
			file.write(v);
		}
	}
}
