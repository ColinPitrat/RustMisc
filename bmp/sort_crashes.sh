#!/bin/sh

# This requires to add the break at the end of the loop in main.rs
# TODO: Add an option to not have to modify the source code!
echo -n "" > crashing.txt
echo -n "" > not_crashing.txt
for file in `find resources/ -name \*.bmp`
do
  echo "$file"
  cargo run --quiet parse $file
  if [ $? -eq 0 ]
  then
    echo $file >> not_crashing.txt
  else echo $file >> crashing.txt
  fi
done
