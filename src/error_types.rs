use std::error;
use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub struct GnuplotInitError;

impl fmt::Display for GnuplotInitError
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(
			f,
			"Couldn't spawn gnuplot. Make sure it is installed and available in PATH."
		)
	}
}

impl error::Error for GnuplotInitError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		None
	}
}

impl From<io::Error> for GnuplotInitError
{
    fn from(_error: io::Error) -> Self
	{
        GnuplotInitError
    }
}
