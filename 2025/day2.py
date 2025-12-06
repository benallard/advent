# Sample input:
# 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124


# Invalid numbers are ones that repeat twice, like 55, 6464, or 123123
def isInvalid(number):
    n = len(number)
    if n % 2 != 0:
        return False
    half = n // 2
    return number[:half] == number[half:]

assert isInvalid("55")
assert isInvalid("6464")
assert isInvalid("123123")
assert not isInvalid("101")

ranges = input().strip().split(",")

sum = 0

for r in ranges:
    start, end = map(int, r.split("-"))
    for number in range(start, end + 1):
        strnum = str(number)
        if isInvalid(strnum):
            sum += number

print(sum)