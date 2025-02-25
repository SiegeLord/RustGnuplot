pub use self::ColorType::*;
use crate::util::OneWayOwned;
use std::fmt::Display;

pub trait IntoColor<T>: Into<ColorType<T>> + Clone {}
impl<TC, T: ?Sized + Into<ColorType<TC>> + Clone> IntoColor<TC> for T {}

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
/// `RGBColor("blue")`, `RGBColor("0x0000ff")`, `RGBColor("#0000ff")`, `RGBColor("0x000000ff")`,
/// `RGBColor("#000000ff")`, `RGBIntegerColor(0, 0, 255)`, `ARGBColor(0, 0, 0, 255)`,
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
	/// TODO
	PaletteFracColor(f32),
	/// TODO
	PaletteCBColor(f32),
	/// Vector of `f64` values which act as indexes into the current palette to set the color of
	/// each data point
	VariablePaletteColor(Vec<f64>),
	/// Similar to `VariablePaletteColor` in that it takes a `Vec<f64>` to set the indexes into the
	/// color map for each data point, but in addition to the color data it takes a string hold the name
	/// of the color map to use. This should have been previously created in the workspace using the
	/// (create_colormap())[crate::AxesCommon::create_colormap()] function.
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

impl<T: Display> ColorType<T>
{
	/// Returns the gnuplot string that will produce the requested color
	pub fn command(&self) -> String
	{
		match self
		{
			RGBString(s) => format!(r#"rgb "{}""#, s),
			RGBInteger(r, g, b) => format!(r#"rgb {}"#, from_argb(0, *r, *g, *b)),
			ARGBInteger(a, r, g, b) => format!(r#"rgb {}"#, from_argb(*a, *r, *g, *b)),
			VariableRGBInteger(_) => String::from("rgb variable"),
			VariableARGBInteger(_) => String::from("rgb variable"),
			PaletteFracColor(_) => todo!(),
			PaletteCBColor(_) => todo!(),
			VariablePaletteColor(_) => String::from("palette z"),
			SavedColorMap(s, _) => format!("palette {s}"),
			VariableIndex(_) => String::from("variable"),
			Background => String::from("background"),
			Index(n) => format!("{}", n),
			Black => String::from("black"),
		}
	}

	pub fn data(&self) -> Vec<f64>
	{
		match self
		{
			RGBString(_) => panic!("data() called on non-variable color type."),
			RGBInteger(_, _, _) => panic!("data() called on non-variable color type."),
			ARGBInteger(_, _, _, _) => panic!("data() called on non-variable color type."),
			VariableRGBInteger(items) => items
				.iter()
				.map(|(r, g, b)| from_argb(0, *r, *g, *b) as f64)
				.collect(),
			VariableARGBInteger(items) => items
				.into_iter()
				.map(|(a, r, g, b)| from_argb(*a, *r, *g, *b) as f64)
				.collect(),
			PaletteFracColor(_) => panic!("data() called on non-variable color type."),
			PaletteCBColor(_) => panic!("data() called on non-variable color type."),
			VariablePaletteColor(items) => items.clone(),
			SavedColorMap(_, items) => items.clone(),
			Index(_) => panic!("data() called on non-variable color type."),
			VariableIndex(items) => items.into_iter().map(|v| *v as f64).collect(),
			Background => panic!("data() called on non-variable color type."),
			Black => panic!("data() called on non-variable color type."),
		}
	}

	pub fn is_variable(&self) -> bool
	{
		match self
		{
			VariableRGBInteger(_)
			| VariableARGBInteger(_)
			| VariableIndex(_)
			| VariablePaletteColor(_)
			| SavedColorMap(_, _) => true,
			_ => false,
		}
	}

	pub fn has_alpha(&self) -> bool
	{
		match self
		{
			RGBString(s) =>
			{
				let s = s.to_string();
				if s.starts_with("0x") && s.chars().count() == 10
				{
					true
				}
				else if s.starts_with("#") && s.chars().count() == 9
				{
					true
				}
				else
				{
					false
				}
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

fn float_color_to_int(v: f64) -> u8
{
	if v < 0.0 || v > 1.0
	{
		panic!(
			"Float value must be greater than zero and less than one. Actual value: {}",
			v
		);
	}
	((v * 255.0).round()) as u8
}

/// Converts a set of `f64` red, green and blue values in the range `0 <= x <= 1` to a 3-tuple of `u8` suitable for use as
/// an [RGBInteger]
///
/// Panics if any of the arguments are not in the range `0 <= x <= 1`
///
/// Ses also [floats_to_argb]
///
/// # Arguments
/// * r - red. 0: no red, 1: fully red
/// * g - green. 0: no green, 1: fully green
/// * b - blue. 0: no blue, 1: fully blue
pub fn floats_to_rgb(r: f64, g: f64, b: f64) -> RGBInts
{
	(
		float_color_to_int(r),
		float_color_to_int(g),
		float_color_to_int(b),
	)
}

/// Converts a set of `f64` red, green and blue values in the range `0 <= x <= 1` to a 3-tuple of `u8` suitable for use as
/// an [ARGBInteger]
///
/// Panics if any of the arguments are not in the range `0 <= x <= 1`
///
/// Ses also [floats_to_rgb]
///
/// # Arguments
/// * a - alpha (transparency) value. 0: completely opaque, 1: completely transparent.
/// * r - red. 0: no red, 1: fully red
/// * g - green. 0: no green, 1: fully green
/// * b - blue. 0: no blue, 1: fully blue
pub fn floats_to_argb(a: f64, r: f64, g: f64, b: f64) -> ARGBInts
{
	(
		float_color_to_int(a),
		float_color_to_int(r),
		float_color_to_int(g),
		float_color_to_int(b),
	)
}

impl<'l> Into<ColorType<String>> for &'l str
{
	/// Converts `&str` into [RGBString]
	fn into(self) -> ColorType<String>
	{
		ColorType::RGBString(String::from(self))
	}
}

impl<'l> Into<ColorType<String>> for String
{
	/// Converts `String` into [RGBString]
	fn into(self) -> ColorType<String>
	{
		ColorType::RGBString(self)
	}
}

impl<'l> Into<ColorType<&'l str>> for &'l str
{
	/// Converts `&str` into [RGBString]
	fn into(self) -> ColorType<&'l str>
	{
		ColorType::RGBString(self)
	}
}

impl<T> Into<ColorType<T>> for ARGBInts
{
	/// Converts `(u8, u8, u8, u8)` into [ARGBInteger]
	fn into(self) -> ColorType<T>
	{
		ColorType::ARGBInteger(self.0, self.1, self.2, self.3)
	}
}

impl<T> Into<ColorType<T>> for RGBInts
{
	/// Converts `(u8, u8, u8)` into [RGBInteger]
	fn into(self) -> ColorType<T>
	{
		ColorType::RGBInteger(self.0, self.1, self.2)
	}
}

impl<T> Into<ColorType<T>> for (f64, f64, f64)
{
	/// Converts `(f64, f64, f64)` into [RGBInteger].
	/// All values must be in the range 0-1, or the function will panic.
	fn into(self) -> ColorType<T>
	{
		let ints = floats_to_rgb(self.0, self.1, self.2);
		ColorType::RGBInteger(ints.0, ints.1, ints.2)
	}
}

impl<T> Into<ColorType<T>> for (f64, f64, f64, f64)
{
	/// Converts `(f64, f64, f64, f64)` into [ARGBInteger].
	/// All values must be in the range 0-1, or the function will panic.
	fn into(self) -> ColorType<T>
	{
		let ints = floats_to_argb(self.0, self.1, self.2, self.3);
		ColorType::ARGBInteger(ints.0, ints.1, ints.2, ints.3)
	}
}

impl<T> Into<ColorType<T>> for Vec<RGBInts>
{
	/// Converts `Vec<(u8, u8, u8)>` into [VariableRGBInteger]
	fn into(self) -> ColorType<T>
	{
		ColorType::VariableRGBInteger(self)
	}
}

impl<T> Into<ColorType<T>> for Vec<ARGBInts>
{
	/// Converts `Vec<(u8, u8, u8, u8)>` into [VariableARGBInteger]
	fn into(self) -> ColorType<T>
	{
		ColorType::VariableARGBInteger(self)
	}
}

impl<T> Into<ColorType<T>> for ColorIndex
{
	/// Converts `u8` into [Index]
	fn into(self) -> ColorType<T>
	{
		ColorType::Index(self)
	}
}

impl<T> Into<ColorType<T>> for Vec<ColorIndex>
{
	/// Converts `Vec<u8>` into [VariableIndex]
	fn into(self) -> ColorType<T>
	{
		ColorType::VariableIndex(self)
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
			PaletteFracColor(_) => todo!(),
			PaletteCBColor(_) => todo!(),
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
			PaletteFracColor(_) => todo!(),
			PaletteCBColor(_) => todo!(),
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
