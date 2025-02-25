// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

pub use self::AlignType::*;
pub use self::ArrowheadType::*;
pub use self::AutoOption::*;
pub use self::BorderLocation2D::*;
pub use self::ContourStyle::*;
pub use self::DashType::*;
pub use self::FillPatternType::*;
pub use self::FillRegionType::*;
pub use self::LabelOption::*;
pub use self::LegendOption::*;
pub use self::MarginSide::*;
pub use self::PaletteType::*;
pub use self::PlotOption::*;
pub use self::Tick::*;
pub use self::TickOption::*;
pub use self::XAxis::*;
pub use self::YAxis::*;
use crate::util::OneWayOwned;
use crate::writer::Writer;
use crate::ColorType;
use crate::IntoColor;

/// An enumeration of plot options you can supply to plotting commands, governing
/// things like line width, color and others
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum PlotOption<T>
{
	/// Sets the symbol used for points. The valid characters are as follows:
	///
	/// * `.` - dot
	/// * `+` - plus
	/// * `x` - cross
	/// * `*` - star
	/// * `s` - empty square
	/// * `S` - filled square
	/// * `o` - empty circle
	/// * `O` - filled circle
	/// * `t` - empty triangle
	/// * `T` - filled triangle
	/// * `d` - empty del (upside down triangle)
	/// * `D` - filled del (upside down triangle)
	/// * `r` - empty rhombus
	/// * `R` - filled rhombus
	PointSymbol(char),
	/// Sets the size of the points. The size acts as a multiplier, with 1.0 being the default.
	PointSize(f64),
	/// Sets the caption of the plot element. Set to empty to hide it from the legend.
	Caption(T),
	/// Sets the width of lines.
	LineWidth(f64),
	/// Sets the color of the plot element. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white). This specifies the fill color of a filled plot.
	ColorOpt(ColorType<T>),
	/// Sets the color of the border of a filled plot (if it has one). The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white).
	BorderColorOpt(ColorType<T>),
	/// Sets the style of the line. Note that not all gnuplot terminals support dashed lines. See DashType for the available styles.
	LineStyle(DashType),
	/// Sets the transparency of a filled plot. `0.0` - fully transparent, `1.0` - fully opaque. Cannot be used with `FillPattern`.
	FillAlpha(f64),
	/// Sets the fill region. See `FillRegionType` for the available regions.
	FillRegion(FillRegionType),
	/// Sets the fill pattern. If left at `Auto`, the pattern alternates automatically. Otherwise, see `FillPatternType` for
	/// the available patterns. Cannot be used with `FillAlpha`.
	FillPattern(AutoOption<FillPatternType>),
	/// Sets what an arrowhead looks like
	ArrowType(ArrowheadType),
	/// Sets the size of the arrowhead. This is specified in the units of graph (i.e. `1.0` would make the arrow as big as the graph).
	ArrowSize(f64),
	/// Width of the whisker bars (as a fraction of the box width) for box and whisker plots.
	WhiskerBars(f64),
	/// Which axis pair to use for the plot element.
	Axes(XAxis, YAxis),
}

#[allow(non_snake_case)]
/// Convience function to allow construction of color options implicitly from a wide range of inputs.
///
/// Currently these are:
/// * `String` or `&str` - produces a [ColorType::RGBString]
/// * `(u8, u8, u8)` - produces a [ColorType::RGBInteger]
/// * `(u8, u8, u8, u8)` - produces a [ColorType::ARGBInteger]
/// * `Vec<((u8, u8, u8))>` or `Vec<(u8, u8, u8, u8)>` - produces a
///     [ColorType::VariableRGBInteger] or [ColorType::VariableARGBInteger] respectively
/// * `u8` or `Vec<u8>` produces a [ColorType::Index] or [ColorType::VariableIndex] respectively
///
/// See `examples/color.rs` for usage of this function
pub fn Color<'l, T: IntoColor<&'l str>>(c: T) -> PlotOption<&'l str>
{
	ColorOpt(c.into())
}

#[allow(non_snake_case)]
/// Convience function to allow construction of border color options implicitly from a wide range of inputs.
/// Format and possible inputs are the same as [Color]
pub fn BorderColor<'l, T: IntoColor<&'l str>>(c: T) -> PlotOption<&'l str>
{
	BorderColorOpt(c.into())
}

