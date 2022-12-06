#! /usr/bin/env python

import sys

all = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    all.append(int(line))

prev = 0
ones = 0
threes = 1
pdiff = 0
conseq = 1
factors = {1: 1, 2: 2, 3: 4, 4: 7, 5: 13, 6: 24}
p2 = 1
for a in sorted(all):
    if a - prev == 1:
        ones += 1
        if pdiff == 1:
            conseq += 1
        else:
            p2 *= factors[conseq]
            conseq = 1
    elif a - prev == 3:
        threes += 1
    elif a - prev == 2:
        print(prev, a)
    pdiff = a - prev
    prev = a

p2 *= factors[conseq]

print(ones * threes)
print (p2)

