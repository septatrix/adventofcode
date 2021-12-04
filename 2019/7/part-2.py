#!/usr/bin/env python3

import subprocess
from itertools import permutations

max = (0, None)
for perm in permutations(range(5)):
    amps = [
        subprocess.Popen(
            ["./intcode.py", "input"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
        )
        for i in perm
    ]
    for amp, phase in zip(amps, perm):
        print(amp, phase)