impl<'l> OneWayOwned for PlotOption<&'l str>
{
	type Output = PlotOption<String>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match *self
		{
			PointSymbol(v) => PointSymbol(v),
			PointSize(v) => PointSize(v),
			Caption(v) => Caption(v.into()),
			LineWidth(v) => LineWidth(v),
			ColorOpt(ref v) => ColorOpt(v.to_one_way_owned()),
			BorderColorOpt(ref v) => BorderColorOpt(v.to_one_way_owned()),
			LineStyle(v) => LineStyle(v),
			FillAlpha(v) => FillAlpha(v),
			FillRegion(v) => FillRegion(v),
			ArrowType(v) => ArrowType(v),
			ArrowSize(v) => ArrowSize(v),
			WhiskerBars(v) => WhiskerBars(v),
			FillPattern(v) => FillPattern(v),
			Axes(x, y) => Axes(x, y),
		}
	}
}

/// An enumeration of possible X-axes
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum XAxis
{
	X1,
	X2,
}

/// An enumeration of possible Y-axes
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum YAxis
{
	Y1,
	Y2,
}

/// An enumeration of possible fill regions
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum FillRegionType
{
	Above,
	Below,
	Between,
}

/// An enumeration of possible text and label alignments
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum AlignType
{
	AlignLeft,
	AlignRight,
	AlignCenter,
	AlignTop,
	AlignBottom,
}

/// An enumeration of possible dash styles
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum DashType
{
	Solid,
	SmallDot,
	Dot,
	Dash,
	DotDash,
	DotDotDash,
}

impl DashType
{
	pub fn to_int(&self) -> i32
	{
		match *self
		{
			Solid => 1,
			SmallDot => 0,
			Dash => 2,
			Dot => 3,
			DotDash => 4,
			DotDotDash => 5,
		}
	}
}

/// An enumeration of possible arrow head styles
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ArrowheadType
{
	/// An arrow head shaped like a 'V'
	Open,
	/// An arrow head shaped like an outlined triangle
	Closed,
	/// An arrow head shaped like a filled triangle
	Filled,
	/// No arrow head
	NoArrow,
}

/// An enumeration of something that can either be fixed (e.g. the maximum of X values),
/// or automatically determined
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum AutoOption<T>
{
	/// Fixes the value to a specific value
	Fix(T),
	/// Lets the value scale automatically
	Auto,
}

impl<T> AutoOption<T>
{
	/// Same as `Option::map`
	pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> AutoOption<U>
	{
		match self
		{
			Fix(v) => Fix(f(v)),
			Auto => Auto,
		}
	}
}

impl<T: ToString> OneWayOwned for AutoOption<T>
{
	type Output = AutoOption<String>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match self
		{
			Fix(v) => Fix(v.to_string()),
			Auto => Auto,
		}
	}
}

/// An enumeration of label options that control label attributes
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum LabelOption<T>
{
	/// Sets the offset of the label in characters
	TextOffset(f64, f64),
	/// Sets the font of the label. The string specifies the font type (e.g. "Arial") and the number specifies the size (the units are terminal dependent, but are often points)
	Font(T, f64),
	/// Sets the color of the label text. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white)
	TextColorOpt(ColorType<T>),
	/// Rotates the label by a certain number of degrees
	Rotate(f64),
	/// Sets the horizontal alignment of the label text (default is left alignment). See AlignType.
	TextAlign(AlignType),
	/// Sets a marker for the label. By default no marker is drawn. The valid characters are as follows:
	///
	/// * `.` - dot
	/// * `+` - plus
	/// * `x` - cross
	/// * `*` - star
	/// * `s` - empty square
	/// * `S` - filled square
	/// * `o` - empty circle
	/// * `O` - filled circle
	/// * `t` - empty triangle
	/// * `T` - filled triangle
	/// * `d` - empty del (upside down triangle)
	/// * `D` - filled del (upside down triangle)
	/// * `r` - empty rhombus
	/// * `R` - filled rhombus
	MarkerSymbol(char),
	/// Sets the color of the marker. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white)
	MarkerColor(ColorType<T>),
	/// Sets the size of the marker. The size acts as a multiplier, with 1.0 being the default.
	MarkerSize(f64),
}

impl<'l> OneWayOwned for LabelOption<&'l str>
{
	type Output = LabelOption<String>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match self.clone()
		{
			TextOffset(v1, v2) => TextOffset(v1, v2),
			Font(v1, v2) => Font(v1.into(), v2),
			TextColorOpt(v) => TextColorOpt(v.to_one_way_owned()),
			Rotate(v) => Rotate(v),
			TextAlign(v) => TextAlign(v),
			MarkerSymbol(v) => MarkerSymbol(v),
			MarkerColor(v) => MarkerColor(v.to_one_way_owned()),
			MarkerSize(v) => MarkerSize(v),
		}
	}
}

