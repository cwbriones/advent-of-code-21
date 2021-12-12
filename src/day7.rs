use crate::prelude::*;

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

fn parse(input: &str) -> Vec<usize> {
    parse_split(input, ',').unwrap()
}

fn part_one(mut input: Vec<usize>) -> usize {
    solve(&mut input, |x| x)
}

fn part_two(mut input: Vec<usize>) -> usize {
    solve(&mut input, |x| x * (x + 1) / 2)
}

fn solve<F>(input: &mut [usize], step_cost: F) -> usize
where
    F: Fn(usize) -> usize + Copy,
{
    let min = input.iter().min().cloned().unwrap();
    let max = input.iter().max().cloned().unwrap();
    (min..=max)
        .map(|t| {
            input
                .iter()
                .map(|&x| if x > t { x - t } else { t - x })
                .map(step_cost)
                .sum()
        })
        .min()
        .expect("nonempty")
}
