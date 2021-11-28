#!/bin/bash

for f in $(find ../.. -name Cargo.toml -printf '%h\n' | sort -u); do
  pushd $f > /dev/null;
  cargo test;
  if ! [ $? -eq 0 ]; then
    exit 1
  fi
  popd > /dev/null;
done