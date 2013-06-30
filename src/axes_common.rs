use options::*;
use writer::*;
use coordinates::*;

struct PlotElement
{
	args : ~[u8],
	data : ~[u8]
}

impl PlotElement
{
	pub fn new() -> PlotElement
	{
		PlotElement
		{
			args : ~[],
			data : ~[],
		}
	}
}

pub enum LabelType
{
	XLabel,
	YLabel,
	Title,
	Label(Coordinate, Coordinate)
}

impl LabelType
{
	fn is_label(&self) -> bool
	{
		match *self
		{
			Label(*) => true,
			_ => false
		}
	}
}

pub enum PlotType
{
	Lines,
	Points,
	LinesPoints,
	XErrorLines,
	YErrorLines,
	FillBetween
}

impl PlotType
{
	fn is_line(&self) -> bool
	{
		match *self
		{
			Lines |
			LinesPoints |
			XErrorLines |
			YErrorLines => true,
			_ => false
		}
	}
	
	fn is_points(&self) -> bool
	{
		match *self
		{
			Points |
			LinesPoints |
			XErrorLines |
			YErrorLines => true,
			_ => false
		}
	}
	
	fn is_fill(&self) -> bool
	{
		match *self
		{
			FillBetween => true,
			_ => false
		}
	}
}

struct AxesCommon
{
	commands: ~[u8],
	elems : ~[PlotElement],
	grid_row : uint,
	grid_col : uint
}

pub fn char_to_symbol(c : char) -> int
{
	match c
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
	}
}

impl AxesCommon
{
	pub fn new() -> AxesCommon
	{
		AxesCommon
		{
			commands: ~[],
			elems: ~[],
			grid_row: 0,
			grid_col: 0,
		}
	}
	
	pub fn write_common_commands(&mut self, elem_idx : uint, num_rows : int, num_cols : int, plot_type : PlotType, options : &[PlotOption])
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
		let type_str = match plot_type
		{
			Lines => "lines",
			Points => "points",
			LinesPoints => "linespoints",
			XErrorLines => "xerrorlines",
			YErrorLines => "yerrorlines",
			FillBetween => "filledcurves",
		};
		args.write_str(type_str);
		
		if plot_type.is_fill()
		{
			let mut found = false;
			for options.iter().advance |o|
			{
				match *o
				{
					FillRegion(d) =>
					{
						found = true;
						args.write_str(match d
						{
							Above => " above",
							Below => " below",
							Between => " closed",
						});
						break;
					},
					_ => ()
				};
			}
			if !found
			{
				args.write_str(" closed");
			}
		}
		
		if plot_type.is_line()
		{
			let mut found = false;
			args.write_str(" lw ");
			for options.iter().advance |o|
			{
				match *o
				{
					LineWidth(w) =>
					{
						args.write_float(w);
						found = true;
						break;
					},
					_ => ()
				};
			}
			if !found
			{
				args.write_float(1.0);
			}
			
			args.write_str(" lt ");
			let mut found = false;
			for options.iter().advance |o|
			{
				match *o
				{
					LineType(d) =>
					{
						args.write_int(d.to_int());
						found = true;
						break;
					},
					_ => ()
				};
			}
			if !found
			{
				args.write_int(1);
			}
		}

		if plot_type.is_points()
		{
			for options.iter().advance |o|
			{
				match *o
				{
					PointSymbol(s) =>
					{
						args.write_str(" pt ");
						args.write_int(char_to_symbol(s));
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					PointSize(z) =>
					{
						args.write_str(" ps ");
						args.write_float(z);
						break;
					},
					_ => ()
				};
			}
		}
		
		for options.iter().advance |o|
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
		
		args.write_str(" t \"");
		for options.iter().advance |o|
		{
			match *o
			{
				Caption(s) =>
				{				
					args.write_str(s);
					break;
				},
				_ => ()
			};
		}
		args.write_str("\"");
		
		if plot_type.is_fill()
		{
			args.write_str(" fill transparent solid ");

			for options.iter().advance |o|
			{
				match *o
				{
					FillAlpha(a) =>
					{
						args.write_float(a);
						break;
					},
					_ => ()
				};
			}
			
			if !plot_type.is_line()
			{
				args.write_str(" noborder");
			}
		}
	}
}
