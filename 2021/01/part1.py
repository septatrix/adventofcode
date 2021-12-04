increases = 0

with open("part1.txt") as f:
    prev_depth = int(f.readline())
    for depth in (int(line) for line in f.readlines()):
        if depth > prev_depth:
            increases += 1
        prev_depth = depth

print(increases)
