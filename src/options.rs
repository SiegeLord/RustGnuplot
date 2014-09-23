// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

/// An enumeration of plot options you can supply to plotting commands, governing
/// things like line width, color and others
pub enum PlotOption<'l>
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
	Caption(&'l str),
	/// Sets the width of lines.
	LineWidth(f64),
	/// Sets the color of the plot element. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white). This specifies the fill color of a filled plot.
	Color(&'l str),
	/// Sets the color of the border of a filled plot (if it has one). The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white).
	BorderColor(&'l str),
	/// Sets the style of the line. Note that not all gnuplot terminals support dashed lines. See DashType for the available styles.
	LineStyle(DashType),
	/// Sets the transparency of a filled plot. `0.0` - fully transparent, `1.0` - fully opaque
	FillAlpha(f64),
	/// Sets the fill region. See FillRegion for the available regions.
	FillRegion(FillRegionType),
	/// Sets what an arrowhead looks like
	ArrowType(ArrowheadType),
	/// Sets the size of the arrowhead. This is specified in the units of graph (i.e. `1.0` would make the arrow as big as the graph).
	ArrowSize(f64),
}

/// An enumeration of possible fill regions
pub enum FillRegionType
{
	Above,
	Below,
	Between
}

/// An enumeration of possible text and label alignments
pub enum AlignType
{
	AlignLeft,
	AlignRight,
	AlignCenter,
	AlignTop,
	AlignBottom
}

/// An enumeration of possible dash styles
pub enum DashType
{
	Solid,
	SmallDot,
	Dot,
	Dash,
	DotDash,
	DotDotDash
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
			DotDotDash => 5
		}
	}
}

/// An enumeration of possible arrow head styles
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
pub enum AutoOption<T>
{
	/// Fixes the value to a specific value
	Fix(T),
	/// Lets the value scale automatically
	Auto
}

impl<T> AutoOption<T>
{
	/// Same as `Option::map`
	pub fn map<U>(&self, func: |&T| -> U) -> AutoOption<U>
	{
		match *self
		{
			Fix(ref v) => Fix(func(v)),
			Auto => Auto
		}
	}
}

/// An enumeration of label options that control label attributes
pub enum LabelOption<'l>
{
	/// Sets the offset of the label in characters
	TextOffset(f64, f64),
	/// Sets the font of the label. The string specifies the font type (e.g. "Arial") and the number specifies the size (the units are terminal dependent, but are often points)
	Font(&'l str, f64),
	/// Sets the color of the label text. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white)
	TextColor(&'l str),
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
	MarkerColor(&'l str),
	/// Sets the size of the marker. The size acts as a multiplier, with 1.0 being the default.
	MarkerSize(f64),
}

/// An enumeration of axis tick options
pub enum TickOption
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
	MajorScale(f64)
}

/// Specifies a type of axis tick
pub enum Tick<T>
{
	/// Major ticks have position and an optional label. The label may have a single C-style format specifier which will be replaced by the location of the tick
	Major(T, AutoOption<String>),
	/// Minor ticks only have position
	Minor(T)
}

/// Plot border locations
pub enum BorderLocation2D
{
	Bottom = 1,
	Left = 2,
	Top = 4,
	Right = 8
}

/// Legend options
pub enum LegendOption<'l>
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
	Title(&'l str),
	/// Specifies the maximum number of rows, when the legend is vertical
	MaxRows(u32),
	/// Specifies the maximum number of columns, when the legend is horizontal
	MaxCols(u32),
}

/// Specifies how the contours are drawn
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
	Spline(u32, u32)
}
