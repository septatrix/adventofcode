position = 0
depth = 0

with open("input.txt") as f:
    for cmd, units in ((cmd, int(i)) for line in f.readlines() for cmd, i in [line.split()]):
        if cmd == "forward":
            position += units
        elif cmd == "down":
            depth += units
        elif cmd == "up":
            depth -= units

print(position)
print(depth)
print(position * depth)
