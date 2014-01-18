// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;

pub struct Axes3D
{
	priv common: AxesCommonData
}

pub fn new_axes3d() -> Axes3D
{
	Axes3D{common: AxesCommonData::new()}
}

impl AxesCommon for Axes3D
{
	fn get_common_data_mut<'l>(&'l mut self) -> &'l mut AxesCommonData
	{
		&mut self.common
	}

	fn get_common_data<'l>(&'l self) -> &'l AxesCommonData
	{
		&self.common
	}
}

pub trait Axes3DPrivate
{

}

impl Axes3DPrivate for Axes3D
{

}
