import sys

def maxJoltage(bank):
    posTens = max(range(len(bank) - 1), key=bank.__getitem__)
    posUnits = max(range(posTens + 1, len(bank)), key=bank.__getitem__)
    print(f"Got: {bank[posTens]}{bank[posUnits]}")
    return bank[posTens] * 10 + bank[posUnits]


assert maxJoltage([9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]) == 98
assert maxJoltage([8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]) == 89
assert maxJoltage([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]) == 78
assert maxJoltage([8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]) == 92



sum = 0
for bank in sys.stdin:
    bank = bank.strip()
    if not bank:
        continue
    banks = list(map(int, bank))
    sum += maxJoltage(banks)

print(sum)