//! TODO

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

/// Option type (for lines, axes, and text) that allows the various different gnuplot
/// color formats. The gnuplot [colorspec reference](http://gnuplot.info/docs_6.0/loc3640.html)
/// also explains these.
///
/// There are equivalent many ways of specifying colors, and this allows the user to chose the most convenient.
/// for example, all the following will produce the same blue color:
/// `RGBColor("blue")`, `RGBColor("0x0000ff")`, `RGBColor("#0000ff")`, `RGBColor("0x000000ff")`,
/// `RGBColor("#000000ff")`, `RGBIntegerColor(0, 0, 255)`, `ARGBColor(0, 0, 0, 255)`,
///
/// See example usages of these colors in `color.rs`, `variable_color.rs` in the
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
	VariableRGBIntegers(Vec<RGBInts>),
	/// Vector of tuples of `u8` (as per `ARGBColor`), but as with `VariableRGBColor`, a separate
	/// color value is given for each data point.
	VariableARGBIntegers(Vec<ARGBInts>),
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
	/// `axes.create_colormap()` function.
	SavedColorMap(T, Vec<f64>),
	/// Set the color of all elements of the plot to the `n`th color in the current gnuplot color cycle.
	Index(ColorIndex),
	/// A color type that sets the color per element using a index `n` which represents the `n`th
	/// color in the current gnuplot color scheme. In gnuplot this is the last element in the plot command,
	/// in Rust gnuplot, the color type takes a vector of u8, where each index is treated the same as the
	/// fixed `IndexColor`.
	/// This is useful for setting bars/boxes etc to be
	/// the same color from multiple plot commands. The `color.rs` example has examples of this usage.
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
			VariableRGBIntegers(_) => String::from("rgb variable"),
			VariableARGBIntegers(_) => String::from("rgb variable"),
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
			VariableRGBIntegers(items) => items
				.iter()
				.map(|(r, g, b)| from_argb(0, *r, *g, *b) as f64)
				.collect(),
			VariableARGBIntegers(items) => items
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
			VariableRGBIntegers(_)
			| VariableARGBIntegers(_)
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
			ARGBInteger(_, _, _, _) | VariableARGBIntegers(_) => true,
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

pub fn floats_to_rgb(r: f64, g: f64, b: f64) -> RGBInts
{
	(
		float_color_to_int(r),
		float_color_to_int(g),
		float_color_to_int(b),
	)
}

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
	fn into(self) -> ColorType<String>
	{
		ColorType::RGBString(String::from(self))
	}
}

impl<'l> Into<ColorType<String>> for String
{
	fn into(self) -> ColorType<String>
	{
		ColorType::RGBString(self)
	}
}

impl<'l> Into<ColorType<&'l str>> for &'l str
{
	fn into(self) -> ColorType<&'l str>
	{
		ColorType::RGBString(self)
	}
}

impl<T> Into<ColorType<T>> for ARGBInts
{
	fn into(self) -> ColorType<T>
	{
		ColorType::ARGBInteger(self.0, self.1, self.2, self.3)
	}
}

impl<T> Into<ColorType<T>> for RGBInts
{
	fn into(self) -> ColorType<T>
	{
		ColorType::RGBInteger(self.0, self.1, self.2)
	}
}

impl<T> Into<ColorType<T>> for (f64, f64, f64)
{
	fn into(self) -> ColorType<T>
	{
		let ints = floats_to_rgb(self.0, self.1, self.2);
		ColorType::RGBInteger(ints.0, ints.1, ints.2)
	}
}

impl<T> Into<ColorType<T>> for (f64, f64, f64, f64)
{
	fn into(self) -> ColorType<T>
	{
		let ints = floats_to_argb(self.0, self.1, self.2, self.3);
		ColorType::ARGBInteger(ints.0, ints.1, ints.2, ints.3)
	}
}

impl<T> Into<ColorType<T>> for Vec<RGBInts>
{
	fn into(self) -> ColorType<T>
	{
		ColorType::VariableRGBIntegers(self)
	}
}

impl<T> Into<ColorType<T>> for Vec<ARGBInts>
{
	fn into(self) -> ColorType<T>
	{
		ColorType::VariableARGBIntegers(self)
	}
}

impl<T> Into<ColorType<T>> for Vec<ColorIndex>
{
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
			VariableRGBIntegers(d) => VariableRGBIntegers(d.clone()),
			PaletteFracColor(_) => todo!(),
			PaletteCBColor(_) => todo!(),
			VariablePaletteColor(d) => VariablePaletteColor(d.clone()),
			SavedColorMap(s, d) => SavedColorMap(s.to_string(), d.clone()),
			VariableIndex(d) => VariableIndex(d.clone()),
			Background => Background,
			Index(n) => Index(*n),
			Black => Black,
			ARGBInteger(a, r, g, b) => ARGBInteger(*a, *r, *g, *b),
			VariableARGBIntegers(d) => VariableARGBIntegers(d.clone()),
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
			VariableRGBIntegers(d) => VariableRGBIntegers(d.to_vec()),
			VariableARGBIntegers(d) => VariableARGBIntegers(d.to_vec()),
			PaletteFracColor(_) => todo!(),
			PaletteCBColor(_) => todo!(),
			VariablePaletteColor(d) => VariablePaletteColor(d.to_vec()),
			SavedColorMap(s, d) => SavedColorMap(s, d.to_vec()),
			VariableIndex(d) => VariableIndex(d.to_vec()),
			Background => todo!(),
			Index(n) => Index(*n),
			Black => Black,
			ARGBInteger(a, r, g, b) => ARGBInteger(*a, *r, *g, *b),
		}
	}
}
