import sys

grid = []

for line in sys.stdin:
    line = line.strip()
    grid.append(list(x for x in line))
print(grid[0].index('S'))

rays = [grid[0].index('S')]

numsplits = 0
for line in grid[1:]:
    newrays = list()
    for ray in rays:
        if line[ray] == '.':
            newrays.append(ray)
        if line[ray] == '^':
            newrays.append(ray-1)
            newrays.append(ray+1)
            numsplits += 1
    rays = newrays

print(len(rays))
print(numsplits)

