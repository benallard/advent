import sys


def isFresh(article):
    for start, end in fresh:
        if start > article:
            continue
        if end < article:
            continue
        return True
    return False


def countAllFresh():
    count = 0
    sortedFresh = sorted(fresh, key=lambda f: f[0])
    for i, (start, end) in enumerate(sortedFresh[:-1]):
        nextStart = sortedFresh[i + 1][0]
        if nextStart < end:
            # Only add till next Start, next range wil care of the rest
            count += nextStart - start
            print(f"Adding start: {start} - {nextStart}: {count}")
        else:
            # Disjointed with next one: add all, move along
            count += end - start + 1
            print(f"Adding all: {start} - {end}: {count}")
            continue
        # Now we should care of the fact that the next range(s!) might end before our own
        lastEnd = end
        for nextStart, nextEnd in sortedFresh[i + 1:]:
            if nextEnd > end:
                # Too far
                break
            if nextEnd < lastEnd:
                lastEnd = nextEnd
        if end != lastEnd:
            print(f"Adding end: {lastEnd} - {end}: {count}")
            count += end - lastEnd
    # Dont forget the last range !
    lastStart, lastEnd = sortedFresh[-1]
    count += lastEnd - lastStart + 1
    print(f"Adding last {lastStart} - {lastEnd}: {count}")
    return count


importfresh = True

fresh = []

countFresh = 0
for line in sys.stdin:
    line = line.strip()
    if not line:
        importfresh = False
        continue

    if importfresh:
        start, end = map(int, line.split("-"))
        fresh.append((start, end))
    else:
        article = int(line)
        if isFresh(article):
            print(f"{article} is fresh")
            countFresh += 1
        else:
            print(f"{article} is spoiled")

print(f"{countFresh} fresh articles")

allFresh = countAllFresh()
print(f"{allFresh} posible fresh Articles")
