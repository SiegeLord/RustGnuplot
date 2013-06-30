/// An enumeration of plot options you can supply to plotting commands, governing
/// things like line width, color and others
pub enum PlotOption<'self>
{
	/// Sets the symbol used for points. The valid characters are as follows:
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
	PointSize(float),
	/// Sets the caption of the plot element. Set to empty to hide it from the legend.
	Caption(&'self str),
	/// Sets the width of lines.
	LineWidth(float),
	/// Sets the color of the plot element. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white). This specifies the fill color of a filled plot.
	Color(&'self str),
	/// Sets the dash type. Note that not all gnuplot terminals support dashed lines. See [DashType](#enum-dashtype) for the available types.
	LineType(DashType),
	/// Sets the transparency of a filled plot. `0.0` - fully transparent, `1.0` - fully opaque
	FillAlpha(float),
	/// Sets the fill region. See See [FillRegion](#enum-fillregion) for the available regions.
	FillRegion(FillRegion)
}

/// An enumeration of possible fill regions
pub enum FillRegion
{
	Above,
	Below,
	Between
}

/// An enumeration of possible text alignments
pub enum AlignType
{
	AlignLeft,
	AlignRight,
	AlignCenter
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

/// An enumeration of label options that control label attributes
pub enum LabelOption<'self>
{
	/// Sets the offset of the label in characters
	Offset(float, float),
	/// Sets the font of the label. The string specifies the font type (e.g. "Arial") and the number specifies the size (the units are terminal dependent, but are often points)
	Font(&'self str, float),
	/// Sets the color of the label text. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white)
	TextColor(&'self str),
	/// Rotates the label by a certain number of degrees
	Rotate(float),
	/// Sets a marker for the label. By default no marker is drawn. The valid characters are as follows:
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
	MarkerColor(&'self str),
	/// Sets the size of the marker. The size acts as a multiplier, with 1.0 being the default.
	MarkerSize(float),
	/// Sets the horizontal alignment of the label text (default is left alignment). See [AlignType](#enum-aligntype).
	Align(AlignType),
}

pub enum ArrowOption<'self>
{
	/// Sets what an arrowhead looks like
	HeadType(ArrowheadType),
	/// Sets the size of the arrowhead. This is specified in the units of graph (i.e. `1.0` would make the arrow as big as the graph).
	HeadSize(float),
	/// Sets what an arrow shaft looks like
	ShaftType(DashType),
	/// Sets how wide the arrow shaft is
	ShaftWidth(float),
	/// Sets the color of the arrow. The passed string can be a color name
	/// (e.g. "black" works), or an HTML color specifier (e.g. "#FFFFFF" is white)
	ArrowColor(&'self str),
}

mod private
{
	use super::*;
	
	impl DashType
	{
		pub fn to_int(&self) -> int
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
}
