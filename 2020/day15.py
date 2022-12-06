#! /usr/bin/env python

INPUT = [20, 9, 11, 0 ,1 ,2]
MAX = 2020
#INPUT = [0,3,6]
MAX = 30_000_000

aage = {}
age = {}
last = None

for i in range(MAX):
    if i < len(INPUT):
        last = INPUT[i]
    elif last in aage:
        last = age[last] - aage[last]
    else:
        last = 0
    if last in age:
        aage[last] = age[last]
    age[last] = i
    if i % 100_000 == 0:
        print(i)

print(last)
