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

class Memoize:
    def __init__(self, f):
        self.f = f
        self.memo = {}
    def __call__(self, *args):
        if not args in self.memo:
            print("Computing: ", args)
            self.memo[args] = self.f(*args)
        #Warning: You may wish to do a deepcopy here if returning objects
        return self.memo[args]

@Memoize
def dist(a, b):
    return a.distance(b)

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
            if dist(a,b) < min:
                min = dist(a, b)
                min_a = a
                min_b = b
    return (min_a, min_b)


circuits = []

while len(strings) < 1000:
    a, b = shortest(boxes, strings)
    print(f"{a} - {b}")
    strings.add((a._id, b._id))
    found = None
    found2 = None
    for i, circuit in enumerate(circuits):
        if a._id in circuit or b._id in circuit:
            if not found:
                circuit.update([a._id, b._id])
                found = i
            else:
                circuits[found].update(circuit)
                found2 = i
                break # No need to look further

    if found is None:
        circuits.append(set([a._id, b._id]))
    if found2 is not None:
        del circuits[found2]

    print (circuits)

print(circuits)
print (strings)

lens = list(reversed(sorted(map(len, circuits))))
print (lens)

print(f"Part1: {lens[0] * lens[1] * lens[2]}")
