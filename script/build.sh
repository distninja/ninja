#!/bin/bash

build=$(date +%FT%T%z)
target="target/release/ninja"

if [ "$1" = "offline" ]; then
  build=$build cargo build --release --all-features --all-targets --offline
else
  build=$build cargo build --release --all-features --all-targets
fi

if [ -f $target ]; then upx $target; fi
if [ -f "$target".exe ]; then upx "$target".exe; fi
