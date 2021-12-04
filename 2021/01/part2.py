from itertools import pairwise

increases = 0

def triplewise(iterable):
    "Return overlapping triplets from an iterable"
    # triplewise('ABCDEFG') -> ABC BCD CDE DEF EFG
    for (a, _), (b, c) in pairwise(pairwise(iterable)):
        yield a, b, c


with open("input.txt") as f:
    window_iter = triplewise(int(line) for line in f.readlines())
    prev_depth = sum(next(window_iter))
    for depth in (sum(depths) for depths in window_iter):
        if depth > prev_depth:
            increases += 1
        prev_depth = depth

print(increases)
