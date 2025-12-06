import sys


def isFresh(article):
    for start, end in fresh:
        if start > article:
            continue
        if end < article:
            continue
        return True
    return False


def countAllFresh(fresh):
    count = 0
    sortedFresh = sorted(fresh, key=lambda f: f[0])
    for i, (start, end) in enumerate(sortedFresh[:-1]):
        nextStart = sortedFresh[i + 1][0]
        if nextStart <= end:
            # Only add till next Start, next range wil care of the rest
            count += nextStart - start
            print(f"Adding start: [{start} - {nextStart}[: {count} (+{nextStart - start})")
        else:
            # Disjointed with next one: add all, move along
            count += end - start + 1
            print(f"Adding all: [{start} - {end}]: {count} (+{end - start + 1})")
            continue
        # Now we should care of the fact that the next range(s!) might end before our own
        # So we might have an own rest at the end of our range
        lastEnd = start
        for nextStart, nextEnd in sortedFresh[i + 1:]:
            if nextEnd > end:
                # Too far
                break
            if nextEnd > lastEnd:
                lastEnd = nextEnd
        if start != lastEnd:
            # Now, only count to the next start after lastEnd
            nextStarts = list(filter(lambda x: x > lastEnd, map(lambda f: f[0], sortedFresh)))
            if nextStarts:
                count += nextStarts[0] - lastEnd - 1
                print(f"Adding end: ]{lastEnd} - {nextStarts[0]}[: {count} (+{nextStarts[0] - lastEnd - 1})")
            else:
                count += end - lastEnd
                print(f"Adding end: ]{lastEnd} - {end}]: {count} (+{end - lastEnd})")
    # Dont forget the last range !
    lastStart, lastEnd = sortedFresh[-1]
    count += lastEnd - lastStart + 1
    print(f"Adding last [{lastStart} - {lastEnd}]: {count} (+{lastEnd - lastStart + 1})")
    print(f"Done: {count}")
    return count

assert countAllFresh([(3,5), (10, 14), (16, 20), (12, 18)]) == 14
assert countAllFresh([(5,15), (5,8), (8, 10)]) == 11
assert countAllFresh([(5, 15), (6,8), (12, 20)]) == 16

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

allFresh = countAllFresh(fresh)
print(f"{allFresh} posible fresh Articles")
# 358520522905153 too high
# 413102448573199 too high