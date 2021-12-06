from pathlib import Path

input = Path(__file__).resolve().parent / "input.txt"

WIDTH = 12

bit_counts = [0] * WIDTH

with input.open() as f:
    for line in f.readlines():
        for i, char in enumerate(line.strip()):
            bit_counts[i] += (char == "0") * 2 - 1

gamma_rate = sum((n < 0) << i for i, n in enumerate(reversed(bit_counts)))

print(gamma_rate * (gamma_rate ^ 0b111111111111))
