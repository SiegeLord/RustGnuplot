//! TODO

pub use self::ColorType::*;

pub trait IntoColor: Into<ColorType> + Clone {}
impl<T: ?Sized + Into<ColorType> + Clone> IntoColor for T {}

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
    RGBVariableColor(Vec<u32>),
    PaletteFracColor(f32),
    PaletteCBColor(f32),
    PaletteZColor,
    PaletteColorMap(T),
    VariableColor(Vec<u32>),
    BackgroundColor,
    IndexColor,
    Black,
}

impl ColorType {
    /// Returns the gnuplot string that will produce the requested color
	pub fn command(&self) -> String {
		match self {
			RGBColor(s) => format!(r#"rgb "{}""#, s),
            RGBIntegerColor(r, g, b) => format!(r#"rgb {}"#, from_argb(0, *r, *g, *b)),
            ARGBIntegerColor(a, r, g, b) => format!(r#"rgb {}"#, from_argb(*a, *r, *g, *b)),
            RGBVariableColor(_) => String::from("rgb variable"),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableColor(_) => String::from("variable"),
            BackgroundColor => todo!(),
            IndexColor => todo!(),
            Black => String::from("black"),

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


impl<T:ToString> ColorType<T>{
    pub fn to_one_way_owned(&self) -> ColorType<String> {
        match self {
            RGBColor(s)=>RGBColor(s.to_string()),
            RGBIntegerColor(r, g,b) => RGBIntegerColor(*r, *g, *b),
            RGBVariableColor(_) => todo!(),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableColor(d) => VariableColor(d.clone()),
            BackgroundColor => todo!(),
            IndexColor => todo!(),
            Black => Black,
            ARGBIntegerColor(a, r, g, b) => ARGBIntegerColor(*a, *r,*g, *b),
        }
    }
}
impl ColorType {
    pub fn to_ref(&self) -> ColorType<&str> {
        match self {
            RGBColor(s)=>RGBColor(&s),
            RGBIntegerColor(r,g,b) =>  RGBIntegerColor(*r, *g, *b),
            RGBVariableColor(_) => todo!(),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableColor(_) => todo!(),
            BackgroundColor => todo!(),
            IndexColor => todo!(),
            Black => Black,
            ARGBIntegerColor(a, r, g, b) => ARGBIntegerColor(*a, *r,*g, *b),
        }
    }
}
