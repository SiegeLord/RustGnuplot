// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

mod private
{
	use axes_common::*;

	struct Axes3D
	{
		common: AxesCommon
	}

	impl Axes3D
	{
		pub fn new() -> Axes3D
		{
			Axes3D
			{
				common: AxesCommon::new()
			}
		}
	}
}
