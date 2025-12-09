import sys

reds = []

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    x, y = map(int, line.split(","))
    reds.append((x, y))

def area(a, b):
    ax, ay = a
    bx, by = b
    return (abs(ax - bx) + 1) * (abs(ay - by) + 1)

maxarea = -1
for i, a in enumerate(reds):
    for b in reds[i+1:]:
        cur = area(a, b)
        if cur > maxarea:
            maxarea = cur

print(maxarea)

