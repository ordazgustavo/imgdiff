#!/bin/bash

# set -xe

warmup=2
runs=10

apath="assets/4k/a.png"
bpath="assets/4k/b.png"
cpath="assets/4k/c.png"

hyperfine \
  --warmup $warmup \
  --runs $runs \
  --setup "cargo b -r" \
  --prepare "rm $cpath || true" \
  "./target/release/imgdiff $apath $bpath $cpath" \
  --cleanup "rm $cpath"
  # --prepare "rm $cpath || true" \
  # "pixelmatch $apath $bpath $cpath 0" \
