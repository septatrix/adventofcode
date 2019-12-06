#!/usr/bin/env python3

import fileinput


def calculate_fuel(mass: int):
    return mass // 3 - 2


print(sum(map(calculate_fuel, map(int, fileinput.input()))))
