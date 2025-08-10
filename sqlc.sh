#!/bin/sh

if [ -z "$1" ]; then
  echo "No SQL file provided for compilation."
  exit 1
fi
if [ ! -f "$1" ]; then
  echo "The file '$1' does not exist."
  exist 1
fi

cargo run -- $1 > $(basename -s sql $1)cpp
clang-format -i $(basename -s sql $1)cpp
clang -lstdc++ -lm -std=c++23 -o $(basename -s .sql $1) $(basename -s sql $1)cpp
