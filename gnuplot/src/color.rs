pub use self::ColorType::*;
use crate::util::OneWayOwned;
use std::fmt::{Debug, Display};

pub type ColorIndex = u8;
pub type ColorComponent = u8;
pub type ColorInt = u32;
pub type RGBInts = (ColorComponent, ColorComponent, ColorComponent);
pub type ARGBInts = (
	ColorComponent,
	ColorComponent,
	ColorComponent,
	ColorComponent,
);

/// Option type (for plots, borders, and text) that allows the various different gnuplot
/// color formats. The gnuplot [colorspec reference](http://gnuplot.info/docs_6.0/loc3640.html)
/// also explains these.
///
/// There are many equivalent ways of specifying colors, and this allows the user to chose the most convenient.
/// For example, all the following will produce the same blue color:
/// `RGBColor("blue".into())`, `RGBColor("0x0000ff".into())`, `RGBColor("#0000ff".into())`, `RGBColor("0x000000ff".into())`,
/// `RGBColor("#000000ff".into())`, `RGBIntegerColor(0, 0, 255)`, `ARGBColor(0, 0, 0, 255)`,
///
/// See example usages of these colors in `color.rs` and `variable_color.rs` in the
/// [Examples folder](https://github.com/SiegeLord/RustGnuplot/tree/master/gnuplot/examples) on Github
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColorType<T = String>
{
	/// string (`&str` or `String`, but usually created with `&str`) in one of the following gnuplot-supported formats
	/// - colorname   --- e.g. "blue" [See the gnuplot
	///     [list of colornames](http://gnuplot.info/docs_6.0/loc11229.html)]
	/// - 0xRRGGBB    --- string containing hexadecimal constant
	/// - 0xAARRGGBB  --- string containing hexadecimal constant
	/// - #RRGGBB     --- string containing hexadecimal in x11 format
	/// - #AARRGGBB   --- string containing hexadecimal in x11 format
	///
	/// "#AARRGGBB" represents an RGB color with an alpha channel (transparency) value in the high bits.
	/// An alpha value of 0 represents a fully opaque color; i.e., "#00RRGGBB" is the same as "#RRGGBB".
	/// An alpha value of 255 (FF) represents full transparency.
	RGBString(T),
	/// tuple of u8 representing red, green and blue values as 0-255
	RGBInteger(ColorComponent, ColorComponent, ColorComponent),
	/// tuple of u8 representing alpha, red, green and blue values as 0-255.
	/// As with `RGBColor`, an alpha value of 0 represents a fully opaque color;
	/// an alpha value of 255 (FF) represents full transparency.
	ARGBInteger(
		ColorComponent,
		ColorComponent,
		ColorComponent,
		ColorComponent,
	),
	/// Vector of tuples of `u8` (as per `RGBColor`), but instead of a single color for the whole
	/// plot, the vector should contain a separte color for each data point.
	VariableRGBInteger(Vec<RGBInts>),
	/// Vector of tuples of `u8` (as per `ARGBColor`), but as with `VariableRGBColor`, a separate
	/// color value is given for each data point.
	VariableARGBInteger(Vec<ARGBInts>),
	/// Sets the color of the plot element to a value picked from the current palette (see
	/// [set_palette()](crate::AxesCommon::set_palette())). The value supplied to this color type
	/// selects the color within the color range of the palette: i.e. it if the color bar range had been
	/// set with `ax.set_cb_range(Fix(min), Fix(max))`, the value would be expected to be between
	/// `min` and `max`.
	///
	/// Example of usage is give in the `color` example.
	///
	/// Compare with [PaletteFracColor]
	PaletteFracColor(f64),
	/// Sets the color of the plot element to a value picked from the current palette (see
	/// [set_palette()](crate::AxesCommon::set_palette()) . The value supplied to this color type
	/// selects the color as a fraction of the current color range i.e. it is expected to be
	/// between `0` and `1`.
	///
	/// Example of usage is give in the `color` example.
	///
	/// Comparing with [PaletteCBColor]: given the following code
	/// ```
	/// use gnuplot::{PaletteCBColor, PaletteFracColor, Fix, Figure, AxesCommon, Color};
	///# let min = -5.0; // or any value
	///# let max = 12.0; // or any value
	///
	///# let frac = 0.5; // or any value 0.0 <= frac <= 1.0
	///# let x = [1,2,3];
	///# let y = [4,5,6];
	/// assert!(frac >= 0.0);
	/// assert!(frac <= 1.0);
	///
	/// let mut fg = Figure::new();
	/// let ax = fg.axes2d();
	/// ax.set_cb_range(Fix(min), Fix(max));
	/// let col1 = Color(PaletteFracColor(frac));
	/// let cb_range = max - min;
	/// let col2 = Color(PaletteCBColor(min + (frac * cb_range)));
	/// ax.lines(x, y, &[col1]);
	/// ax.lines(x, y, &[col2]);
	/// ```
	/// the two lines should give the same color for any values of `max` and `min`, and `0 <= frac <= 1`.
	PaletteCBColor(f64),
	/// Vector of `f64` values which act as indexes into the current palette to set the color of
	/// each data point. These variable values work in the same was as the single fixed value supplied
	/// to a [PaletteCBColor]
	VariablePaletteColor(Vec<f64>),
	/// Similar to `VariablePaletteColor` in that it takes a `Vec<f64>` to set the indexes into the
	/// color map for each data point, but in addition to the color data it takes a string hold the name
	/// of the color map to use. This should have been previously created in the workspace using the
	/// [create_colormap()](crate::AxesCommon::create_colormap) function.
	SavedColorMap(T, Vec<f64>),
	/// Set the color of all elements of the plot to the `n`th color in the current gnuplot color cycle.
	Index(ColorIndex),
	/// A color type that sets the color per element using a index `n` which represents the `n`th
	/// color in the current gnuplot color scheme. In gnuplot this is the last element in the plot command,
	/// in Rust gnuplot, the color type takes a vector of u8, where each index is treated the same as the
	/// fixed `IndexColor`.
	/// This is useful for setting bars/boxes etc to be
	/// the same color from multiple plot commands. The `variable_color` example has examples of this usage.
	VariableIndex(Vec<ColorIndex>),
	/// Set the color of the plot to the current background color.
	Background,
	/// Fixed black color
	Black,
}

