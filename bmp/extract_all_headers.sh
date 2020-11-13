#!/bin/sh

# This requires to add the break at the end of the loop in main.rs
# TODO: Add an option to not have to modify the source code!
for file in `find resources/ -name \*.bmp`
do
  base=`echo $file | sed 's/resources.//' | sed 's/.bmp//'`
  cargo run --quiet parse $file > headers/$base.txt
done
