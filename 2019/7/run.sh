#!/usr/bin/env sh
readonly DAY="${PWD##*/}"
readonly PARENT_DIR="${PWD%/*}"
readonly YEAR="${PARENT_DIR##*/}"

if [ ! -f input ]; then
    curl -O "https://adventofcode.com/$YEAR/day/$DAY/input" --header "$COOKIE"
fi
./part-1.py input
# ./part-2.py input
