#!/bin/sh

echo -n "" > crashing.txt
echo -n "" > not_crashing.txt
find tests/ -name \*.bmp | while read file
do
  echo "$file"
  cargo run --quiet parse $file > /dev/null
  if [ $? -eq 0 ]
  then
    echo $file >> not_crashing.txt
  else
    echo $file >> crashing.txt
  fi
done
