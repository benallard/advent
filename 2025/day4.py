import sys


def isOccupied(map, x, y):
    if y < 0 or y >= len(map):
        return False
    if x < 0 or x >= len(map[y]):
        return False
    print(f"{x}:{y}")
    return map[y][x]


def isAccessible(map, x, y):
    if not isOccupied(map, x, y):
        return False
    sum = 0
    for dx, dy in zip((-1, -1, -1, 0, 0, 1, 1, 1),(-1, 0, 1, -1, 1, -1, 0, 1)):
        sum += int(isOccupied(map, x + dx, y + dy))
        print(f"{x}:{y}: {x+dx}:{y+dy}: {sum}")
    return sum < 4


grid = [[]]

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    grid.append(list(map(lambda x: x == '@', line)))

count = 0
for j in range(len(grid)):
    for i in range(len(grid[j])):
        if isAccessible(grid, i, j):
            count += 1

print(count)
