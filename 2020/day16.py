#! /usr/bin/env python

import sys

state = "FIELD"
FIELDS = {}
vmin, vmax = (99999999999999999,0)

pfields = {}


def cleanup(name, position):
    ret = False
    print('cleanup', name, position)
    for k in pfields:
        if k == position:
            continue
        if name in pfields[k]:
            ret = True
            pfields[k].remove(name)
            if len(pfields[k]) == 1:
                cleanup(pfields[k][0], k)
    return ret

def check(ticket):
    global error
    good = True
    for field in ticket:
        if vmin <= field <= vmax:
            continue
        error += field
        good = False
        break
    if not good:
        return
    for i, field in enumerate(ticket):
        #print(i, pfields[i])
        for name in pfields[i]:
            good = False
            for _range in FIELDS[name]:
                if _range[0] <= field <= _range[1]:
                    good = True
                    break
            if not good:
                print(i, "bad", name, field, FIELDS[name])
                pfields[i].remove(name)
                if len(pfields[i]) == 1:
                    cleanup(pfields[i][0], i)

error = 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    if line == "your ticket:":
        state = "MINE"
        continue
    elif line == "nearby tickets:":
        state = "THEIR"
        continue
    if state == "FIELD":
        name, ranges = line.split(":")
        ranges = ranges.strip().split(" or ")
        FIELDS[name] = []
        for r in ranges:
            v = r.split("-")
            FIELDS[name].append((int(v[0]), int(v[1])))
        vmin = min([r[0] for r in FIELDS[name]] + [vmin])
        vmax = max([r[1] for r in FIELDS[name]] + [vmax])
    elif state == "MINE":
        mine = [int(v) for v in line.split(",")]
        for i in range(len(FIELDS)):
            pfields[i] = list(FIELDS.keys())
        check(mine)
    else:
        fields = [int(v) for v in line.split(",")]
        check(fields)

print(FIELDS)
print(mine)
print(error)
print(pfields)

res = 1
for i in pfields:
    if pfields[i][0].startswith("departure"):
        res *= mine[i]

print(res)

