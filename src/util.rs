macro_rules! first_opt
{
	($O : ident , $P : pat => $B : expr ) =>
	(
		for $O.iter().advance |o|
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
