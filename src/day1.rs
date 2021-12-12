use crate::prelude::*;

fn parse(r: &str) -> Vec<usize> {
    r.lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn part_one(nums: Vec<usize>) -> usize {
    nums.windows(2).filter(|s| s[0] < s[1]).count()
}

fn part_two(nums: Vec<usize>) -> usize {
    nums.windows(4)
        .filter(|window| {
            let sa = window[..3].iter().sum::<usize>();
            let sb = window[1..4].iter().sum::<usize>();
            sa < sb
        })
        .count()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
