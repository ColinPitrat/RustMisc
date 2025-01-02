#!/bin/sh

for d in day*
do
  pushd $d > /dev/null
  cargo -q build --release
  echo -n "$d: "
  (time target/release/$d --filename my_input.txt >/dev/null) 2>&1 | grep real | awk '{ print $2 }'
  popd > /dev/null
done | sort -k 2 -r
