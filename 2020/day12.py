#! /usr/bin/env python

import sys

dir = {'N': (0, -1),
        'S': (0, 1),
        'E': (1, 0),
        'W': (-1, 0)}

face = 'E'
x=0
y=0

wpx = 10
wpy = -1

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    amount = int(line[1:])
    if line[0] in dir:
        wpx += dir[line[0]][0] * amount
        wpy += dir[line[0]][1] * amount
    elif line[0] == 'F':
        x += wpx * amount
        y += wpy * amount
    elif line[0] == 'R':
        for i in range(amount // 90):
            wpx, wpy = (-wpy, wpx)
    elif line[0] == 'L':
        for i in range(amount // 90):
            wpx, wpy = (wpy, -wpx)

print(abs(x)+abs(y))

