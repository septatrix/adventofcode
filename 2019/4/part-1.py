#!/usr/bin/env python3

import re

lower, upper = map(int, input().split("-"))

duplicate = re.compile(r"(.)\1")


def valid(n):
    as_str = str(n)
    if not duplicate.search(as_str):
        return False
    last = "0"
    for digit in as_str:
        if digit < last:
            return False
        last = digit
    return True


numbers = filter(valid, range(lower, upper))
print(sum(1 for number in numbers))
