#!/usr/bin/env python3

program = input()
parsed = list(map(int, program.split(",")))

parsed[1] = 12
parsed[2] = 2

counter = 0
while parsed[counter] != 99:
    if parsed[counter] == 1:
        parsed[parsed[counter + 3]] = (
            parsed[parsed[counter + 1]] + parsed[parsed[counter + 2]]
        )
    elif parsed[counter] == 2:
        parsed[parsed[counter + 3]] = (
            parsed[parsed[counter + 1]] * parsed[parsed[counter + 2]]
        )
    counter += 4

print(parsed[0])
