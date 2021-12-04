position = 0
depth = 0
aim = 0

with open("input.txt") as f:
    for cmd, units in ((cmd, int(i)) for line in f.readlines() for cmd, i in [line.split()]):
        if cmd == "forward":
            position += units
            depth += aim*units
        elif cmd == "down":
            aim += units
        elif cmd == "up":
            aim -= units

print(position)
print(depth)
print(position * depth)
