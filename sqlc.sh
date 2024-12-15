#!/bin/sh

cargo run -- $1 > $(basename -s sql $1)cpp
clang-format -i $(basename -s sql $1)cpp
