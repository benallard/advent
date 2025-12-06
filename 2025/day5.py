import sys

importfresh = True

def isFresh(article):
    for start, end in fresh:
        if start > article:
            continue
        if end < article:
            continue
        return True
    return False

fresh = []
max = 0

countFresh = 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        importfresh = False
        continue

    if importfresh:
        start, end = map(int, line.split("-"))
        fresh.append((start, end))
        if end > max:
            max = end
    else:
        article = int(line)
        if isFresh(article):
            print(f"{article} is fresh")
            countFresh += 1
        else:
            print(f"{article} is spoiled")

print(f"{countFresh} fresh articles")

allFresh = 0
for article in range(max + 1):
    if isFresh(article):
        allFresh += 1

print(f"{allFresh} posible fresh Articles")
