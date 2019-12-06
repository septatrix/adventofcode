#!/usr/bin/env python3

program = input()
parsed = list(map(int, program.split(",")))

# parsed[1] = 12
# parsed[2] = 2

counter = 0
while parsed[counter] != 99:
    modes, opcode = divmod(parsed[counter], 10)
    if opcode == 1:
        parsed[parsed[counter + 3]] = (
            parsed[parsed[counter + 1]] + parsed[parsed[counter + 2]]
        )
        counter += 4
    elif opcode == 2:
        parsed[parsed[counter + 3]] = (
            parsed[parsed[counter + 1]] * parsed[parsed[counter + 2]]
        )
        counter += 4
    elif opcode == 3:
        parsed[parsed[counter + 1]] = int(input())
        counter += 2
    elif opcode == 4:
        print(parsed[parsed[counter + 1]])
        counter += 2
    else:
        raise ValueError("Invalid opcode")
