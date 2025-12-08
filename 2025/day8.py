import sys


class JunctionBox:
    def __init__(self, id, x, y, z):
        self._id = id
        self._x = x
        self._y = y
        self._z = z

    def distance(self, other):
        return ((self._x - other._x) ** 2 + (self._y - other._y) ** 2 + (self._z - other._z) ** 2) ** 0.5

    def __str__(self):
        return f"{self._x},{self._y},{self._z} ({self._id})"

boxes = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    x, y, z = map(int, line.split(","))
    boxes.append(JunctionBox(len(boxes), x, y, z))

strings = set()


def shortest(boxes, strings):
    min = 999999999999999999999999999
    for i, a in enumerate(boxes):
        for b in boxes[i+1:]:
            if (a._id, b._id) in strings:
                continue
            if a.distance(b) < min:
                min = a.distance(b)
                min_a = a
                min_b = b
    return (min_a, min_b)


circuits = []

while len(strings) < 10:
    a, b = shortest(boxes, strings)
    print(f"{a} - {b}")
    strings.add((a._id, b._id))
    found = False
    for circuit in circuits:
        if a._id in circuit or b._id in circuit:
            circuit.update([a._id, b._id])
            found = True
    if not found:
        circuits.append(set([a._id, b._id]))

print(circuits)

lens = list(reversed(sorted(map(len, circuits))))
print (lens)

print(f"Part1: {lens[0] * lens[1] * lens[2]}")
