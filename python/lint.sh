#!/usr/bin/env bash -x

cd "$(dirname "$0")"

ROOTS=(
  cross_service
  example_code
)

for ROOT in ${ROOTS[@]} ; do
  for DIR in $(ls $ROOT) ; do
    mkdir -p ".pylint/$ROOT/$DIR"
    python3 -m pylint $ROOT/$DIR/**/*.py --rcfile .pylintrc.toml > $ROOT/$DIR/pylint.txt
  done
done