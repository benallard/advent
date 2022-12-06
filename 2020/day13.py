#! /usr/bin/env python

import sys

est, buses = list(sys.stdin)
est = int(est)
buses = [int(b) if b != 'x' else None for b in buses.split(',')]
print(buses)
minstart = est * 2
minbus = 0

for bus in buses:
    if not bus:
        continue
    next = ((est // bus) + 1) * bus
    if next < minstart:
        minstart = next
        minbus = bus

print((minstart - est) * minbus)

current = 0
incr = buses[0]
for i, bus in enumerate(buses[1:]):
    if not bus:
        continue
    i += 1 # we skipped the first one
    i = i % bus
    print (current, incr, bus)
    while ((current + incr + i) % bus) != 0:
        print (current, (current+incr+i) % bus)
        current += incr
    current += incr
    incr = incr * bus
print(current)
