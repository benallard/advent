#! /usr/bin/env python

import sys


last = []

res1 = 26796446

def ok(num):
    good = False
    for n in last:
        if (num - n) in last:
            good = True
            break
    return good

acc = 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    line = int(line)
    acc += line
    last.append(line)
    while acc > res1:
        acc -= last[0]
        last = last[1:]

    if acc == res1:
        print(min(last) + max(last))
        break

