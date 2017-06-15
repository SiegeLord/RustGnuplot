// This file is released into Public Domain.

extern crate argparse_rs;

use self::argparse_rs::*;
use gnuplot::*;
use std::env;

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
		BetterIterator { idx: 0, slice: self }
	}
}

pub struct Common
{
	pub no_show: bool,
	pub term: Option<String>,
}

impl Common
{
	pub fn new() -> Option<Common>
	{
		let arg_vec: Vec<_> = env::args().collect();

		let mut args = ArgParser::new(arg_vec[0].clone());

		args.add_opt("no-show", Some("false"), 'n', false, "do not run the gnuplot process.", ArgType::Flag);
		args.add_opt("terminal", None, 't', false, "specify what terminal to use for gnuplot.", ArgType::Option);

		let res = args.parse(arg_vec.iter()).unwrap();

		if res.get("help").unwrap_or(false)
		{
			args.help();
			return None;
		}

		Some(Common {
			no_show: res.get("no-show").unwrap(),
			term: res.get::<String>("terminal").map(|s| s.to_string()),
		})
	}

	pub fn show(&self, fg: &mut Figure, filename: &str)
	{
		if !self.no_show
		{
			fg.show();
		}
		fg.echo_to_file(filename);
	}

	pub fn set_term(&self, fg: &mut Figure)
	{
		self.term.as_ref().map(|t| { fg.set_terminal(&t[..], ""); });
	}
}
