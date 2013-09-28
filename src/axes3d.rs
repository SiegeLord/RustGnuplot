// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;

pub struct Axes3D
{
	priv common: AxesCommon
}

mod private
{
	use axes_common::*;

	pub fn new_axes3d() -> super::Axes3D
	{
		super::Axes3D
		{
			common: AxesCommon::new()
		}
	}

	impl super::Axes3D
	{
		pub fn get_common<'l>(&'l self) -> &'l AxesCommon
		{
			&self.common
		}
	}
}
