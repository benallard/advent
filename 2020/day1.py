#! /usr/bin/env python

import sys

seen = set()

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    number = int(line)
    seen.add(number)
    if 2020 - number in seen:
        print( number * (2020 - number))

for a in seen:
    for b in seen:
        if 2020 - a - b in seen:
            print(a * b * (2020 - a - b))
