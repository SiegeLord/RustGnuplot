// This file is released into Public Domain.

extern crate getopts;

use self::getopts::*;
use std::os;
use gnuplot::*;

pub fn run() -> Option<(bool, Box<Fn(/*fg: */&mut Figure, /*filename: */&str) + 'static>, Box<Fn(/*fg: */&mut Figure) + 'static>)>
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

	let no_show = matches.opt_present("n");
	let term = matches.opt_str("t").map(|s| s.to_string());
	
	Some((!no_show,
		(box move |fg: &mut Figure, filename|
		{
			if !no_show
			{
				fg.show();
			}
			fg.echo_to_file(filename);
		}) as Box<Fn(&mut _, &_)>,
		(box move |fg: &mut Figure|
		{
			term.as_ref().map(|t|
			{
				fg.set_terminal(t.as_slice(), "");
			});
		}) as Box<Fn(&mut _)>))
}
