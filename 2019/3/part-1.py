#!/usr/bin/env python3

import fileinput

grid = {}
intersections = {}

with fileinput.input() as stdin:
    for index, wire in enumerate(stdin):
        position = 0 + 0j
        for section in wire.split(","):
            direction, distance = section[:1], int(section[1:])
            vector = 0
            if direction == "U":
                vector = 1j
            elif direction == "D":
                vector = -1j
            elif direction == "R":
                vector = 1
            elif direction == "L":
                vector = -1

            for i in range(distance):
                position += vector
                if position in grid and grid[position] != index:
                    intersections[position] = abs(position.real) + abs(position.imag)
                grid[position] = index

min_distance = min(intersections, key=intersections.get)
print(intersections[min_distance], min_distance)
