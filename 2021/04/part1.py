import sys
from pathlib import Path

input = Path(__file__).resolve().parent / "input.txt"


def grouper(iterable, n):
    "Collect data into non-overlapping fixed-length chunks or blocks"
    args = [iter(iterable)] * n
    return zip(*args)


with input.open() as f:
    draws = [int(n) for n in f.readline().split(",")]
    boards = list(
        grouper(([int(n) for n in line.split()] for line in f if line != "\n"), 5)
    )

lines: list[set] = []

for board in boards:
    for row in board:
        lines.append(set(row))
    for column in zip(*board):
        lines.append(set(column))

for number in draws:
    for i, line in enumerate(lines):
        line.discard(number)
        if not line:
            board_id = i // 10
            unmarked_fields = set(
                field
                for line in lines[board_id * 10 : (board_id + 1) * 10]
                for field in line
            )
            unmarked_fields.discard(number)
            print(sum(unmarked_fields) * number)
            sys.exit(0)
