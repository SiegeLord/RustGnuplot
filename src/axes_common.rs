// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use std::io::mem::MemWriter;
use std::io::Writer;

use options::*;
use writer::*;
use internal::coordinates::*;

pub struct PlotElement
{
	args: MemWriter,
	data: MemWriter
}

impl PlotElement
{
	pub fn new() -> PlotElement
	{
		PlotElement
		{
			args: MemWriter::new(),
			data: MemWriter::new(),
		}
	}
}

pub enum LabelType
{
	XLabel,
	YLabel,
	TitleLabel,
	Label(Coordinate, Coordinate),
	AxesTicks,
}

impl LabelType
{
	fn is_label(&self) -> bool
	{
		match *self
		{
			Label(..) => true,
			_ => false
		}
	}
}

pub fn write_out_label_options<T: PlotWriter + Writer>(label_type: LabelType, options: &[LabelOption], writer: &mut T)
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
	
	first_opt!(options,
		TextOffset(x, y) =>
		{
			w.write_str(" offset character ");
			w.write_float(x);	
			w.write_str(",");
			w.write_float(y);
		}
	)
	
	first_opt!(options,
		TextColor(s) =>
		{
			w.write_str(" tc rgb \"");
			w.write_str(s);
			w.write_str("\"");
		}
	)
	
	first_opt!(options,
		Font(f, s) =>
		{
			w.write_str(" font \"");
			w.write_str(f);
			w.write_str(",");
			w.write_str(s.to_str());
			w.write_str("\"");
		}
	)
	
	first_opt!(options,
		Rotate(a) =>
		{
			w.write_str(" rotate by ");
			w.write_float(a);
		}
	)
	
	if label_type.is_label()
	{
		let mut have_point = false;
		first_opt!(options,
			MarkerSymbol(s) =>
			{
				w.write_str(" point pt ");
				w.write_i32(char_to_symbol(s));
				have_point = true;
			}
		)
		
		if have_point
		{
			first_opt!(options,
				MarkerColor(s) =>
				{
					w.write_str(" lc rgb \"");
					w.write_str(s);
					w.write_str("\"");
				}
			)
			
			first_opt!(options,
				MarkerSize(z) =>
				{
					w.write_str(" ps ");
					w.write_float(z);
					w.write_str("");
				}
			)
		}
		
		first_opt!(options,
			TextAlign(a) =>
			{
				w.write_str(match(a)
				{
					AlignLeft => " left",
					AlignRight => " right",
					_ => " center",
				});
			}
		)
	}
}

pub enum TickAxis
{
	XTicks,
	YTicks,
}

impl TickAxis
{
	pub fn to_str(&self) -> &str
	{
		match *self
		{
			XTicks => "xtics",
			YTicks => "ytics",
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

pub struct AxesCommon
{
	commands: MemWriter,
	elems: ~[PlotElement],
	grid_row: u32,
	grid_col: u32
}

pub fn char_to_symbol(c: char) -> i32
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
		a => fail!("Invalid symbol {}", a)
	}
}

impl AxesCommon
{
	pub fn new() -> AxesCommon
	{
		AxesCommon
		{
			commands: MemWriter::new(),
			elems: ~[],
			grid_row: 0,
			grid_col: 0,
		}
	}
	
	pub fn write_line_options(c: &mut MemWriter, options: &[PlotOption])
	{
		let mut found = false;
		c.write_str(" lw ");
		first_opt!(options,
			LineWidth(w) =>
			{
				c.write_float(w);
				found = true;
			}
		)
		if !found
		{
			c.write_float(1.0);
		}
		
		c.write_str(" lt ");
		let mut found = false;
		first_opt!(options,
			LineStyle(d) =>
			{
				c.write_i32(d.to_int());
				found = true;
			}
		)
		if !found
		{
			c.write_i32(1);
		}
	}

	pub fn write_color_options<'l>(c: &mut MemWriter, options: &[PlotOption<'l>], default: Option<&'l str>)
	{
		let mut col = default;
		first_opt!(options,
			Color(s) =>
			{
				col = Some(s)
			}
		)
		match col
		{
			Some(s) => 
			{
				c.write_str(" lc rgb \"");
				c.write_str(s);
				c.write_str("\"");
			},
			None => ()
		}
	}

	pub fn write_common_commands(&mut self, elem_idx: uint, num_rows: i32, num_cols: i32, plot_type: PlotType, options: &[PlotOption])
	{
		let args = &mut self.elems[elem_idx].args;
		args.write_str(" \"-\" binary endian=little record=");
		args.write_i32(num_rows);
		args.write_str(" format=\"%float64\" using ");
		
		let mut col_idx: i32 = 1;
		while(col_idx < num_cols + 1)
		{
			args.write_i32(col_idx);
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
					first_opt!(options,
						FillRegion(d) =>
						{
							found = true;
							args.write_str(match d
							{
								Above => " above",
								Below => " below",
								Between => " closed",
							});
						}
					)
					if !found
					{
						args.write_str(" closed");
					}
				},
				_ => ()
			}
			
			args.write_str(" fill transparent solid ");

			first_opt!(options,
				FillAlpha(a) =>
				{
					args.write_float(a);
				}
			)
			
			if plot_type.is_line()
			{
				args.write_str(" border");
				first_opt!(options,
					BorderColor(s) =>
					{
						args.write_str(" rgb \"");
						args.write_str(s);
						args.write_str("\"");
					}
				)
			}
			else
			{
				args.write_str(" noborder");
			}
		}
		
		if plot_type.is_line()
		{
			AxesCommon::write_line_options(args, options);
		}

		if plot_type.is_points()
		{
			first_opt!(options,
				PointSymbol(s) =>
				{
					args.write_str(" pt ");
					args.write_i32(char_to_symbol(s));
				}
			)
			
			first_opt!(options,
				PointSize(z) =>
				{
					args.write_str(" ps ");
					args.write_float(z);
				}
			)
		}
		
		AxesCommon::write_color_options(args, options, None);
		
		args.write_str(" t \"");
		first_opt!(options,
			Caption(s) =>
			{
				args.write_str(s);
			}
		)
		args.write_str("\"");
	}
}
