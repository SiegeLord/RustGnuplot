use options::*;
use writer::*;

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
			for options.each() |o|
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
							Closed => " closed",
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
			for options.each() |o|
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
			for options.each() |o|
			{
				match *o
				{
					LineType(d) =>
					{
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
		
		args.write_str(" t \"");
		for options.each() |o|
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

			for options.each() |o|
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
