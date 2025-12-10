import sys

reds = []
greens = []


def fill_greens(a, b):
    ax, ay = a
    bx, by = b
    if ax == bx:
        fill_hor_greens(ay, by, ax)
    elif ay == by:
        fill_vert_greens(ax, bx, ay)
    else:
        raise AssertionError()


def fill_hor_greens(a, b, x):
    a, b = sorted([a, b])
    for y in range(a + 1, b):
        greens.append((x, y))


def fill_vert_greens(a, b, y):
    a, b = sorted([a, b])
    for x in range(a + 1, b):
        greens.append((x, y))


for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    x, y = map(int, line.split(","))
    if reds:
        prevx, prevy = reds[len(reds) - 1]
        fill_greens((prevx, prevy), (x, y))
    reds.append((x, y))

fill_greens(reds[len(reds) - 1], reds[0])


def area(a, b):
    ax, ay = a
    bx, by = b
    return (abs(ax - bx) + 1) * (abs(ay - by) + 1)


maxarea = -1
for i, a in enumerate(reds):
    for b in reds[i + 1:]:
        cur = area(a, b)
        if cur > maxarea:
            maxarea = cur

print(f"Part1: {maxarea}")


class Memoize:
    def __init__(self, f):
        self.f = f
        self.memo = {}

    def __call__(self, *args):
        if not args in self.memo:
            print(f"Computing: {args}")
            self.memo[args] = self.f(*args)
        # Warning: You may wish to do a deepcopy here if returning objects
        return self.memo[args]


@Memoize
def is_inside(x, y):
    if (x, y) in greens:
        # Actually is green
        return True
    if (x, y) in reds:
        # Well, reds are 'inside' as well
        return True
    # Go to the left edge
    greencounts = 0
    redcounts = 0
    _reds = list(filter(lambda r: r[1] == y and r[0] <= x, reds))
    maxred = max([*map(lambda r: r[0], _reds), 0])
    _greens = filter(lambda g: g[1] == y and g[0] <= x and g[0] > maxred, greens)

    redcounts = len(list(_reds))
    greencounts = len(list(_greens))
    print(f"Checking {x},{y}: r{redcounts} g{greencounts}")

    if redcounts == 1:
        # On a hor. line
        return True
    if redcounts == 2:
        # Right of a hor. line, not too right, right ?
        return greencounts == 0
    if greencounts == 1:
        # Simply inside
        return True
    print(f"Not good: reds: {redcounts}, greens: {greencounts}")
    return False


def area_inside(a, b):
    """
    Returns if the area between a and b is completely inside the shape
    """
    ax, ay = a
    bx, by = b
    ax, bx = sorted([ax, bx])
    ay, by = sorted([ay, by])
    print(f"Checking: {a} - {b}")

    # If any green is in there (except on the edge), we are not clean
    insidegreen = len(list(filter(lambda g: ax < g[0] < bx and ay < g[1] < by, greens)))
    print(f"{insidegreen} greens inside")

    return insidegreen == 0

    for i in range(ax, bx + 1):
        for j in range(ay, by + 1):
            if not is_inside(i, j):
                print(f"Not good: {i},{j}")
                return False
    return True


def print_floor():
    maxx = max(map(lambda x: x[0], reds))
    maxy = max(map(lambda x: x[1], reds))
    for y in range(maxy + 2):
        line = ""
        for x in range(maxx + 2):
            if (x, y) in reds:
                line += '#'
            elif (x, y) in greens:
                line += 'X'
            else:
                line += '.'
        print(line)


# print_floor()

maxarea = -1
for i, a in enumerate(reds):
    for b in reds[i + 1:]:
        if not area_inside(a, b):
            continue
        print(f"Valid: {a} - {b}")
        cur = area(a, b)
        if cur > maxarea:
            maxarea = cur

print(f"Part2: {maxarea}")
