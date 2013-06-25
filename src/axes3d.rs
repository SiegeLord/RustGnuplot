mod private
{
	use axes_common::*;

	struct Axes3D
	{
		common : AxesCommon
	}

	impl Axes3D
	{
		pub fn new() -> Axes3D
		{
			Axes3D
			{
				common : AxesCommon::new()
			}
		}
	}
}
