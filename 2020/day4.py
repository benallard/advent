#! /usr/bin/env python

import re
import sys

def byr(y):
    return 1920 <= int(y) <= 2002

def iyr(y):
    return 2010 <= int(y) <= 2020

def eyr(y):
    return 2020 <= int(y) <= 2030

def hgt(h):
    if h.endswith('cm'):
        return 150 <= int(h[:-2]) <= 193
    elif h.endswith('in'):
        return 59 <= int(h[:-2]) <= 76
    else:
        return False

def hcl(c):
    return c[0] == '#' and re.match(r'[0-9a-f]{6}', c[1:])

def ecl(c):
    return c in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}

def pid(i):
    return len(i) == 9 and int(i)

REQUIRED = {'byr': byr, 'iyr': iyr, 'eyr': eyr, 'hgt': hgt, 'hcl': hcl, 'ecl': ecl, 'pid': pid
        #, 'cid'
        }


present = set()

valid = 0

for line in sys.stdin:
    line = line.strip()
    if not line:
        print(present)
        if len(REQUIRED.keys() - present) == 0:
            valid += 1
        present = set()
        continue
    for t in line.split(' '):
        k, v = t.split(':', 1)
        if k == 'cid':
            continue
        if REQUIRED[k](v):
            present |= set([k])

print(valid)
