// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

pub trait DataType
{
	fn get(&self) -> f64;
}

macro_rules! impl_data_type
{
	($T:ty) =>
	(
		impl<'l> DataType for &'l $T
		{
			fn get(&self) -> f64
			{
				self.to_f64().unwrap()
			}
		}
	)
}

macro_rules! impl_data_type_ref
{
	($T:ty) =>
	(
		impl DataType for $T
		{
			fn get(&self) -> f64
			{
				self.to_f64().unwrap()
			}
		}
	)
}

impl_data_type!(u8);
impl_data_type!(u16);
impl_data_type!(u32);
impl_data_type!(u64);
impl_data_type!(uint);

impl_data_type!(i8);
impl_data_type!(i16);
impl_data_type!(i32);
impl_data_type!(i64);
impl_data_type!(int);

impl_data_type!(f32);
impl_data_type!(f64);

impl_data_type_ref!(u8);
impl_data_type_ref!(u16);
impl_data_type_ref!(u32);
impl_data_type_ref!(u64);
impl_data_type_ref!(uint);

impl_data_type_ref!(i8);
impl_data_type_ref!(i16);
impl_data_type_ref!(i32);
impl_data_type_ref!(i64);
impl_data_type_ref!(int);

impl_data_type_ref!(f32);
impl_data_type_ref!(f64);
