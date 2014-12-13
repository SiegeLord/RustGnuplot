// Copyright (c) 2013-2014 by SiegeLord
// 
// All rights reserved. Distributed under LGPL 3.0. For full terms see the file LICENSE.

macro_rules! first_opt
{
	($O: ident , $P: pat => $B: expr ) =>
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
	($O: ident , $P: pat => $B: expr , _ => $E: expr ) =>
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