impl<T: Display + Debug> ColorType<T>
{
	/// Returns the gnuplot string that will produce the requested color
	pub fn command(&self) -> String
	{
		let str = match self
		{
			RGBString(s) => &format!(r#"rgb "{}""#, s),
			RGBInteger(r, g, b) => &format!(r#"rgb {}"#, from_argb(0, *r, *g, *b)),
			ARGBInteger(a, r, g, b) => &format!(r#"rgb {}"#, from_argb(*a, *r, *g, *b)),
			VariableRGBInteger(_) => "rgb variable",
			VariableARGBInteger(_) => "rgb variable",
			PaletteFracColor(v) => &format!("palette frac {v}"),
			PaletteCBColor(v) => &format!("palette cb {v}"),
			VariablePaletteColor(_) => "palette z",
			SavedColorMap(s, _) => &format!("palette {s}"),
			VariableIndex(_) => "variable",
			Background => "bgnd",
			Index(n) => &format!("{}", n),
			Black => "black",
		};
		String::from(str)
	}

	pub fn data(&self) -> Vec<f64>
	{
		match self
		{
			VariableRGBInteger(items) => items
				.iter()
				.map(|(r, g, b)| from_argb(0, *r, *g, *b) as f64)
				.collect(),
			VariableARGBInteger(items) => items
				.iter()
				.map(|(a, r, g, b)| from_argb(*a, *r, *g, *b) as f64)
				.collect(),
			VariablePaletteColor(items) => items.clone(),
			SavedColorMap(_, items) => items.clone(),
			VariableIndex(items) => items.iter().map(|v| *v as f64).collect(),
			c => panic!("data() called on non-variable color type: {:?}", *c),
		}
	}

	pub fn is_variable(&self) -> bool
	{
		matches!(
			self,
			VariableRGBInteger(_)
				| VariableARGBInteger(_)
				| VariableIndex(_)
				| VariablePaletteColor(_)
				| SavedColorMap(_, _)
		)
	}

	pub fn has_alpha(&self) -> bool
	{
		match self
		{
			RGBString(s) =>
			{
				let s = s.to_string();
				s.starts_with("0x") && s.chars().count() == 10
					|| s.starts_with("#") && s.chars().count() == 9
			}
			ARGBInteger(_, _, _, _) | VariableARGBInteger(_) => true,
			_ => false,
		}
	}
}

fn from_argb(a: ColorComponent, r: ColorComponent, g: ColorComponent, b: ColorComponent)
	-> ColorInt
{
	((a as ColorInt) << 24) + ((r as ColorInt) << 16) + ((g as ColorInt) << 8) + (b as ColorInt)
}

fn float_color_to_int(v: f64) -> Result<u8, String>
{
	if !(0.0..=1.0).contains(&v)
	{
		Err(format!(
			"Float value must be greater than zero and less than one. Actual value: {}",
			v
		))
	}
	else
	{
		Ok(((v * 255.0).round()) as u8)
	}
}

/// Converts a set of `f64` red, green and blue values in the range `0 <= x <= 1` to a 3-tuple of `u8` suitable for use as
/// an [RGBInteger]
///
/// Returns an error String if any of the arguments are not in the range `0 <= x <= 1`
///
/// Ses also [floats_to_argb]
///
/// # Arguments
/// * r - red. 0: no red, 1: fully red
/// * g - green. 0: no green, 1: fully green
/// * b - blue. 0: no blue, 1: fully blue
fn floats_to_rgb(r: f64, g: f64, b: f64) -> Result<RGBInts, String>
{
	Ok((
		float_color_to_int(r)?,
		float_color_to_int(g)?,
		float_color_to_int(b)?,
	))
}

/// Converts a set of `f64` red, green and blue values in the range `0 <= x <= 1` to a 3-tuple of `u8` suitable for use as
/// an [ARGBInteger]
///
/// Returns an error String if any of the arguments are not in the range `0 <= x <= 1`
///
/// Ses also [floats_to_rgb]
///
/// # Arguments
/// * a - alpha (transparency) value. 0: completely opaque, 1: completely transparent.
/// * r - red. 0: no red, 1: fully red
/// * g - green. 0: no green, 1: fully green
/// * b - blue. 0: no blue, 1: fully blue
fn floats_to_argb(a: f64, r: f64, g: f64, b: f64) -> Result<ARGBInts, String>
{
	Ok((
		float_color_to_int(a)?,
		float_color_to_int(r)?,
		float_color_to_int(g)?,
		float_color_to_int(b)?,
	))
}

impl<'l> From<&'l str> for ColorType<String>
{
	/// Converts `&str` into [RGBString]
	fn from(value: &'l str) -> Self
	{
		ColorType::RGBString(String::from(value))
	}
}

impl<'l> From<String> for ColorType<String>
{
	/// Converts `String` into [RGBString]
	fn from(value: String) -> Self
	{
		ColorType::RGBString(value)
	}
}

impl<'l> From<&'l str> for ColorType<&'l str>
{
	/// Converts `&str` into [RGBString]
	fn from(value: &'l str) -> Self
	{
		ColorType::RGBString(value)
	}
}

impl<T> From<ARGBInts> for ColorType<T>
{
	/// Converts `(u8, u8, u8, u8)` into [ARGBInteger]
	fn from(value: ARGBInts) -> Self
	{
		ColorType::ARGBInteger(value.0, value.1, value.2, value.3)
	}
}

impl<T> From<RGBInts> for ColorType<T>
{
	/// Converts `(u8, u8, u8)` into [RGBInteger]
	fn from(value: RGBInts) -> Self
	{
		ColorType::RGBInteger(value.0, value.1, value.2)
	}
}

impl<T> TryFrom<(f64, f64, f64)> for ColorType<T>
{
	type Error = String;
	/// Converts `(f64, f64, f64)` into [RGBInteger].
	/// Returns an error unless all values are in the range `0 <= v <= 1`.
	fn try_from(value: (f64, f64, f64)) -> Result<Self, Self::Error>
	{
		let ints = floats_to_rgb(value.0, value.1, value.2)?;
		Ok(ColorType::RGBInteger(ints.0, ints.1, ints.2))
	}
}

impl<T> TryFrom<(f64, f64, f64, f64)> for ColorType<T>
{
	type Error = String;
	/// Converts `(f64, f64, f64, f64)` into [ARGBInteger].
	/// Returns an error unless all values are in the range `0 <= v <= 1`.
	fn try_from(value: (f64, f64, f64, f64)) -> Result<Self, Self::Error>
	{
		let ints = floats_to_argb(value.0, value.1, value.2, value.3)?;
		Ok(ColorType::ARGBInteger(ints.0, ints.1, ints.2, ints.3))
	}
}

impl<T> From<Vec<RGBInts>> for ColorType<T>
{
	/// Converts `Vec<(u8, u8, u8)>` into [VariableRGBInteger]
	fn from(value: Vec<RGBInts>) -> Self
	{
		ColorType::VariableRGBInteger(value)
	}
}

impl<T> From<Vec<ARGBInts>> for ColorType<T>
{
	/// Converts `Vec<(u8, u8, u8, u8)>` into [VariableARGBInteger]
	fn from(value: Vec<ARGBInts>) -> Self
	{
		ColorType::VariableARGBInteger(value)
	}
}

impl<T> From<ColorIndex> for ColorType<T>
{
	/// Converts `u8` into [Index]
	fn from(value: ColorIndex) -> Self
	{
		ColorType::Index(value)
	}
}

impl<T> From<Vec<ColorIndex>> for ColorType<T>
{
	/// Converts `Vec<u8>` into [VariableIndex]
	fn from(value: Vec<ColorIndex>) -> Self
	{
		ColorType::VariableIndex(value)
	}
}

impl<T: Display> OneWayOwned for ColorType<T>
{
	type Output = ColorType<String>;

	fn to_one_way_owned(&self) -> ColorType<String>
	{
		match self
		{
			RGBString(s) => RGBString(s.to_string()),
			RGBInteger(r, g, b) => RGBInteger(*r, *g, *b),
			VariableRGBInteger(d) => VariableRGBInteger(d.clone()),
			PaletteFracColor(v) => PaletteFracColor(*v),
			PaletteCBColor(v) => PaletteCBColor(*v),
			VariablePaletteColor(d) => VariablePaletteColor(d.clone()),
			SavedColorMap(s, d) => SavedColorMap(s.to_string(), d.clone()),
			VariableIndex(d) => VariableIndex(d.clone()),
			Background => Background,
			Index(n) => Index(*n),
			Black => Black,
			ARGBInteger(a, r, g, b) => ARGBInteger(*a, *r, *g, *b),
			VariableARGBInteger(d) => VariableARGBInteger(d.clone()),
		}
	}
}

impl ColorType<String>
{
	pub fn to_ref(&self) -> ColorType<&str>
	{
		match self
		{
			RGBString(s) => RGBString(s),
			RGBInteger(r, g, b) => RGBInteger(*r, *g, *b),
			VariableRGBInteger(d) => VariableRGBInteger(d.to_vec()),
			VariableARGBInteger(d) => VariableARGBInteger(d.to_vec()),
			PaletteFracColor(v) => PaletteFracColor(*v),
			PaletteCBColor(v) => PaletteCBColor(*v),
			VariablePaletteColor(d) => VariablePaletteColor(d.to_vec()),
			SavedColorMap(s, d) => SavedColorMap(s, d.to_vec()),
			VariableIndex(d) => VariableIndex(d.to_vec()),
			Background => Background,
			Index(n) => Index(*n),
			Black => Black,
			ARGBInteger(a, r, g, b) => ARGBInteger(*a, *r, *g, *b),
		}
	}
}
