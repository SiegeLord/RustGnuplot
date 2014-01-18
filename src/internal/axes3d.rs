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

pub trait Axes3DPrivate
{
	fn get_common<'l>(&'l self) -> &'l AxesCommonData;
}

impl Axes3DPrivate for Axes3D
{
	fn get_common<'l>(&'l self) -> &'l AxesCommonData
	{
		&self.common
	}
}
