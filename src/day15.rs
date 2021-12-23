use crate::prelude::*;
use crate::search::SearchQueue;

fn parse(r: &str) -> Vec<Vec<usize>> {
    r.lines()
        .map(|line| {
            line.chars()
                .map(|c| (c as usize) - ('0' as usize))
                .collect()
        })
        .collect()
}

fn part_one(nums: Vec<Vec<usize>>) -> usize {
    search(nums, 1)
}

fn part_two(nums: Vec<Vec<usize>>) -> usize {
    search(nums, 5)
}

fn search(nums: Vec<Vec<usize>>, tile_size: isize) -> usize {
    let mut fringe = SearchQueue::new();
    fringe.push(0, (0, 0));
    let height = (nums.len() as isize) * tile_size;
    let width = (nums[0].len() as isize) * tile_size;
    let target = (height - 1, width - 1);

    let mut visited = HashSet::default();
    while let Some((cost, p)) = fringe.pop() {
        if visited.contains(&p) {
            continue;
        }
        visited.insert(p);

        let (y, x) = p;
        if p == target {
            return cost;
        }
        let neighbors = [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];
        neighbors
            .iter()
            .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < height && *x < width)
            .for_each(|&p| {
                fringe.push(cost + cost_to(&nums, p.1, p.0), p);
            });
    }
    panic!("no path found");
}

fn cost_to(nums: &[Vec<usize>], y: isize, x: isize) -> usize {
    let tile_height = nums.len() as isize;
    let tile_width = nums[0].len() as isize;

    let tile_y = y / tile_height;
    let tile_x = x / tile_width;

    let yy = (y % tile_height) as usize;
    let xx = (x % tile_width) as usize;

    let cost = nums[yy as usize][xx as usize] as isize + tile_x + tile_y;
    if (tile_y > 0 || tile_x > 0) && cost > 9 {
        (cost % 9) as usize
    } else {
        cost as usize
    }
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
