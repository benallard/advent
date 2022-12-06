#! /usr/bin/env python

import sys

class Instr():
    def __init__(self, line):
        self.op, self.arg = line.split(' ', 1)
        self.arg = int(self.arg)
    def __str__(self):
        return self.op + ' ' + str(self.arg)

prgm = []

for line in sys.stdin:
    line = line.strip()
    if not line: 
        continue
    prgm.append(Instr(line))

def attempt(prgm):
    pc =0
    acc = 0
    seen = set()
    while pc < len(prgm) and pc not in seen:
        seen.add(pc)
        if prgm[pc].op == 'acc':
            acc += prgm[pc].arg
            pc += 1
        elif prgm[pc].op == 'jmp':
            pc += prgm[pc].arg
        elif prgm[pc].op == 'nop':
            pc += 1
    print("res: "+ str(acc))
    return pc in seen

for i in range(len(prgm)):
    copy = prgm[:]
    if copy[i].op == 'jmp':
        copy[i] = Instr('nop ' + str(copy[i].arg))
    elif copy[i].op == 'nop':
        copy[i] = Instr('jmp ' + str(copy[i].arg))
    else:
        continue
    if not attempt(copy):
        print("won")
        break


