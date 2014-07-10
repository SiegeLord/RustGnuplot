# RustGnuplot

A Gnuplot controller written in Rust.

[![Build Status](https://travis-ci.org/SiegeLord/RustGnuplot.png)](https://travis-ci.org/SiegeLord/RustGnuplot)

## Documentation

See [here](http://siegelord.github.io/RustGnuplot/doc/gnuplot/index.html)

## Examples

A somewhat involved 2D example:

![2D Example plot](doc/fg1.1.png)

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

The included packages are:

* gnuplot - For the main gnuplot library
* gnuplot_examples - Some examples to try out

### Via CMake 2.8

~~~
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=<your_prefix_goes_here>
make -j
make install
~~~
