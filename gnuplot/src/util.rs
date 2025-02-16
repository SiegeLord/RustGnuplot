// Copyright (c) 2013-2014 by SiegeLord
//
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

macro_rules! first_opt
{
	($O: expr , $P: pat => $B: expr ) =>
	(
		for o in $O.iter()
		{
			match *o
			{
				$P =>
				{
					$B
					break;
				},
				_ => ()
			};
		}
	)
}

macro_rules! first_opt_default
{
	($O: expr , $P: pat => $B: expr , _ => $E: expr ) =>
	(
		{
			let mut found = false;
			for o in $O.iter()
			{
				match *o
				{
					$P =>
					{
						found = true;
						$B
						break;
					},
					_ => ()
				};
			}
			if !found
			{
				$E
			}
		}
	)
}

// returns (data, num_rows, num_cols)
macro_rules! generate_data {
	($options: ident, $( $d:ident ),*) => {
		{let mut c_data = None;

		first_opt! {$options,
			ColorOpt(ref color) =>
			{
				match color
				{
					ColorType::VariableColor(values)| ColorType::RGBVariableColor(values) => {
						c_data = Some(values);
					},
					_ => (),
				}
			}
		}
		if let Some(c_values) = c_data {
			generate_data_inner!(
				$(
					$d,
				)*
				c_values
			)
		} else{
			generate_data_inner!(
				$(
					$d,
				)*
			)
		}
	}
	};
}

// returns (data, num_rows, num_cols)
macro_rules! generate_data_inner {
	($( $d:ident ),* $(,)?) => {
		{
			let mut num_rows = 0;
			let num_cols = count_data!($($d )*);
			let mut data = vec![];
			// TODO: Reserve.
			for nested_tuples!($($d,)*) in multizip!($($d, )*) //macro
			{
				$( data.push($d.get()); )*
				num_rows += 1;
			}
			(data, num_rows, num_cols)
		}
	}
}

macro_rules! nested_tuples {
	($last: ident $(,)?)=>
	{
		$last
	};
	($first: ident, $( $tail:ident ),* $(,)? ) => {
		($first, nested_tuples!($($tail, )*))
	};
}

macro_rules! multizip {
	($last: ident $(,)?)=>
	{
		($last.into_iter())
	};
	($first: ident, $( $tail:ident ),* , ) => {
		$first.into_iter().zip(multizip!($($tail, )*))
	};
}

macro_rules! replace_expr {
	($_t:tt $sub:expr) => {
		$sub
	};
}

macro_rules! count_data {
    ($($data:tt)*) => {0usize $(+ replace_expr!($data 1usize))*};
}

pub(crate) trait OneWayOwned
{
	type Output;
	fn to_one_way_owned(&self) -> Self::Output;
}

impl<'l, T: OneWayOwned> OneWayOwned for &'l [T]
{
	type Output = Vec<<T as OneWayOwned>::Output>;
	fn to_one_way_owned(&self) -> Self::Output
	{
		self.iter().map(|v| v.to_one_way_owned()).collect()
	}
}

pub(crate) fn escape(s: &str) -> String
{
	let mut res = String::with_capacity(s.len());

	for c in s.chars()
	{
		match c
		{
			'\\' => res.push_str(r"\\"),
			'\n' => res.push_str(r"\n"),
			'\t' => res.push_str(r"\t"),
			'"' => res.push_str(r#"\""#),
			// gnuplot uses ` for command substitution, seems like a
			// terrible idea.
			'`' => res.push_str(r"\`"),
			c => res.push(c),
		}
	}
	res
}

#[test]
fn escape_test()
{
	assert_eq!(r"\\", escape(r"\"));
	assert_eq!(r"\\\\", escape(r"\\"));
	assert_eq!(r#"\\\""#, escape(r#"\""#));
	assert_eq!(r#"\"\""#, escape(r#""""#));
	assert_eq!(r"\n", escape("\n"));
	assert_eq!(r"\`", escape("`"));
}
