#!/usr/bin/env python3

import sys

val = 50
num0= 0

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    dir = line[0] == 'L' and -1 or 1
    steps = int(line[1:])

    val += dir * steps
    val = (val + 100) % 100
    print(val)
    if val == 0:
        num0 += 1

print("Number of times at 0:", num0)

