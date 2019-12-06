#!/usr/bin/env python3

import fileinput

relations = {}

with fileinput.input() as stdin:
    for line in stdin:
        parent, child = line.strip().split(")")
        relations[child] = parent

path = []
parent = relations["YOU"]
path.append(parent)
while parent != "COM":
    parent = relations[parent]
    path.append(parent)

distance = 0
parent = relations["SAN"]
while parent not in path:
    parent = relations[parent]
    distance += 1
distance += path.index(parent)

print(distance)
