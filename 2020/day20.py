#! /usr/bin/env python

import sys

odir = {'N': 'S', 'E': 'W', 'S':'E', 'W':'E'}

class Tile(object):
    def __init__(self, nr, content):
        self.nr = nr
        self.content = content
        self.north = None
        self.east = None
        self.south = None
        self.west = None

    def __getitem__(self, key):
        return {'N': self.north,
                'E': self.east,
                'S': self.south,
                'W': self.west}[key]

    def __setitem__(self, key, value):
        if key == 'N':
            self.north = value
        elif key == 'E':
            self.east = value
        elif key == 'S':
            self.south = value
        elif key == 'W':
            self.west = value
        else:
            raise KeyError(key)

    def hasmatch(self, dir) -> bool:
        return self[dir] != None

    def match(self, other, dir) -> bool:
        """ search a match for the specific side of this
        """
        seq = self.side(dir)
        print(self.nr, seq)
        for i in range(4):
            other.turn()
            if other.hasmatch(odir[dir]):
                print (" ..", other.nr, other.side(odir[dir]), other[odir[dir]].nr)
                continue
            print(" ->", other.nr, other.side(odir[dir]))
            if seq == other.side(odir[dir]):
                return True
            if seq == other.side(odir[dir])[::-1]:
                print("Flip necessary")
                other.flip(dir)
                return True
        return False

    def side(self, dir):
        if dir == 'N':
            return self.content[0]
        elif dir == 'E':
            return [l[-1] for l in self.content]
        elif dir == 'S':
            return self.content[-1]
        elif dir == 'W':
            return [l[0] for l in self.content]

    def turn(self, cw=True):
        content = []
        for i in range(len(self.content[0])):
            content.append([l[i] for l in self.content][::-1])
        self.content = content
        self.north, self.east, self.south, self.west = \
                self.east, self.south, self.west, self.north
        # Turn the neighbors

    def flip(self, dir):
        if dir in "NS": # horizontal flip
            self.content = self.content[::-1]
            self.north, self.east, self.south, self.west = \
                    self.south, self.east, self.north, self.west
        elif dir in "EW": # vertical flip
            for i, line in enumerate(self.content):
                self.content[i] = line[::-1]
            self.north, self.east, self.south, self.west = \
                    self.north, self.west, self.south, self.east
        # flip the neighbors where it doesn't fit
        for dir in "NSEW":
            if not self.hasmatch(dir):
                continue
            if self.side(dir) != self[dir].side(odir[dir]):
                self[dir].flip(dir)

    def setmatch(self, dir, other):
        if self.hasmatch(dir):
            print("double match")
        self[dir] = other
        other[odir[dir]] = self

    def neighbors(self):
        res = 0
        for dir in "NESW":
            if self.hasmatch(dir):
                res += 1
        print(self.nr, res, "neighbors")
        return res

    def __str__(self):
        s = ""
        for l in self.content:
            s += "".join(l)+"\n"
        return s

tiles = []

nr = None
buffer = []
for line in sys.stdin:
    line = line.strip()
    if not line:
        tile = Tile(nr, buffer)
        tiles.append(tile)
    elif line.startswith("Tile "):
        nr = int(line[len("Tile "):-1])
        buffer = []
    else:
        buffer.append([c for c in line])

print(len(tiles), "tiles")

for i, tile in enumerate(tiles):
    for dir in "NESW":
        if tile.hasmatch(dir):
            continue
        for ttile in tiles[i:]:
            if tile.nr == ttile.nr:
                continue
            if tile.match(ttile, dir):
                print ("match", tile.nr, ttile.nr, dir)
                tile.setmatch(dir, ttile)
            else:
                pass
                #print("nomatch", tile.nr, ttile.nr, dir)

res = 1
for tile in tiles:
    if tile.neighbors() == 2:
        res *= tile.nr
print(res)



