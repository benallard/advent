import sys
from itertools import combinations

class Light:
    def __init__(self, str):
        self.target = [x == '#' for x in str]
        self.reset()

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

    def __len__(self):
        return len(self.buttons)

class Joltage:
    def __init__(self, str):
        self.target = list(int(x) for x in str.split(","))
        self.reset()

    def apply(self, change):
        #print(change)
        for x in change:
            self.joltage[x] = self.joltage[x] + 1
        #print(self)

    def reset(self):
        self.joltage = [0] * len(self.target)
        #print("-")

    def done(self):
        if self.joltage != self.target:
            print(self)
        return self.joltage == self.target

    def burst(self):
        for i, x in enumerate(self.joltage):
            if x > self.target[i]:
                return True
        return False

    def __str__(self):
        return '{'+','.join(map(str,self.joltage))+'} -> {' +','.join(map(str,self.target))+'}'

class Machine:
    def __init__(self, str):
        endlight = str.index(']')
        self.lights = Light(str[1:endlight])
        startjoltage = str.index('{')
        self.buttons = Button(str[endlight + 2: startjoltage - 1])
        self.joltage = Joltage(str[startjoltage+1:-1])

    def done(self, joltage=False):
        if joltage:
            return self.joltage.done()
        else:
            return self.lights.done()

    def press(self, num):
        changes = self.buttons[num]
        self.lights.apply(changes)
        self.joltage.apply(changes)

    def presses(self, nums):
        for x in nums:
            self.press(x)

    def reset(self):
        self.lights.reset()
        self.joltage.reset()

    def attempt(self, buttons):
        self.reset()
        self.presses(buttons)
        return self.done()


    def burst(self, presses):
        self.attempt2(presses)
        return self.joltage.burst()

    def attempt2(self, presses):
        self.reset()
        for i, num in enumerate(presses):
            for _ in range(num):
                self.press(i)
        return self.done(True)

    def solve1(self) -> int:
        """
        Returns the minimal amount of presses to switch the machine to its target state

        The point is: pressing a button twice is the same as not pushing it.
        Three time is the same as one.

        The real question is Which buttons should I push (once!)
        """
        if self.attempt([]):
            return 0

        indices = list(self.buttons)
        n = len(self.buttons)

        for k in range(1, n + 1):
            for combo in combinations(indices, k):
                if self.attempt(list(combo)):
                    return k

        return n

    def solve2(self):
        """
        Now, it's not about which buttons, it about how many time which one.
        So, instead of working with a list of buttons, let's work with a list of presses-amount.
        From [0, 0, ... , 0] to [10, 10, ... , 10]  (or whatever).
        The good part is, we must stop increasing a position when we burst over
        So our possibility tree spans from [10, 0, ..., 0] on the far left to [10,10] on the far right. the origin in [0,0]

        """
        presses = [0] * len(self.buttons)
        if self.attempt2(presses):
            return 0
        for value in _bfs(self, presses):
            print(sum(value))
            if self.attempt2(value):
                return sum(value)

from collections import deque

def _bfs(m, start):
    q = deque()
    # enqueue immediate neighbours of start
    for i in range(len(start)):
        cand = start.copy()
        cand[i] += 1
        if not m.burst(cand):
            q.append(cand)
            yield cand
    # breadth-first traversal
    while q:
        cur = q.popleft()
        for i in range(len(cur)):
            nxt = cur.copy()
            nxt[i] += 1
            if not m.burst(nxt):
                q.append(nxt)
                yield nxt




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
assert m.solve1() == 2

assert m.attempt2([1,3,0,3,1,2])
assert m.solve2() == 10

m = Machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
assert m.solve1() == 3

assert m.attempt2([2,5,0,5])
assert m.solve2() == 12

m = Machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
assert m.solve1() == 2
assert m.attempt2([5,0,5,1])
assert m.solve2() == 11

machines = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue

    machines.append(Machine(line))

res = sum(m.solve1() for m in machines)
print(f"Part1: {res}")


