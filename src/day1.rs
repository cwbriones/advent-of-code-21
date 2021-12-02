use crate::prelude::*;

fn parse(r: &str) -> Result<Vec<usize>> {
    r.lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
}

fn part_one(nums: &[usize]) -> usize {
    nums.windows(2).filter(|s| s[0] < s[1]).count()
}

fn part_two(nums: &[usize]) -> usize {
    nums.windows(4)
        .filter(|window| {
            let sa = window[..3].iter().sum::<usize>();
            let sb = window[1..4].iter().sum::<usize>();
            sa < sb
        })
        .count()
}

//
// Everything below this point can be moved into a template
//

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let input = parse(input)?;
    runner.part_one(|| part_one(&input));
    runner.part_two(|| part_two(&input));
    Ok(())
}
