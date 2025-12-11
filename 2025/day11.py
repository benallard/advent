import sys

devices = {}

for line in sys.stdin:
    line = line.strip()
    if not line: continue
    name, out = line.split(": ")
    devices[name] = set(out.split())


def numpaths(start, end):
    sum = 0
    for next in devices[start]:
        if next == end:
            sum += 1
        else:
            sum += numpaths(next, end)
    return sum


if 'you' in devices:
    print(f"Part1: {numpaths('you', 'out')}")


def paths(start, end, prefix=[]):
    for next in devices[start]:
        if next == end:
            yield prefix + [next]
        else:
            yield from paths(next, end, prefix + [next])


sum = 0
for path in paths("svr", "out"):
    if "fft" in path and "dac" in path:
        sum += 1

print(f"Part2: {sum}")
