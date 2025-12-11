import sys



devices = {}

for line in sys.stdin:
    line = line.strip()
    if not line: continue
    name, out = line.split(": ")
    devices[name] = set(out.split())

def paths(start, end):
    sum = 0
    for next in devices[start]:
        if next == end:
            sum += 1
        else:
            sum += paths(next, end)
    return sum

print(f"Part1: {paths('you', 'out')}")