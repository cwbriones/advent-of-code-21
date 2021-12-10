# Alternative implementation of permutations that doesn't recurse.
def permutations2(slice):
    counts = [0 for _ in slice]
    while advance(slice, counts):
        yield slice

def advance(slice, counts):
    if counts[0] == len(slice):
        return False
    for i in range(len(slice)-2, -1, -1):
        j = counts[i]
        if j + i < len(slice) - 1:
            if j > 0:
                # undo the previous swap
                swap(slice, i, i+j)
            swap(slice, i, i+j+1)
            counts[i] += 1
            return True
        elif i == 0:
            counts[i] += 1
            swap(slice, 0, len(slice)-1)
            return True
        swap(slice, i, i+j)
        counts[i] = 0

def swap(slice, i, j):
    slice[i], slice[j] = slice[j], slice[i]
