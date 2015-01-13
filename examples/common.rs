// This file is released into Public Domain.

extern crate getopts;

use self::getopts::*;
use std::os;
use gnuplot::*;

pub struct Common
{
	pub no_show: bool,
	pub term: Option<String>,
}

impl Common
{
	pub fn new() -> Option<Common>
	{
		let args = os::args();

		let opts =
		&[
			optflag("n", "no-show", "do not run the gnuplot process."),
			optflag("h", "help", "show this help and exit."),
			optopt("t", "terminal", "specify what terminal to use for gnuplot.", "TERM")
		];

		let matches = match getopts(args.tail(), opts)
		{
			Ok(m) => m,
			Err(f) => panic!("{}", f)
		};
		if matches.opt_present("h")
		{
			println!("{}", usage("A RustGnuplot example.", opts));
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
			fg.set_terminal(t.as_slice(), "");
		});
	}
}
