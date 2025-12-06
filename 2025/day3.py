import sys

def posMax(bank, left, right):
    print(f"Investigating {bank[left:len(bank) - right]} {left} {right}")
    return max(range(left, len(bank) - right), key=bank.__getitem__)

def maxJoltage(bank):
    posTens = posMax(bank, 0, 1)
    posUnits = posMax(bank, posTens + 1, 0)
    print(f"Got: {bank[posTens]}{bank[posUnits]}")
    return bank[posTens] * 10 + bank[posUnits]

def maxDynJoltage(bank, amount):
    sum = 0
    left = -1
    for i in range(amount):
        left = posMax(bank, left+1, amount - i - 1)
        sum += bank[left] * 10**(amount - i - 1)
    print(f"Dyn: {sum}")
    return sum


assert maxJoltage([9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]) == 98
assert maxJoltage([8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]) == 89
assert maxJoltage([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]) == 78
assert maxJoltage([8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]) == 92

assert maxDynJoltage([9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 2) == 98
assert maxDynJoltage([8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 2) == 89
assert maxDynJoltage([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 2) == 78
assert maxDynJoltage([8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 2) == 92

assert maxDynJoltage([9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 12) == 987654321111
assert maxDynJoltage([8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 12) == 811111111119
assert maxDynJoltage([2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 12) == 434234234278
assert maxDynJoltage([8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 12) == 888911112111



sum = 0
sum12 = 0
for bank in sys.stdin:
    bank = bank.strip()
    if not bank:
        continue
    banks = list(map(int, bank))
    sum += maxJoltage(banks)
    sum12 += maxDynJoltage(banks, 12)

print(sum)
print(sum12)