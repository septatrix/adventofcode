#!/usr/bin/env python3

from re import compile
from collections import Counter

lower, upper = map(int, input().split("-"))

duplicate = compile(r"(.)\1")


def valid(n):
    as_str = str(n)
    last = "0"
    for digit in as_str:
        if digit < last:
            return False
        last = digit
    return 2 in Counter(as_str).values()


numbers = filter(valid, range(lower, upper))
print(sum(1 for number in numbers))
