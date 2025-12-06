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
        if sum >= 4:
            return False
    return sum < 4


def countAccessible(grid):
    count = 0
    for j in range(len(grid)):
        for i in range(len(grid[j])):
            if isAccessible(grid, i, j):
                count += 1
    return count

def removeAccessible(grid):
    coords = []
    for j in range(len(grid)):
        for i in range(len(grid[j])):
            if isAccessible(grid, i, j):
                coords.append((i, j))
    for x, y in coords:
        grid[y][x] = False
    return len(coords)


grid = [[]]

for line in sys.stdin:
    line = line.strip()
    if not line:
        continue
    grid.append(list(map(lambda x: x == '@', line)))


print(countAccessible(grid))

totalRemoved = 0
accessible = removeAccessible(grid)
while accessible != 0:
    totalRemoved += accessible
    accessible = removeAccessible(grid)

print(totalRemoved)


