import sys
from collections import Counter

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

# Construct all possible candidates for the wire mapping
def build_candidates(seen):
    candidates = {
        c: set('abcdefg') for c in 'abcdefg'
    }
    for s in seen:
        temp = {c: set() for c in s}
        # Any matching mapping could be true.
        #
        # For each seen example, take the union of all
        # matching digit lengths.
        for d in digits:
            if len(d) == len(s):
                for c in s:
                    temp[c] |= set(d)
        # Across each example, all inferences must be consistent.
        # So we take the intersection.
        for c in temp:
            candidates[c] &= temp[c]
    return candidates

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

def solve(seen, available, mapping = None, indent = 0):
    """
    Recusively solve the mapping by running inferences.
    """
    if not available:
        return consistent(seen, mapping)

    if mapping is None:
        mapping = {}

    # indent_str = '  ' * indent
    # Find the wire with the smallest amount of candidates and try them all.
    wire = min(available, key=lambda w: len(available[w]))
    for candidates in available[wire]:
        for c in candidates:
            # print(f'{indent_str}fix {wire} -> {c}')
            new_mapping = dict(mapping)
            new_mapping[wire] = c
            new_remaining = {k: set(available[k]) for k in available if k != wire}
            run_inferences(new_remaining, new_mapping)
            # print(f'{indent_str}mapping: {new_mapping}')
            # print(f'{indent_str}remain: {new_remaining}')
            if (solution := solve(seen, new_remaining, new_mapping, indent+1)) is not None:
                return solution
            # print(f'{indent_str}X')

def run_inferences(remaining, mapping):
    """
    Keep running inferences until we hit an ambiguity
    """
    while True:
        k = next((k for k in remaining if len(remaining[k]) == 1), None)
        if k is None:
            return
        mapping[k] = remaining[k].pop()
        del remaining[k]
        for other in remaining.values():
            if mapping[k] in other:
                other.remove(mapping[k])

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
        mapping = build_candidates(examples)
        sol = solve(examples, mapping)
        ds = to_digits(output, sol)
        total += ds
    print(total)

problem = parse()
part_one(problem)
part_two(problem)
