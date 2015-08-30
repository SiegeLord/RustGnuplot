// This file is released into Public Domain.

extern crate getopts;

use self::getopts::*;
use std::env;
use gnuplot::*;

#[derive(Copy, Clone)]
pub struct BetterIterator<'l, T: 'l>
{
	idx: usize,
	slice: &'l [T]
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
		BetterIterator{ idx: 0, slice: self }
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
		let args: Vec<_> = env::args().collect();

		let mut opts = Options::new();

		opts.optflag("n", "no-show", "do not run the gnuplot process.");
		opts.optflag("h", "help", "show this help and exit.");
		opts.optopt("t", "terminal", "specify what terminal to use for gnuplot.", "TERM");

		let matches = match opts.parse(&args[1..])
		{
			Ok(m) => m,
			Err(f) => panic!("{}", f)
		};
		if matches.opt_present("h")
		{
			println!("{}", opts.usage("A RustGnuplot example."));
			return None;
		}

		Some(Common
		{
			no_show: matches.opt_present("n"),
			term: matches.opt_str("t").map(|s| s.to_string())
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
		self.term.as_ref().map(|t|
		{
			fg.set_terminal(&t[..], "");
		});
	}
}
