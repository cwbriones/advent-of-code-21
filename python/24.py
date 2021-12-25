import sys
import re
from collections import defaultdict

lines = [line.strip() for line in sys.stdin]

# Group all the digits
grouped = []
group = []
for line in lines:
    if line.startswith('inp') and group:
        grouped.append(group)
        group = []
    group.append(line)
grouped.append(group)

# Extract the parts that match the template
def extract(prog):
    template = [
        "inp w",
        "mul x 0",
        "add x z",
        "mod x 26",
        "div z DIV",
        "add x COND",
        "eql x w",
        "eql x 0",
        "mul y 0",
        "add y 25",
        "mul y x",
        "add y 1",
        "mul z y",
        "mul y 0",
        "add y w",
        "add y INC",
        "mul y x",
        "add z y",
    ]
    bindings = {}
    for (temp, ins) in zip(template, prog):
        if temp == ins:
            continue
        # try to perform binding
        temp_parts = temp.split(' ')
        ins_parts = ins.split(' ')
        for (t, i) in zip(temp.split(' '), ins.split(' ')):
            if t== i:
                continue
            if t in ("DIV", "COND", "INC"):
                bindings[t] = int(i)
    return (bindings['DIV'] == 1, bindings['COND'], bindings['INC'])

stack = []
for i, prog in enumerate(grouped):
    push, cond, inc = extract(prog)
    if push:
        stack.append((i, inc))
    else:
        j, last = stack.pop()

# sanity check
stack = []
inp = list(map(int, "11912814611156"))
for i, prog in enumerate(grouped):
    push, cond, inc = extract(prog)
    if push:
        stack.append((i, inc))
    else:
        j, last = stack.pop()
        if not inp[j] + last + cond == inp[i]:
            print(f'x inp[{j}] + {last + cond} == inc[{i}]')
            print(f'{inp[j] + last + cond} != {inc[i]}')
        else:
            print(f'✓ inp[{j}] + {last + cond} == inc[{i}]')
