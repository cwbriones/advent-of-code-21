import sys

def step(grid):
    needs_flash = []
    for (j, row) in enumerate(grid):
        for (i, _) in enumerate(row):
            row[i] += 1
            if row[i] > 9:
                needs_flash.append((i, j))
    flashed = set()
    while needs_flash:
        p = needs_flash.pop()
        if p in flashed:
            continue
        flashed.add(p)
        (i, j) = p
        for n in neighbors(grid, i, j):
            i2, j2 = n
            grid[j2][i2] += 1
            if grid[j2][i2] > 9:
                needs_flash.append(n)
    for (i, j) in flashed:
        grid[j][i] = 0

    return len(flashed)

def neighbors(grid, i, j):
    delta = [-1, 0, 1]
    for dx in delta:
        for dy in delta:
            if dx == 0 and dy == 0:
                continue
            if j + dy == len(grid) or j + dy < 0:
                continue
            if i + dx == len(grid[0]) or i + dx < 0:
                continue
            yield (i + dx, j + dy)

def part_one(grid):
    flashes = 0
    for _ in range(100):
        flashes += step(grid)
    print(flashes)

def part_two(grid):
    total = sum(len(r) for r in grid)
    i = 0
    flashes = 0
    while flashes != total:
        i += 1
        flashes = step(grid)
    print(i)

grid = []
for line in sys.stdin.readlines():
    line = line.strip()
    grid.append([int(n) for n in line])

part_one([list(r) for r in grid])
part_two(grid)
