//! TODO

use std::fmt::Display;
use crate::util::OneWayOwned;
pub use self::ColorType::*;

pub trait IntoColor<T>: Into<ColorType<T>> + Clone {}
impl<TC, T: ?Sized + Into<ColorType<TC>> + Clone> IntoColor<TC> for T {}

pub type ColorIndex = u8;
pub type ColorComponent = u8;
pub type ColorInt = u32;

/// Option type (for lines, axes, and text) that allows the various different gnuplot
/// color formats. The gnuplot [colorspec reference](http://gnuplot.info/docs_6.0/loc3640.html)
/// also explains these
///
/// There are equivalent many ways of specifying colors, and this allows the user to chose the most convenient.
/// for example, all the following will produce the same blue color:
/// `RGBColor("blue")`, `RGBColor("0x0000ff")`, `RGBColor("#0000ff")`, `RGBColor("0x000000ff")`,
/// `RGBColor("#000000ff")`, `RGBIntegerColor(0, 0, 255)`, `ARGBColor(0, 0, 0, 255)`,
///
/// See example usage of this in `color.rs` in the
/// [Examples folder](https://github.com/SiegeLord/RustGnuplot/tree/master/gnuplot/examples) on Github
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColorType<T = String> {
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
	RGBColor(T),
    /// tuple of u8 representing red, green and blue values as 0-255
    RGBIntegerColor(u8, u8, u8),
    /// tuple of u8 representing alpha, red, green and blue values as 0-255.
    /// As with `RGBColor`, an alpha value of 0 represents a fully opaque color;
    /// an alpha value of 255 (FF) represents full transparency.
    ARGBIntegerColor(u8,u8,u8,u8),
    VariableRGBColor(Vec<(u8, u8, u8)>),
    VariableARGBColor(Vec<(u8, u8, u8, u8)>),
    PaletteFracColor(f32),
    PaletteCBColor(f32),
    PaletteZColor,
    PaletteColorMap(T),
    /// Set the color of all elements of the plot to the `n`th color in the current gnuplot color cycle.
    IndexColor(u8),
    /// A color type that sets the color per element using a index `n` which represents the `n`th
    /// color in the current gnuplot color scheme. In gnuplot this is the last element in the plot command,
    /// in Rust gnuplot, the color type takes a vector of u32, where each index is treated the same as the
    /// fixed `IndexColor`.
    /// This is useful for setting bars/boxes etc to be
    /// the same color from multiple plot commands. The `color.rs` example has examples of this usage
    VariableIndexColor(Vec<u8>),
    ///
    BackgroundColor,

    /// Fixed black color
    Black,
}

impl <T: Display> ColorType<T> {
    /// Returns the gnuplot string that will produce the requested color
	pub fn command(&self) -> String {
		match self {
            RGBColor(s) => format!(r#"rgb "{}""#, s),
            RGBIntegerColor(r, g, b) => format!(r#"rgb {}"#, from_argb(0, *r, *g, *b)),
            ARGBIntegerColor(a, r, g, b) => format!(r#"rgb {}"#, from_argb(*a, *r, *g, *b)),
            VariableRGBColor(_) => String::from("rgb variable"),
            VariableARGBColor(_) => String::from("rgb variable"),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableIndexColor(_) => String::from("variable"),
            BackgroundColor => todo!(),
            IndexColor(n) => format!("{}", n),
            Black => String::from("black"),

		}
	}

    pub fn data(&self) -> Vec<ColorInt> {
        match self {
            RGBColor(_) => panic!("data() called on non-variable color type."),
            RGBIntegerColor(_, _, _) => panic!("data() called on non-variable color type."),
            ARGBIntegerColor(_, _, _, _) => panic!("data() called on non-variable color type."),
            VariableRGBColor(items) => {
                items.iter().map(|(r, g, b)| from_argb(0, *r, *g, *b)).collect()
            },
            VariableARGBColor(items) => {
                items.into_iter().map(|(a, r, g, b)| from_argb(*a, *r, *g, *b)).collect()
            },
            PaletteFracColor(_) => panic!("data() called on non-variable color type."),
            PaletteCBColor(_) => panic!("data() called on non-variable color type."),
            PaletteZColor => panic!("data() called on non-variable color type."),
            PaletteColorMap(_) => panic!("data() called on non-variable color type."),
            IndexColor(_) => panic!("data() called on non-variable color type."),
            VariableIndexColor(items) => items.into_iter().map(|v| *v as ColorInt).collect(),
            BackgroundColor => panic!("data() called on non-variable color type."),
            Black => panic!("data() called on non-variable color type."),
        }
    }

    pub fn is_variable(&self) -> bool {
        match self {
            VariableRGBColor(_) | VariableARGBColor(_) | VariableIndexColor(_) => true,
            _ => false,
        }
    }
}

pub fn from_argb(a:u8, r:u8, g:u8, b:u8) -> u32{
    (a as u32) << 24 + (r as u32) << 16 + (g as u32) << 8 + (b as u32)
}

impl <'l> Into<ColorType<String>> for &'l str {
	fn into(self) -> ColorType<String> {
		ColorType::RGBColor(String::from(self))
	}
}

impl <'l> Into<ColorType<String>> for String {
	fn into(self) -> ColorType<String> {
		ColorType::RGBColor(self)
	}
}

impl <'l> Into<ColorType<&'l str>> for &'l str {
	fn into(self) -> ColorType<&'l str> {
		ColorType::RGBColor(self)
	}
}

// impl <'l> Into<ColorType<&'l str>> for String {
// 	fn into(self) -> ColorType<&'l str> {
// 		ColorType::RGBColor(&self)
// 	}
// }


impl<T:Display> OneWayOwned for ColorType<T>
{
	type Output = ColorType<String>;

    fn to_one_way_owned(&self) -> ColorType<String> {
        match self {
            RGBColor(s)=>RGBColor(s.to_string()),
            RGBIntegerColor(r, g,b) => RGBIntegerColor(*r, *g, *b),
            VariableRGBColor(d) => VariableRGBColor(d.clone()),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => PaletteZColor,
            PaletteColorMap(_) => todo!(),
            VariableIndexColor(d) => VariableIndexColor(d.clone()),
            BackgroundColor => BackgroundColor,
            IndexColor(n) => IndexColor(*n),
            Black => Black,
            ARGBIntegerColor(a, r, g, b) => ARGBIntegerColor(*a, *r,*g, *b),
            VariableARGBColor(d) => VariableARGBColor(d.clone()),
        }
    }
}

impl ColorType<String> {
    pub fn to_ref(&self) -> ColorType<&str> {
        match self {
            RGBColor(s)=>RGBColor(s),
            RGBIntegerColor(r,g,b) =>  RGBIntegerColor(*r, *g, *b),
            VariableRGBColor(d) => VariableRGBColor(d.to_vec()),
            VariableARGBColor(d) => VariableARGBColor(d.to_vec()),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableIndexColor(d) => VariableIndexColor(d.to_vec()),
            BackgroundColor => todo!(),
            IndexColor(n) => IndexColor(*n),
            Black => Black,
            ARGBIntegerColor(a, r, g, b) => ARGBIntegerColor(*a, *r,*g, *b),
        }
    }
}
