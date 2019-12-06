#!/usr/bin/env python3

import fileinput

relations = {}

with fileinput.input() as stdin:
    for line in stdin:
        parent, child = line.strip().split(")")
        relations[child] = parent

orbits = 0
for planet in relations:
    parent = relations[planet]
    orbits += 1
    while parent != "COM":
        parent = relations[parent]
        orbits += 1

print(orbits)
