import sys

def evaluate(nums, op):
    print(f"Evaluating {op}: {nums}")
    if op == '+':
        res = 0
    elif op == '*':
        res = 1
    for num in nums:
        if op == '+':
            res += num
        elif op == '*':
            res *= num
    print(f"Res: {res}")
    return res

def processNumbers(input):
    nums = []
    op = None
    sum = 0
    for num in input:
        if not num.strip():
            sum += evaluate(nums, op)
            nums = []
            op = None
            continue
        print (f"'{num}'")
        if num[-1] in ('+', '*'):
            op = num[-1]
            num = num[:-1]
        nums.append(int(num.strip()))
    sum += evaluate(nums, op)
    return sum


problems = []
numbers = []
sum = 0
for line in sys.stdin:
    line = line.rstrip()
    if not problems:
        problems = [[int(x)] for x in line.split()]
    else:
        for i, part in enumerate(line.split()):
            if part in ('+', '*'):
                sum += evaluate(problems[i], part)
            else:
                problems[i].append(int(part))

    # Part 2: We rotate the input anti-CW, one quarter, so we can read the numbers easily
    if not numbers:
        numbers = [char for char in line]
        print(numbers)
    else:
        for i, char in enumerate(line):
            if i >= len(numbers):
                numbers.append(' ' * (len(numbers[0]) - 1)) # Take 0 as length reference
            numbers[i] += char


print(sum)


print(numbers)
print(f"Part 2: {processNumbers(numbers)}")
