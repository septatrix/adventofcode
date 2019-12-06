#!/usr/bin/env python3

import fileinput

grid = {}
intersections = {}
intersections_lookup = {}

with fileinput.input() as stdin:
    for index, wire in enumerate(stdin):

        position = 0 + 0j
        steps = 0

        for section in wire.split(","):
            direction, length = section[:1], int(section[1:])
            vector = 0
            if direction == "U":
                vector = 1j
            elif direction == "D":
                vector = -1j
            elif direction == "R":
                vector = 1
            elif direction == "L":
                vector = -1

            for i in range(length):
                position += vector
                steps += 1
                if position in grid and grid[position] != index:
                    intersections_lookup[position] = True
                    intersections[position] += steps
                grid[position] = index
                if position not in intersections:
                    intersections[position] = steps

min_intersection = min(intersections_lookup, key=intersections.get)
print(min_intersection, intersections[min_intersection])
