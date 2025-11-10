#!/bin/bash
output=$(dist build 2>&1 | tee /dev/stderr)
if [ $? -eq 0 ]; then
  bin_path=$(echo "$output" | grep "\[bin\] gua" -B1 | head -1 | sed 's/.* //')
  dir_name=$(basename "$bin_path" .tar.xz)
  ./target/distrib/$dir_name/gua
fi
