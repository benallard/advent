#! /usr/bin/env python

import sys

map = []

code = {'.': False, '#': True}

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    map.append(list(code[c] for c in line))

def count_trees(xx, yy):
    x = 0
    y = 0

    trees = 0

    while y < len(map):
        x = x % len(map[0])
        if map[y][x]:
            trees += 1
        x += xx
        y += yy
    return trees

print(count_trees(1,1) * count_trees(3, 1) * count_trees(5, 1) * count_trees(7, 1) * count_trees(1, 2))
