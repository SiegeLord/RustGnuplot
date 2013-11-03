// Copyright (c) 2013 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

use axes_common::*;

pub struct Axes3D
{
	priv common: AxesCommon
}

pub fn new_axes3d() -> Axes3D
{
	Axes3D{common: AxesCommon::new()}
}

pub trait Axes3DPrivate
{
	fn get_common<'l>(&'l self) -> &'l AxesCommon;
}

impl Axes3DPrivate for Axes3D
{
	fn get_common<'l>(&'l self) -> &'l AxesCommon
	{
		&self.common
	}
}
