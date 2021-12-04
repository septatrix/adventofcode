#!/usr/bin/env python3

from sys import argv
from enum import IntEnum, unique
import fileinput


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


class IntcodeComputer:
    def __init__(self, code: str):
        self.program = list(map(int, code.split(",")))
        self.counter = 0

    def add(self, a, b, out, mode):
        if mode % 10 == 0:
            a = self.program[a]
        if mode // 10 % 10 == 0:
            b = self.program[b]
        self.program[out] = a + b
        self.counter += 4

    def mult(self, a, b, out, mode):
        if mode % 10 == 0:
            a = self.program[a]
        if mode // 10 % 10 == 0:
            b = self.program[b]
        self.program[out] = a * b
        self.counter += 4

    def read(self, target):
        self.program[target] = int(input())
        self.counter += 2

    def write(self, out, mode):
        if mode % 10 == 0:
            out = self.program[out]
        print(out)
        self.counter += 2

    def jmp_if_true(self, check, target, mode):
        if mode % 10 == 0:
            check = self.program[check]
        if mode // 10 % 10 == 0:
            target = self.program[target]
        if check:
            self.counter = target
        else:
            self.counter += 3

    def jmp_if_false(self, check, target, mode):
        if mode % 10 == 0:
            check = self.program[check]
        if mode // 10 % 10 == 0:
            target = self.program[target]
        if not check:
            self.counter = target
        else:
            self.counter += 3

    def less_than(self, a, b, out, mode):
        if mode % 10 == 0:
            a = self.program[a]
        if mode // 10 % 10 == 0:
            b = self.program[b]
        self.program[out] = 1 if a < b else 0
        self.counter += 4

    def equals(self, a, b, out, mode):
        if mode % 10 == 0:
            a = self.program[a]
        if mode // 10 % 10 == 0:
            b = self.program[b]
        self.program[out] = 1 if a == b else 0
        self.counter += 4

    def run(self):
        while self.program[self.counter] % 100 != Opcode.HALT:
            parameter_mode, opcode = divmod(self.program[self.counter], 100)
            self.execute(opcode, parameter_mode)

    def execute(self, opcode, parameter_mode):
        if opcode == Opcode.ADD:
            self.add(*self.program[self.counter + 1 : self.counter + 4], parameter_mode)
        elif opcode == Opcode.MUL:
            self.mult(
                *self.program[self.counter + 1 : self.counter + 4], parameter_mode
            )
        elif opcode == Opcode.IN:
            self.read(self.program[self.counter + 1])
        elif opcode == Opcode.OUT:
            self.write(self.program[self.counter + 1], parameter_mode)
        elif opcode == Opcode.JMP_IF_TRUE:
            self.jmp_if_true(
                *self.program[self.counter + 1 : self.counter + 3], parameter_mode
            )
        elif opcode == Opcode.JMP_IF_FALSE:
            self.jmp_if_false(
                *self.program[self.counter + 1 : self.counter + 3], parameter_mode
            )
        elif opcode == Opcode.LESS_THAN:
            self.less_than(
                *self.program[self.counter + 1 : self.counter + 4], parameter_mode
            )
        elif opcode == Opcode.EQUALS:
            self.equals(
                *self.program[self.counter + 1 : self.counter + 4], parameter_mode
            )
        else:
            raise ValueError("Invalid opcode")



with fileinput.input(argv[1]) as file:
    code = next(file)

pc = IntcodeComputer(code)
pc.run()
