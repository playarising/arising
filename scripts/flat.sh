#!/usr/bin/env bash

rm -rf flat

ROOT=contracts/
FLAT=flat/

mkdir -p "$FLAT"/base/;
mkdir -p "$FLAT"/civilizations/;
mkdir -p "$FLAT"/core/;
mkdir -p "$FLAT"/gadgets/;
mkdir -p "$FLAT"/materials/;
mkdir -p "$FLAT"/materials/raw;
mkdir -p "$FLAT"/materials/basic;

iterate_sources() {
  files=$(ls "$1"*.sol)
  for file in $files; do
    file_name=$(basename "$file")
    npx hardhat flatten "$file" > "$2""$file_name" 
  done
}

iterate_sources "$ROOT"/base/ "$FLAT"/base/
iterate_sources "$ROOT"/civilizations/ "$FLAT"/civilizations/
iterate_sources "$ROOT"/core/ "$FLAT"/core/
iterate_sources "$ROOT"/gadgets/ "$FLAT"/gadgets/
iterate_sources "$ROOT"/materials/basic/ "$FLAT"/materials/basic/
iterate_sources "$ROOT"/materials/raw/ "$FLAT"/materials/raw/

sed -i '' '/SPDX-License-Identifier: MIT/d' flat/**/*.sol
sed -i '' '/pragma solidity/d' flat/**/*.sol
sed -i '' '1s/^/pragma solidity 0.8.17;\n/' flat/**/*.sol