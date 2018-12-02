#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR=$(dirname $0)

files=$(find ${ROOT_DIR}/src/bin -printf "%f\n" -name "*.rs" | sort -n)
echo ${files}

for src_file in ${files}; do
  name="${src_file%.*}"
  test_name="${name%-*}"

  echo -n "Running $name "
  if [[ -f "$ROOT_DIR/target/release/$name" ]]; then
    bin="$ROOT_DIR/target/release/$name"
  else
    echo "No binary found."
    continue
  fi

  echo

  output=$("$bin" <"tests/$test_name/input")
  diff_output=$(diff -u "tests/$test_name/output" <( echo "$output" ) 2>&1 || true)
  if [[ -n ${diff_output} ]]; then
    echo "Failure:"
    echo "$diff_output"
    echo
  else
    echo "Success"
  fi
done