#! /usr/bin/env python

import sys

deck1 = []
deck2 = []

current = deck1
for line in sys.stdin:
    line = line.strip()
    if not line:
        current = deck2
        continue
    elif line.startswith("Player"):
        continue
    current.append(int(line))

deck1.reverse()
deck2.reverse()

print(deck1, deck2)

while deck1 and deck2:
    card1 = deck1.pop()
    card2 = deck2.pop()
    recurse = len(deck1) > card1 and len(deck2) > card2
    if recurse:
        print("recursing", card1, deck1, card2, deck2)
    if card1 > card2:
        deck1.insert(0, card1)
        deck1.insert(0, card2)
    else:
        deck2.insert(0, card2)
        deck2.insert(0, card1)

score = 0
for i, card in enumerate(deck1 + deck2):
    score += (i+1)*card

print(score)



