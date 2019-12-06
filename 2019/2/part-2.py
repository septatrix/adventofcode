#!/usr/bin/env python3

from itertools import count

program = input()
parsed = tuple(map(int, program.split(",")))


def execute(parsed, arg1, arg2):
    counter = 0
    parsed[1] = arg1
    parsed[2] = arg2
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
    return parsed[0]


# https://drhagen.com/blog/superior-pairing-function/
def cantor_pairing(index1, index2):
    return (index1 + index2) * (index1 + index2 + 1) / 2 + index2


from math import sqrt, floor


def cantor_unpairing(index):
    w = floor((sqrt(8 * index + 1) - 1) / 2)
    t = (w ** 2 + w) // 2
    index2 = index - t
    index1 = w - index2
    return [index1, index2]


from time import sleep

for x, y in map(cantor_unpairing, count()):
    try:
        if execute(list(parsed), x, y) == 19690720:
            print(x * 100 + y)
            break
    except IndexError:
        pass
