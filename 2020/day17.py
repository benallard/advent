#! /usr/bin/env python

import sys

class World(object):

    def __init__(self, size):
        x, y, z, w = size
        self.state = [None] * w
        for ww in range(w):
            self.state[ww] = [None] * z
            for zz in range(z):
                self.state[ww][zz] = [None] * y
                for yy in range(y):
                    self.state[ww][zz][yy] = [False] * x

    def activate(self, x, y, z, w):
        self.state[w][z][y][x] = True

    def deactivate(self, x, y, z, w):
        self.state[w][z][y][x] = False

    def neighbors(self, x, y, z,w):
        res = 0
        for dw in (-1, 0, 1):
            for dz in (-1, 0, 1):
                for dy in (-1, 0, 1):
                    for dx in (-1, 0, 1):
                        if dx == 0 and dy == 0 and dz == 0 and dw == 0:
                            continue
                        if self.get(x+dx, y+dy, z+dz, w+dw):
                            res += 1
        return res

    def get(self, x, y, z, w):
        dim = self.size()
        if x < 0 or x >= dim[0]:
            return False
        if y < 0 or y >= dim[1]:
            return False
        if z < 0 or z >= dim[2]:
            return False
        if w < 0 or w >= dim[3]:
            return False
        return self.state[w][z][y][x]

    def size(self):
        return ( len(self.state[0][0][0]), len(self.state[0][0]), len(self.state[0]), len(self.state))

    def count(self):
        dim = self.size()
        res = 0
        for w in range(dim[3]):
            for z in range(dim[2]):
                for y in range(dim[1]):
                    for x in range(dim[0]):
                        if self.state[w][z][y][x]:
                            res += 1
        return res

    @classmethod
    def fromInput(cls, input):
        res = cls((0, 0, 1, 1))
        for line in input:
            line = line.strip()
            if not line:
                return
            l = []
            for c in line:
                l.append(c == '#')

            res.state[0][0].append(l)
        return res

state = World.fromInput(sys.stdin)

for cycle in range(6):
    print(state.size(), state.count())
    dim = state.size()
    new = World((dim[0]+2, dim[1]+2, dim[2]+2, dim[3]+2))
    for w in range(-1, dim[3]+1):
        for z in range(-1, dim[2]+1):
            for y in range(-1, dim[1]+1):
                for x in range(-1, dim[0]+1):
                    if state.get(x, y, z, w):
                        if state.neighbors(x, y, z, w) in (2,3):
                            new.activate(x+1, y+1, z+1, w+1)
                        else:
                            new.deactivate(x+1, y+1, z+1, w+1)
                    else:
                        if state.neighbors(x, y, z, w) == 3:
                            new.activate(x+1, y+1, z+1, w+1)
                        else:
                            new.deactivate(x+1, y+1, z+1, w+1)
    state = new


print(state.count())
