#!/usr/bin/env python3

import fileinput


def calculate_fuel(mass):
    return mass // 3 - 2


def calculate_fuel_recursively(mass):
    if calculate_fuel(mass) <= 0:
        return 0
    else:
        return calculate_fuel(mass) + calculate_fuel_recursively(calculate_fuel(mass))


modules = map(int, fileinput.input())
print(sum(map(calculate_fuel_recursively, modules)))
