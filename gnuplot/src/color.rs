pub use self::ColorType::*;

pub trait IntoColor: Into<ColorType> + Clone {}
impl<T: ?Sized + Into<ColorType> + Clone> IntoColor for T {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ColorType<T = String> {
	RGBColor(T),
    RGBIntegerColor(u16),
    RGBVariableColor,
    PaletteFracColor(f32),
    PaletteCBColor(f32),
    PaletteZColor,
    PaletteColorMap(T),
    VariableColor,
    BackgroundColor,
    IndexColor,
    Black,
}

impl ColorType {
	pub fn command(&self) -> String {
		match self {
			RGBColor(s) => format!(r#"rgb "{}""#, s),
            RGBIntegerColor(_) => todo!(),
            RGBVariableColor => todo!(),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableColor => todo!(),
            BackgroundColor => todo!(),
            IndexColor => todo!(),
            Black => String::from("black"),
		}
	}
}

impl <'l> Into<ColorType<&'l str>> for &'l str {
	fn into(self) -> ColorType<&'l str> {
		ColorType::RGBColor(self)
	}
}

impl<T:ToString> ColorType<T>{
    pub fn to_one_way_owned(&self) -> ColorType<String> {
        match self {
            RGBColor(s)=>RGBColor(s.to_string()),
            RGBIntegerColor(c) => RGBIntegerColor(*c),
            RGBVariableColor => todo!(),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableColor => todo!(),
            BackgroundColor => todo!(),
            IndexColor => todo!(),
            Black => todo!(),
            // x=>x,
        }
    }
}
impl ColorType {
    pub fn to_ref(&self) -> ColorType<&str> {
        match self {
            RGBColor(s)=>RGBColor(&s),
            RGBIntegerColor(_) => todo!(),
            RGBVariableColor => todo!(),
            PaletteFracColor(_) => todo!(),
            PaletteCBColor(_) => todo!(),
            PaletteZColor => todo!(),
            PaletteColorMap(_) => todo!(),
            VariableColor => todo!(),
            BackgroundColor => todo!(),
            IndexColor => todo!(),
            Black => todo!(),
        }
    }
}
