#!/usr/bin/env python3

import sys

val = 50
num0= 0
# Number of time we pass 0
pass0 = 0

def is0(start, steps, dir):
    return start + dir * steps == 0

# Counts how many times we cross 0
# The difficulty here is that we might cross it multiple time
# And we don't want to count the times when we stop on 0
def countCross0(start, steps, dir):
    count = 0
    for step in range(1, steps + 1):
        pos = start + dir * step
        if pos % 100 == 0:
            count += 1
    # Make sure to subtract the case where we stop on 0
    if is0(start, steps, dir):
        count -= 1
    return count

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    dir = line[0] == 'L' and -1 or 1
    steps = int(line[1:])

    print(f"Val: {val} Dir: {dir} Steps: {steps}")

    if is0(val, steps, dir):
        num0 += 1

    pass0 += countCross0(val, steps, dir)

    val += dir * steps
    val %= 100

    print(f"New Val: {val} Pass0: {pass0}")

    #print(val, prevVal, steps, dir)

print("Number of times at 0:", num0)
print("Number of passes at 0:", pass0)

print(f"Sum: {num0 + pass0}")
