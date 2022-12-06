#! /usr/bin/env python

import re
import sys

SYNTAX = re.compile(r"(.+) bags contain (?:(?:(?:\d+) (.+?) bags?(?:, )?)+|no other bags).")
SYNTAX1 = re.compile(r"^(.+) bags contain ")
SYNTAX2 = re.compile(r"(?:\d+) (.+?) bags?(?:, )?|no other bags")


rules = {}

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    g = SYNTAX1.match(line)
    if not g:
        print(line)
    print(g.groups())
