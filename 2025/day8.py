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

    def __hash__(self):
        return hash("JunctionBox" + str(self._id))


class LightString:
    def __init__(self, a, b):
        self._a = a
        self._b = b
        self._length = a.distance(b)

    def ends(self):
        return (self._a, self._b)


boxes = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    x, y, z = map(int, line.split(","))
    boxes.append(JunctionBox(len(boxes), x, y, z))

all_strings = []

for i, a in enumerate(boxes):
    for b in boxes[i + 1:]:
        all_strings.append(LightString(a, b))

all_strings = list(sorted(all_strings, key=lambda x: x._length))

def add_string(circuits, a, b):
    found = None
    found2 = None
    for i, circuit in enumerate(circuits):
        if a._id in circuit or b._id in circuit:
            if found is None:
                circuit.update([a._id, b._id])
                found = i
            else:
                circuits[found].update(circuit)
                found2 = i
                break  # No need to look further

    if found is None:
        circuits.append(set([a._id, b._id]))
    if found2 is not None:
        del circuits[found2]
    return circuits

MAX = 1000

circuits = []

for i in range(MAX):
    a, b = all_strings[i].ends()
    print(f"{a} - {b}")
    circuits = add_string(circuits, a, b)

    print(circuits)

print(circuits)

lens = list(reversed(sorted(map(len, circuits))))
print(lens)

print(f"Part1: {lens[0] * lens[1] * lens[2]}")

def complete(circuits):
    return len(circuits) == 1 and sum(map(len, circuits)) == len(boxes)

idx = MAX
while not complete(circuits):
    a, b = all_strings[idx].ends()
    print(f"{a} - {b}")
    circuits = add_string(circuits, a, b)
    print(circuits)

    print (f"Part2: {a._x * b._x}")
    idx += 1

# 17001512: too low