#[allow(non_snake_case)]
/// Convience function to allow construction of text color options implicitly from a wide range of inputs.
/// Format and possible inputs are the same as [Color]
pub fn TextColor<'l, T: IntoColor<&'l str>>(c: T) -> LabelOption<&'l str>
{
	TextColorOpt(c.into())
}

/// An enumeration of axis tick options
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum TickOption<T>
{
	/// Specifies whether the ticks are drawn at the borders of the plot, or on the axis
	OnAxis(bool),
	/// If the axes are drawn on the border, this specifies whether to draw the ticks on the opposite border as well
	Mirror(bool),
	/// If the axes are drawn on the border, this specifies whether to draw the ticks pointing inward or outward
	Inward(bool),
	/// Sets the scale of the minor ticks
	MinorScale(f64),
	/// Sets the scale of the major ticks
	MajorScale(f64),
	/// Format of the tick labels, e.g. "%.1f ms" will produces labels with "1 ms, 2 ms" etc.
	Format(T),
}

impl<'l> OneWayOwned for TickOption<&'l str>
{
	type Output = TickOption<String>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match *self
		{
			OnAxis(v) => OnAxis(v),
			Mirror(v) => Mirror(v),
			Inward(v) => Inward(v),
			MinorScale(v) => MinorScale(v),
			MajorScale(v) => MajorScale(v),
			Format(v) => Format(v.into()),
		}
	}
}

/// Specifies a type of axis tick
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Tick<T, S>
{
	/// Major ticks have position and an optional label. The label may have a single C-style format specifier which will be replaced by the location of the tick
	Major(T, AutoOption<S>),
	/// Minor ticks only have position
	Minor(T),
}

impl<'l, T: Clone, S: ToString> OneWayOwned for Tick<T, S>
{
	type Output = Tick<T, String>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match self
		{
			Minor(v) => Minor(v.clone()),
			Major(v, s) => Major(v.clone(), s.to_one_way_owned()),
		}
	}
}

/// Plot border locations
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum BorderLocation2D
{
	Bottom = 1,
	Left = 2,
	Top = 4,
	Right = 8,
}

/// Plot margins
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum MarginSide
{
	MarginLeft(f32),
	MarginRight(f32),
	MarginTop(f32),
	MarginBottom(f32),
}

/// Legend options
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum LegendOption<T>
{
	/// Puts curve samples to the left of the label
	Reverse,
	/// Displays legend entries in opposite order
	Invert,
	/// Makes the legend horizontal (default is vertical)
	Horizontal,
	/// Specifies the location of the legend. The first argument specifies the horizontal
	/// placement with respect to its position, and the second argument specifies the vertical placement
	Placement(AlignType, AlignType),
	/// Title of the legend
	Title(T),
	/// Specifies the maximum number of rows, when the legend is vertical
	MaxRows(u32),
	/// Specifies the maximum number of columns, when the legend is horizontal
	MaxCols(u32),
}

impl<'l> OneWayOwned for LegendOption<&'l str>
{
	type Output = LegendOption<String>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match *self
		{
			Reverse => Reverse,
			Invert => Invert,
			Horizontal => Horizontal,
			Placement(v1, v2) => Placement(v1, v2),
			Title(v) => Title(v.into()),
			MaxRows(v) => MaxRows(v),
			MaxCols(v) => MaxCols(v),
		}
	}
}

/// Specifies how the contours are drawn
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum ContourStyle
{
	/// Draw lines between points of equal value
	Linear,
	/// Draw somewhat smoother curves between points with equal value.
	///
	/// The argument specifies the number of points to use to approximate the
	/// curve for the final drawing step (clamped to range from 2 to 100).
	Cubic(u32),
	/// Draw an even smoother curve, this time approximating the locations of
	/// points with equal value (clamped to range from 2 to 100).
	///
	/// The first argument specifies the number of points to use to approximate
	/// the curve for the final drawing step. The second argument specifies the
	/// order of the polynomial (clamped to range from 2 to 10).
	Spline(u32, u32),
}

