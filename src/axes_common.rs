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
	Label(Coordinate, Coordinate),
	AxesTicks,
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

pub fn write_out_label_options<T : PlotWriter>(label_type : LabelType, options : &[LabelOption], writer : &mut T)
{
	let w = writer;

	match label_type
	{
		Label(x, y) => 
		{
			w.write_str(" at ");
			x.write(w);
			w.write_str(",");
			y.write(w);
			w.write_str(" front");
		}
		_ => ()
	}
	
	for options.iter().advance |o|
	{
		match *o
		{
			Offset(x, y) =>
			{
				w.write_str(" offset character ");
				w.write_float(x);	
				w.write_str(",");
				w.write_float(y);
				break;
			},
			_ => ()
		};
	}
	
	for options.iter().advance |o|
	{
		match *o
		{
			TextColor(s) =>
			{
				w.write_str(" tc rgb \"");
				w.write_str(s);
				w.write_str("\"");
				break;
			},
			_ => ()
		};
	}
	
	for options.iter().advance |o|
	{
		match *o
		{
			Font(f, s) =>
			{
				w.write_str(" font \"");
				w.write_str(f);
				w.write_str(",");
				w.write_str(s.to_str());
				w.write_str("\"");
				break;
			},
			_ => ()
		};
	}
	
	for options.iter().advance |o|
	{
		match *o
		{
			Rotate(a) =>
			{
				w.write_str(" rotate by ");
				w.write_float(a);
				break;
			},
			_ => ()
		};
	}
	
	if label_type.is_label()
	{
		let mut have_point = false;
		for options.iter().advance |o|
		{
			match *o
			{
				MarkerSymbol(s) =>
				{
					w.write_str(" point pt ");
					w.write_int(char_to_symbol(s));
					have_point = true;
					break;
				},
				_ => ()
			};
		}
		
		if have_point
		{
			for options.iter().advance |o|
			{
				match *o
				{
					MarkerColor(s) =>
					{
						w.write_str(" lc rgb \"");
						w.write_str(s);
						w.write_str("\"");
						break;
					},
					_ => ()
				};
			}
			
			for options.iter().advance |o|
			{
				match *o
				{
					MarkerSize(z) =>
					{
						w.write_str(" ps ");
						w.write_float(z);
						w.write_str("");
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
				Align(a) =>
				{
					w.write_str(match(a)
					{
						AlignLeft => " left",
						AlignRight => " right",
						AlignCenter => " center",
					});
					break;
				},
				_ => ()
			};
		}
	}
}

pub enum TickType
{
	XTics,
	YTics,
}

impl TickType
{
	pub fn to_str(&self) -> &str
	{
		match *self
		{
			XTics => "xtics",
			YTics => "ytics",
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
	FillBetween,
	Boxes,
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
			Boxes |
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
			Boxes |
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
			Boxes => "boxes",
		};
		args.write_str(type_str);
		
		if plot_type.is_fill()
		{
			match plot_type
			{
				FillBetween =>
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
				},
				_ => ()
			}
			
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
			
			if plot_type.is_line()
			{
				args.write_str(" border");
				for options.iter().advance |o|
				{
					match *o
					{
						BorderColor(s) =>
						{
							args.write_str(" rgb \"");
							args.write_str(s);
							args.write_str("\"");
							break;
						},
						_ => ()
					};
				}
			}
			else
			{
				args.write_str(" noborder");
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
					LineStyle(d) =>
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
	}
}
