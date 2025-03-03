#!/usr/bin/env bash

rm -rf flat

ROOT=contracts/
FLAT=flat/

mkdir -p "$FLAT"/base/;
mkdir -p "$FLAT"/codex/;
mkdir -p "$FLAT"/core/;
mkdir -p "$FLAT"/items/;

iterate_sources() {
  files=$(ls "$1"*.sol)
  for file in $files; do
    file_name=$(basename "$file")
    npx hardhat flatten "$file" > "$2""$file_name" 
  done
}

iterate_sources "$ROOT"base/ "$FLAT"base/
iterate_sources "$ROOT"codex/ "$FLAT"codex/
iterate_sources "$ROOT"core/ "$FLAT"core/
iterate_sources "$ROOT"items/ "$FLAT"/items/

sed -i '/SPDX-License-Identifier: MIT/d' flat/**/*.sol
sed -i '/pragma solidity/d' flat/**/*.sol
sed -i '1s/^/pragma solidity 0.8.25;\n/' flat/**/*.sol
