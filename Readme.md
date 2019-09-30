# RustGnuplot

A Gnuplot controller written in Rust.

[![Build Status](https://travis-ci.org/SiegeLord/RustGnuplot.png)](https://travis-ci.org/SiegeLord/RustGnuplot)
[![](http://meritbadge.herokuapp.com/gnuplot)](https://crates.io/crates/gnuplot)

## Documentation

See [here](http://siegelord.github.io/RustGnuplot/doc/gnuplot/index.html)

## Examples

A simple example:

```rust
let mut fg = Figure::new();
fg.axes2d()
	.set_title("A plot", &[])
	.set_legend(Graph(0.5), Graph(0.9), &[], &[])
	.set_x_label("x", &[])
	.set_y_label("y^2", &[])
	.lines(
		&[-3., -2., -1., 0., 1., 2., 3.],
		&[9., 4., 1., 0., 1., 4., 9.],
		&[Caption("Parabola")],
	);
fg.show().unwrap();
```

![Simple example plot](doc/fg.readme_example.png)

A somewhat involved 2D example (see `example1.rs` in the `examples` directory):

![Complicated example plot](doc/fg1.1.png)

## Features

* Simple 2D plots
	* lines
	* points
	* points + lines
	* error bars
	* ...and more!
* Simple 3D plots
	* surface plots
	* heatmaps
	* contours

## Building

### Via Cargo

```
cargo build
```
