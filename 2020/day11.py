#! /usr/bin/env python

import sys

room = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    row = [c for c in line]
    room.append( row)

def get(x, y) -> str:
    if x < 0 or x >= len(room[0]):
        return 'L'
    if y < 0 or y >= len(room):
        return 'L'
    return room[y][x]

def occupied(x, y) -> int:
    res = 0
    for xx,yy in [(-1, -1),(-1,0),(-1,1),
            (0,-1),(0,1),
            (1, -1),(1,0 ),(1,1)]:
        c = 1
        while get(x+(c*xx), y+(c*yy)) == '.':
            c += 1
        if get(x+(c*xx), y+(c*yy)) == '#':
            res += 1
    return res

def round() -> bool:
    global room
    newroom = []
    changed = False
    for y in range(len(room)):
        newrow = []
        for x in range(len(room[0])):
            if room[y][x] == 'L' and occupied(x, y) == 0:
                newrow.append('#')
                changed = True
            elif room[y][x] == '#' and occupied(x, y) >= 5:
                newrow.append('L')
                changed = True
            else:
                newrow.append(room[y][x])
        newroom.append(newrow)
    room = newroom
    return changed

print(room)
while round():
    print(room)

count = 0
for row in room:
    for seat in row:
        if seat == '#':
            count += 1
print(count)



