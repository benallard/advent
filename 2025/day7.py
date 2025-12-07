import sys

grid = []

for line in sys.stdin:
    line = line.strip()
    grid.append(list(x for x in line))
print(grid[0].index('S'))

rays = {grid[0].index('S'): 1}

numsplits = 0
for line in grid[1:]:
    newrays = {}
    for ray, weight in rays.items():
        if line[ray] == '.':
            newrays[ray] = newrays.get(ray, 0) + weight
        if line[ray] == '^':
            newrays[ray-1] = newrays.get(ray - 1, 0) + weight
            newrays[ray + 1] = newrays.get(ray + 1, 0) + weight
            numsplits += 1
    rays = newrays

print(len(rays))
print(numsplits)

print(f"Part2: {sum(rays.values())}")

