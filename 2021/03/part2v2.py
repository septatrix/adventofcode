from pathlib import Path

input = Path(__file__).resolve().parent / "input.txt"

WIDTH = 12

with input.open() as f:
    numbers = [int(n, base=2) for n in f.readlines()]

upper_bound = 1 << WIDTH
lower_bound = 0
for _ in range(WIDTH):
    midpoint = (lower_bound + upper_bound) // 2
    zero_surplus = sum(
        ((lower_bound <= n < midpoint) - (midpoint <= n < upper_bound)) for n in numbers
    )
    lower_bound = midpoint * (zero_surplus <= 0) + lower_bound * (zero_surplus > 0)
    upper_bound = midpoint * (zero_surplus > 0) + upper_bound * (zero_surplus <= 0)


oxygen_rating = midpoint

upper_bound = 1 << WIDTH
lower_bound = 0

for i in range(WIDTH):
    midpoint = (lower_bound + upper_bound) // 2
    zero_surplus = sum(
        ((lower_bound <= n < midpoint) - (midpoint <= n < upper_bound)) for n in numbers
    )
    lower_bound = midpoint * (zero_surplus > 0) + lower_bound * (zero_surplus <= 0)
    upper_bound = midpoint * (zero_surplus <= 0) + upper_bound * (zero_surplus > 0)
    if sum(lower_bound <= n < upper_bound for n in numbers) == 1:
        break

[co2_rating] = [n for n in numbers if lower_bound <= n < upper_bound]


print(oxygen_rating, co2_rating)
print(oxygen_rating * co2_rating)
