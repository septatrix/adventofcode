#!/usr/bin/env sh
if [ ! -f input ]; then
    curl -o input 'https://adventofcode.com/2019/day/5/input' --header "$COOKIE"
fi
cat input | ./part-1.py
cat input | ./part-2.py
