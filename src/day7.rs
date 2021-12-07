use crate::prelude::*;

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let mut input = parse(input.trim());
    runner.part_one(|| part_one(&mut input));
    runner.part_two(|| part_two(&mut input));
    Ok(())
}

fn parse(input: &str) -> Vec<usize> {
    parse_split(input, ',').unwrap()
}

fn part_one(input: &mut [usize]) -> usize {
    solve(input, |x| x)
}

fn part_two(input: &mut [usize]) -> usize {
    solve(input, |x| x * (x + 1) / 2)
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
