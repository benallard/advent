#! /usr/bin/env python

import sys

answers = set(c for c in "abcdefghijklmnopqrstuvwxzy")

count = 0

for line in sys.stdin:
    line = line.strip()
    if not line:
        count += len(answers)
        answers = set(c for c in "abcdefghijklmnopqrstuvwxzy")
        continue
    answers &= set(c for c in line)

print(count)
