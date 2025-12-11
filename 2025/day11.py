import sys

devices = {}

for line in sys.stdin:
    line = line.strip()
    if not line: continue
    name, out = line.split(": ")
    devices[name] = set(out.split())

# Make sure the out node is there
if 'out' not in devices:
    devices['out'] = {}

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

# Let's be smarter and reduce the search space:
#start - fft - dac - end
path1 = numpaths("svr", "fft")
print(path1)
path1 *= numpaths("fft", "dac")
print(path1)
path1 *= numpaths("dac", "out")
print(path1)

# start - dac - fft - end
path2 = numpaths("svr", "dac")
print(path2)
path2 *= numpaths("dac", "fft")
print(path2)
path2 *= numpaths("fft", "out")
print(path2)

print(f"Part2: {path1 + path2}")
