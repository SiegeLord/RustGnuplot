use std::error;
use std::fmt;
use std::io;

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
			"Couldn't spawn gnuplot. Make sure it is installed and available in PATH.\nCause: {}",
			self.inner
		)
	}
}

impl fmt::Debug for GnuplotInitError
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self)
	}
}

impl error::Error for GnuplotInitError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		Some(&*self.inner)
	}
}