/// Specifies what sort of palette to use
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub enum PaletteType<T>
{
	/// Use a gray palette with a specified gamma
	Gray(f32),
	/// Use a palette with that uses a predefined formula for each color component.
	/// Each formula is identified by an integer between [-36, 36]. See gnuplot documentation, or use the pre-defined constants.
	Formula(i32, i32, i32),
	/// Use a cube helix palette, with a certain start (in radians), cycles, saturation and gamma.
	CubeHelix(f32, f32, f32, f32),
	/// A custom palette
	/// is specified by a sequence of 4-tuples (with at least two elements). The first
	/// element is the grayscale value that is mapped to the remaining three elements
	/// which specify the red, green and blue components of the color.
	/// The grayscale values must be non-decreasing. All values must range from 0 to 1.
	Custom(T),
}

impl<'l> OneWayOwned for PaletteType<&'l [(f32, f32, f32, f32)]>
{
	type Output = PaletteType<Vec<(f32, f32, f32, f32)>>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		match *self
		{
			Gray(v) => Gray(v),
			Formula(v1, v2, v3) => Formula(v1, v2, v3),
			CubeHelix(v1, v2, v3, v4) => CubeHelix(v1, v2, v3, v4),
			Custom(v) => Custom(v.into()),
		}
	}
}

/// A gray palette
pub const GRAY: PaletteType<&'static [(f32, f32, f32, f32)]> = Gray(1.0);
/// Default Gnuplot palette
pub const COLOR: PaletteType<&'static [(f32, f32, f32, f32)]> = Formula(7, 5, 15);
/// Classic rainbow palette with terrible perceptual properties
pub const RAINBOW: PaletteType<&'static [(f32, f32, f32, f32)]> = Formula(33, 13, 10);
/// A black body palette
pub const HOT: PaletteType<&'static [(f32, f32, f32, f32)]> = Formula(34, 35, 36);
/// A nice default for a cube helix
pub const HELIX: PaletteType<&'static [(f32, f32, f32, f32)]> = CubeHelix(0.5, -0.8, 2.0, 1.0);

impl PaletteType<Vec<(f32, f32, f32, f32)>>
{
	pub fn write_out_commands(&self, w: &mut dyn Writer)
	{
		match *self
		{
			Gray(gamma) =>
			{
				assert!(gamma > 0.0, "Gamma must be positive");
				writeln!(w, "set palette gray gamma {:.12e}", gamma);
			}
			Formula(r, g, b) =>
			{
				assert!(r >= -36 && r <= 36, "Invalid r formula!");
				assert!(g >= -36 && g <= 36, "Invalid g formula!");
				assert!(b >= -36 && b <= 36, "Invalid b formula!");
				writeln!(w, "set palette rgbformulae {},{},{}", r, g, b);
			}
			CubeHelix(start, rev, sat, gamma) =>
			{
				assert!(sat >= 0.0, "Saturation must be non-negative");
				assert!(gamma > 0.0, "Gamma must be positive");
				writeln!(
						w,
						"set palette cubehelix start {:.12e} cycles {:.12e} saturation {:.12e} gamma {:.12e}",
						start, rev, sat, gamma
					);
			}
			Custom(ref entries) =>
			{
				if entries.len() < 2
				{
					panic!("Need at least 2 elements in a custom palette");
				}
				write!(w, "set palette defined (");

				let mut first = true;
				let mut old_x = 0.0;
				for &(x, r, g, b) in entries
				{
					if first
					{
						old_x = x;
						first = false;
					}
					else
					{
						write!(w, ",");
					}
					assert!(x >= old_x, "The gray levels must be non-decreasing!");
					old_x = x;

					write!(w, "{:.12e} {:.12e} {:.12e} {:.12e}", x, r, g, b);
				}
				writeln!(w, ")");
			}
		}
	}
}

/// Gnuplot version identifier. This is used to handle version-specific
/// features.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct GnuplotVersion
{
	pub major: i32,
	pub minor: i32,
}

/// Fill patterns.
///
/// Note that these are not guaranteed to be consistent between terminals,
/// but the following names appear to be the most common interpretations of
/// the numerical values of these patterns.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum FillPatternType
{
	Pattern0 = 0,
	BigCrosses = 1,
	SmallCrosses = 2,
	Pattern3 = 3,
	BigBackSlashes = 4,
	BigForwardSlashes = 5,
	SmallBackSlashes = 6,
	SmallForwardSlashes = 7,
	Pattern8 = 8,
}

/// Multiplot Fill Order Options
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum MultiplotFillOrder
{
	/// "rowsfirst" gnuplot option.
	RowsFirst,
	/// "columnfirst" gnuplot option.
	ColumnsFirst,
}

/// Multiplot Fill Order Options
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum MultiplotFillDirection
{
	/// "downward" gnuplot option.
	Downwards,
	/// "upward" gnuplot option.
	Upwards,
}
