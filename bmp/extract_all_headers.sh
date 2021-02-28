#!/bin/sh

for file in `find tests/ -name \*.bmp`
do
  base=`echo $file | sed 's/tests.//' | sed 's/.bmp//'`
  txt="headers/$base.txt"
  mkdir -p `dirname "$txt"`
  echo $file
  cargo run --quiet parse $file > $txt
done
