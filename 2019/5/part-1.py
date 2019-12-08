#!/usr/bin/env python3

from enum import IntEnum, unique

program = input()
parsed = list(map(int, program.split(",")))


@unique
class Opcode(IntEnum):
    ADD = 1
    MUL = 2
    IN = 3
    OUT = 4
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
    else:
        raise ValueError("Invalid opcode")
