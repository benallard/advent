#! /usr/bin/env python

import sys
import re

REGEX = re.compile(r"(\d+)-(\d+) (.): (.+)")

def valid1(f, to, char, password):
    count = 0
    for c in password:
        if c == char:
            count+=1
    return f <= count <= to

def valid(p1, p2, char, password):
    return (password[p1-1] == char) ^ (password[p2-1] == char)

total = 0

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    g = REGEX.match(line)
    f, to, char, password = g.groups()

    if valid(int(f) ,int(to), char, password):
        total += 1;

print(total)
