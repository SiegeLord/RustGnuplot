# RustGnuplot

A Gnuplot controller written in Rust.

[![Build Status](https://travis-ci.org/SiegeLord/RustGnuplot.png)](https://travis-ci.org/SiegeLord/RustGnuplot)

See http://siegelord.github.io/RustGnuplot/ for documentation.

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
	* TBA

## Building

~~~
mkdir build
cd build
cmake .. -DCMAKE_INSTALL_PREFIX=<your_prefix_goes_here>
make -j
make install
~~~
