#!/usr/bin/env python3

import subprocess
from itertools import permutations

max = (0, None)
for perm in permutations(range(5)):
    thrust = 0
    for phase in perm:
        proc = subprocess.run(
            ["./intcode.py", "input"],
            capture_output=True,
            text=True,
            input=f"{phase}\n{thrust}"
        )
        thrust = int(proc.stdout)
    if thrust > max[0]:
        max = (thrust, perm)
print(max)
