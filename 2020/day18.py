#! /usr/bin/env python

import sys

def calc(n1, n2, op):
    if op == '+':
        return n1 + n2
    elif op == '*':
        return n1 * n2

def evaluate(line):
    stack = []
    opstack = []
    nb = 0
    nbv = False
    for char in '(' + line + ')':
        print(char, stack, opstack)
        if char == ' ':
            continue
        elif char in '0123456789':
            nb = nb * 10 + int(char)
            nbv = True
        elif char == '(':
            opstack.append(char)
        elif char == ')':
            if nbv:
                stack.append(nb)
                nb = 0
                nbv = False
            while opstack[-1] != '(':
                n1, n2 = stack.pop(), stack.pop()
                op = opstack.pop()
                stack.append(calc(n1, n2, op))
            opstack.pop() # pop the (
        elif char in '+*':
            if nbv:
                stack.append(nb)
                nb = 0
                nbv = False
            if opstack[-1] == '+':
                n2, n1 = stack.pop(), stack.pop()
                op = opstack.pop()
                stack.append(calc(n1, n2, op))
            opstack.append(char)
    return stack.pop()

sum = 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        continue

    res = evaluate(line)
    print(res)
    sum += res

print(sum)

