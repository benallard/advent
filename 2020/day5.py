#! /usr/bin/env python

import sys

def res(input, rule):
    length = 2**len(input)
    span = [0, length-1]
    for c in input:
        length /= 2
        if c == rule[0]:
            span[1] = span[0] + length - 1
        elif c == rule[1]:
            span[0] = span[1] - length + 1
    return span[0]

ids = set()

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    row = res(line[:7], "FB")
    col = res(line[-3:], "LR")
    id = row * 8 + col
    ids.add(id)

prev = -1
for i in sorted(ids):
    if prev == i - 2:
        print(i-1)
        break
    prev = i

