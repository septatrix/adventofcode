#!/usr/bin/env python3

import fileinput
from math import gcd, pi
from cmath import polar
from itertools import cycle

asteroids = set()

with fileinput.input() as space_map:
    for y, line in enumerate(space_map):
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

station = best[0]
lines = {}
for asteroid in asteroids - {station}:
    diff = asteroid - station
    r, phi = polar(diff)
    phi = (phi + 2.5*pi) % (2*pi)
    if phi not in lines:
        lines[phi] = []
    lines[phi].append((r, asteroid))

#def coords_sort(a, b):
#    return (a.real - station.real) * (b.imag - station.imag) - (b.real - station.real) * (a.imag - station.imag)

for phi in lines:
    lines[phi] = sorted(lines[phi], reverse=True) #, key=cmp_to_key(coords_sort))

# print("\n".join(map(str, lines.items())))

i = 0
for angle in cycle(sorted(lines)):
    if lines[angle]:
        i += 1
        if i == 200:
            asteroid = lines[angle].pop()[1]
            print(asteroid, int(asteroid.real * 100 + asteroid.imag))
            break
        lines[angle].pop()
