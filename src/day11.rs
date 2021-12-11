use crate::prelude::*;

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let input = parse(input);
    runner.part_one(|| part_one(input.clone()));
    runner.part_two(|| part_two(input));
    Ok(())
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| char::to_digit(c, 10)).collect())
        .collect()
}

fn part_one(mut grid: Vec<Vec<u32>>) -> usize {
    (0..100).map(|_| tick(&mut grid)).sum()
}

fn part_two(mut grid: Vec<Vec<u32>>) -> usize {
    let total = grid.iter().map(|r| r.len()).sum::<usize>();
    let mut i = 1;
    while total != tick(&mut grid) {
        i += 1
    }
    i
}

fn tick(grid: &mut [Vec<u32>]) -> usize {
    let mut needs_flash = Vec::new();
    for (j, row) in grid.iter_mut().enumerate() {
        for (i, n) in row.iter_mut().enumerate() {
            *n += 1;
            if *n > 9 {
                needs_flash.push((i, j));
            }
        }
    }
    let mut flashed = HashSet::default();
    while let Some(p) = needs_flash.pop() {
        if flashed.contains(&p) {
            continue;
        }
        flashed.insert(p);
        for (i2, j2) in neighbors(grid, p) {
            grid[j2][i2] += 1;
            if grid[j2][i2] > 9 {
                needs_flash.push((i2, j2));
            }
        }
    }
    for (i, j) in &flashed {
        grid[*j][*i] = 0
    }
    flashed.len()
}

fn neighbors(grid: &[Vec<u32>], p: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let i = p.0 as i32;
    let j = p.1 as i32;
    let grid_len = grid.len();
    let row_len = grid[0].len();

    (-1i32..=1)
        .flat_map(move |dx| (-1i32..=1).map(move |dy| (i + dx, j + dy)))
        .filter(move |(x, y)| {
            *x >= 0
                && *y >= 0
                && (*x != i || *y != j)
                && *x < row_len as i32
                && *y < grid_len as i32
        })
        .map(|(x, y)| (x as usize, y as usize))
}
