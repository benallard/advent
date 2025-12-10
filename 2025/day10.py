import sys
from itertools import combinations

class Light:
    def __init__(self, str):
        self.lights = [False] * len(str)
        self.target = [x == '#' for x in str]
        #print(self.lights, self.target)

    def apply(self, change):
        for x in change:
            self.lights[x] = not self.lights[x]

    def done(self):
        if self.lights != self.target:
            print(self)
        return self.lights == self.target

    def reset(self):
        self.lights = [False] * len(self.target)

    def __str__(self):
        return ''.join(x and '#' or '.' for x in self.lights) + " -> " + ''.join(x and '#' or '.' for x in self.target)

class Button:
    def __init__(self, str):
        buttons = str.split(" ")
        #print(str, buttons)
        self.buttons = [set(map(int, x)) for x in map(lambda x: x[1:-1].split(","), buttons)]

    def __getitem__(self, item):
        return self.buttons[item]

    def __iter__(self):
        yield from range(len(self.buttons))

class Joltage:
    pass

class Machine:
    def __init__(self, str):
        endlight = str.index(']')
        self.lights = Light(str[1:endlight])
        startjoltage = str.index('{')
        self.buttons = Button(str[endlight + 2: startjoltage - 1])

    def done(self):
        return self.lights.done()

    def press(self, num):
        changes = self.buttons[num]
        self.lights.apply(changes)

    def presses(self, nums):
        for x in nums:
            self.press(x)

    def reset(self):
        self.lights.reset()

    def attempt(self, buttons):
        self.reset()
        self.presses(buttons)
        return self.done()

    def solve(self) -> int:
        """
        Returns the minimal amount of presses to switch the machine to its target state

        The point is: pressing a button twice is the same as not pushing it.
        Three time is the same as one.

        The real question is Which buttons should I push (once!)
        """
        if self.attempt([]):
            return 0

        indices = list(self.buttons)
        n = len(indices)

        for k in range(1, n + 1):
            for combo in combinations(indices, k):
                if self.attempt(list(combo)):
                    return k

        return n




m = Machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
m.press(0)
m.press(1)
m.press(2)
assert m.done()

m.reset()
m.press(1)
m.press(3)
assert m.done()
m.press(5)
m.press(5)
assert m.done()


m.reset()
m.press(0)
m.press(2)
m.press(3)
m.press(4)
m.press(5)
assert m.done()

m.reset()
m.press(4)
m.press(5)
assert m.done()


m.reset()
assert m.solve() == 2

m = Machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
assert m.solve() == 3

m = Machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
assert m.solve() == 2

machines = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue

    machines.append(Machine(line))

res = sum(m.solve() for m in machines)
print(f"Part1: {res}")