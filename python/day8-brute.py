import sys

def permutations(slice):
    yield from permute(slice, [0 for _ in slice], 0)

def permute(slice, counts, depth):
    if len(slice) - depth <= 1:
        yield slice
        return
    while counts[depth] < len(slice) - depth:
        i = counts[depth]
        slice[depth], slice[depth + i] = slice[depth + i], slice[depth]
        yield from permute(slice, counts, depth+1)
        slice[depth], slice[depth + i] = slice[depth + i], slice[depth]
        counts[depth] += 1
    counts[depth] = 0

digits = {
    'abcefg': '0',
    'cf': '1',
    'acdeg': '2',
    'acdfg': '3',
    'bcdf': '4',
    'abdfg': '5',
    'abdefg': '6',
    'acf': '7',
    'abcdefg': '8',
    'abcdfg': '9',
}

def parse():
    problem = []
    for line in sys.stdin:
        left, right = line.strip().split(" | ")
        examples = [s for s in left.split(" ")]
        output = [s for s in right.split(" ")]
        problem.append((examples, output))
    return problem

def part_one(problem):
    total = 0
    for _, output in problem:
        for o in output:
            if len(o) in [2,3,4,7]:
                total += 1
    print(total)

def part_two(problem):
    total = 0
    for examples, output in problem:
        mapping = find_mapping(examples)
        ds = to_digits(output, mapping)
        total += ds
    print(total)

def find_mapping(examples):
    for perm in permutations(list("abcdefg")):
        mapping = {k: v for (k, v) in zip("abcdefg", perm)}
        if consistent(examples, mapping):
            return mapping
    return None

def to_digits(seen, mapping = None):
    ds = []
    for s in seen:
        mapped = ''.join(sorted(mapping[c] for c in s))
        ds.append(digits[mapped])
    return int(''.join(ds))

def consistent(seen, mapping):
    for s in seen:
        mapped = ''.join(sorted(mapping[c] for c in s))
        if mapped not in digits:
            return None
    return mapping

problem = parse()
part_one(problem)
part_two(problem)
