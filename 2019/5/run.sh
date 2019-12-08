#!/usr/bin/env sh
if [ ! -f input ]; then
    curl -o input 'https://adventofcode.com/2019/day/5/input' --header "$COOKIE"
fi
echo 1 | ./part-1.py input
echo 5 | ./part-2.py input
