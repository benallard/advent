#! /usr/bin/env python

import sys

check = False

rules = {}


def valid(input, nr):
    rule = rules[nr]
    if isinstance(rule, str):
        if input.startswith(rule):
            yield len(rule)
    else:
        for choice in rule:
            yield from valid_seq(input, choice)

def valid_seq(input, nrs):
    # match the first
    if not nrs:
        yield 0
        return
    for l in valid(input, nrs[0]):
        # and the next's
        for v in valid_seq(input[l:], nrs[1:]):
            yield v+l

def match(text):
    for v in valid(text, 0):
        if v == len(text):
            return True
        else:
            print(text, v, len(text))
    return False

valids = 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        check = True
        rules[8] = [(42,), (42, 8)]
        rules[11] = [(42, 31), (42, 11, 31)]
        print(rules)
        continue
    if not check:
        nr, rule = line.split(":")
        if '"' in rule:
            rules[int(nr)] = rule.strip()[1:-1]
            continue
        rule = rule.strip().split("|")
        rules[int(nr)] = [tuple(int(v) for v in r.strip().split(' ')) for r in rule]
    else:
        if match(line):
            valids += 1

print(valids)
