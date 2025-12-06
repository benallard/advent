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

problems = []
sum = 0
for line in sys.stdin:
    line = line.strip()
    if not problems:
        problems = [[int(x)] for x in line.split()]
    else:
        for i, part in enumerate(line.split()):
            if part in ('+', '*'):
                sum += evaluate(problems[i], part)
            else:
                problems[i].append(int(part))

print(sum)