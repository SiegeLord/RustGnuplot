#!/bin/sh

cp main.css doc/gnuplot/
cp search.png doc/gnuplot/
patch -p0 < js.patch
