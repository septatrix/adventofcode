#!/usr/bin/env python3

import fileinput
from math import gcd

asteroids = set()

with fileinput.input() as map:
    for y, line in enumerate(map):
        for x, char in enumerate(line):
            if char == "#":
                asteroids.add(x + y*1j)

best = (None, 0)
for asteroid in asteroids:
    lines = set()
    for other in asteroids - {asteroid}:
        direction = asteroid - other
        # gcd is sufficient and norm or angle may be unprecise
        direction_gcd = gcd(int(direction.real), int(direction.imag))
        lines.add(direction / direction_gcd)
    if len(lines) > best[1]:
        best = (asteroid, len(lines))

print(best)
