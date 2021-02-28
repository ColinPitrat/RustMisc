#!/bin/sh

echo -n "" > crashing.txt
echo -n "" > not_crashing.txt
for file in `find tests/ -name \*.bmp`
do
  echo "$file"
  cargo run --quiet parse $file > /dev/null
  if [ $? -eq 0 ]
  then
    echo $file >> not_crashing.txt
  else echo $file >> crashing.txt
  fi
done
