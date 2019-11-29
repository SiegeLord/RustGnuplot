use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct GnuplotInitError
{
	inner: Box<dyn error::Error + 'static>,
}

impl From<io::Error> for GnuplotInitError
{
	fn from(error: io::Error) -> Self
	{
		GnuplotInitError {
			inner: Box::new(error),
		}
	}
}

impl fmt::Display for GnuplotInitError
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(
			f,
			"Couldn't spawn gnuplot. Make sure it is installed and available in PATH.\n{}",
			self.inner
		)
	}
}

impl error::Error for GnuplotInitError {}
