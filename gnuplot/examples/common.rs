// This file is released into Public Domain.
#![allow(dead_code)]
use argparse_rs::*;
use gnuplot::*;
use std::env;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct BetterIterator<'l, T: 'l>
{
	idx: usize,
	slice: &'l [T],
}

impl<'l, T: 'l> Iterator for BetterIterator<'l, T>
{
	type Item = &'l T;
	fn next(&mut self) -> Option<&'l T>
	{
		let ret = self.slice.get(self.idx);
		self.idx += 1;
		ret
	}
}

pub trait BetterIteratorExt<'l, T>
{
	fn iter2(self) -> BetterIterator<'l, T>;
}

impl<'l, T: 'l> BetterIteratorExt<'l, T> for &'l [T]
{
	fn iter2(self) -> BetterIterator<'l, T>
	{
		BetterIterator {
			idx: 0,
			slice: self,
		}
	}
}

pub struct Common
{
	pub no_show: bool,
	pub save_png: bool,
	pub term: Option<String>,
	pub extension: String,
	pub output_dir: String,
	pub echo: bool,
}

impl Common
{
	pub fn new() -> Option<Common>
	{
		let arg_vec: Vec<_> = env::args().collect();

		let mut args = ArgParser::new(arg_vec[0].clone());

		args.add_opt(
			"no-show",
			Some("false"),
			'n',
			false,
			"do not run the gnuplot process.",
			ArgType::Flag,
		);
		args.add_opt(
			"terminal",
			None,
			't',
			false,
			"specify what terminal to use for gnuplot.",
			ArgType::Option,
		);
		args.add_opt(
			"output-dir",
			None,
			'o',
			false,
			"output directory.",
			ArgType::Option,
		);
		args.add_opt(
			"extension",
			Some("out"),
			'e',
			false,
			"specify what extension the output file should have. Default: 'out'",
			ArgType::Option,
		);
		args.add_opt(
			"save-png",
			Some("false"),
			's',
			false,
			"render the plots to images.",
			ArgType::Flag,
		);
		args.add_opt(
			"echo",
			Some("false"),
			'g',
			false,
			"echo gnuplot commands.",
			ArgType::Flag,
		);

		let res = args.parse(arg_vec.iter()).unwrap();

		if res.get("help").unwrap_or(false)
		{
			args.help();
			return None;
		}

		Some(Common {
			output_dir: res.get("output-dir").unwrap_or("".into()),
			no_show: res.get("no-show").unwrap(),
			save_png: res.get("save-png").unwrap(),
			echo: res.get("echo").unwrap_or(false),
			term: res.get::<String>("terminal").map(|s| s.to_string()),
			extension: res.get::<String>("extension").unwrap(),
		})
	}

	pub fn show(&self, fg: &mut Figure, filename: &str)
	{
		let out_path = Path::new(&self.output_dir).join(filename);
		self.term.as_ref().map(|t| {
			fg.set_terminal(
				&t,
				out_path.with_extension(&self.extension).to_str().unwrap(),
			);
		});
		if !self.no_show
		{
			fg.show().unwrap();
		}
		if self.save_png
		{
			fg.save_to_png(out_path.with_extension("png").to_str().unwrap(), 800, 600)
				.unwrap();
		}
		if self.echo
		{
			fg.echo_to_file(out_path.with_extension("gnuplot").to_str().unwrap());
		}
	}
}
