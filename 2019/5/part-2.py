#!/usr/bin/env python3

from enum import IntEnum, unique
from sys import argv
import fileinput

parsed = []
with fileinput.input(argv[1]) as file:
    for line in file:
        parsed += map(int, line.split(","))


@unique
class Opcode(IntEnum):
    ADD = 1
    MUL = 2
    IN = 3
    OUT = 4
    JMP_IF_TRUE = 5
    JMP_IF_FALSE = 6
    LESS_THAN = 7
    EQUALS = 8
    HALT = 99


counter = 0
while parsed[counter] % 100 != Opcode.HALT:
    parameter_mode, opcode = divmod(parsed[counter], 100)
    if opcode == Opcode.ADD:
        a, b, out = parsed[counter + 1 : counter + 4]
        if not parameter_mode & 1:
            a = parsed[a]
        if not parameter_mode >> 1 & 1:
            b = parsed[b]
        parsed[out] = a + b
        counter += 4
    elif opcode == Opcode.MUL:
        a, b, out = parsed[counter + 1 : counter + 4]
        if not parameter_mode & 1:
            a = parsed[a]
        if not parameter_mode >> 1 & 1:
            b = parsed[b]
        parsed[out] = a * b
        counter += 4
    elif opcode == Opcode.IN:
        parsed[parsed[counter + 1]] = int(input("Input: "))
        counter += 2
    elif opcode == Opcode.OUT:
        out = parsed[counter + 1]
        if not parameter_mode & 1:
            out = parsed[out]
        print(f"Output: {out}")
        counter += 2
    elif opcode == Opcode.JMP_IF_TRUE:
        check, target = parsed[counter + 1 : counter + 3]
        if not parameter_mode & 1:
            check = parsed[check]
        if not parameter_mode >> 1 & 1:
            target = parsed[target]
        if check:
            counter = target
        else:
            counter += 3
    elif opcode == Opcode.JMP_IF_FALSE:
        check, target = parsed[counter + 1 : counter + 3]
        if not parameter_mode & 1:
            check = parsed[check]
        if not parameter_mode >> 1 & 1:
            target = parsed[target]
        if not check:
            counter = target
        else:
            counter += 3
    elif opcode == Opcode.LESS_THAN:
        a, b, out = parsed[counter + 1 : counter + 4]
        if not parameter_mode & 1:
            a = parsed[a]
        if not parameter_mode >> 1 & 1:
            b = parsed[b]
        parsed[out] = 1 if a < b else 0
        counter += 4
    elif opcode == Opcode.EQUALS:
        a, b, out = parsed[counter + 1 : counter + 4]
        if not parameter_mode & 1:
            a = parsed[a]
        if not parameter_mode >> 1 & 1:
            b = parsed[b]
        parsed[out] = 1 if a == b else 0
        counter += 4
    else:
        raise ValueError("Invalid opcode")
