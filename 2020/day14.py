#! /usr/bin/env python

import sys

mask = None
mem = {}

def write(value, addr):
    for i, bit in enumerate(reversed(mask)):
        if bit == "0":
            value &= ~(2**i)
        elif bit == "1":
            value |= 2**i
        mem[addr] = value

def addrs(addr):
    res = ""
    xs = 0
    for i, bit in enumerate(reversed(mask)):
        if bit == "X":
            res = "X"+res
            xs += 1
        elif bit == "0":
            # unchanged
            res = str(addr >> i & 0x1) + res
        elif bit == "1":
            res = "1" + res
    for idx in range(2**xs):
        addr = 0
        x = 0
        for i, bit in enumerate(reversed(res)):
            if bit == "1":
                addr += 2**i
            elif bit == "X":
                addr += ((idx >> x) & 0x1) * (2**i)
                x += 1
        yield addr
        


for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    name, value = line.split(" = ")
    if name == "mask":
        mask = value.strip()
        continue
    for addr in addrs(int(name.strip()[4:-1])):
        mem[addr] = int(value.strip())
        #write(int(value.strip()), addr)

print(mem)
print(sum(mem.values()))
