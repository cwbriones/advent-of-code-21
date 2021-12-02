use crate::prelude::*;

enum Step {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse(input: &str) -> Result<Vec<Step>> {
    let steps = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let kind = parts.next().unwrap();
            let amt = parts.next().and_then(|p| p.parse::<usize>().ok()).unwrap();

            match kind {
                "forward" => Step::Forward(amt),
                "up" => Step::Up(amt),
                "down" => Step::Down(amt),
                m => panic!("unknown step {}", m),
            }
        })
        .collect::<Vec<_>>();
    Ok(steps)
}

fn part_one(steps: &[Step]) -> usize {
    let mut pos = 0;
    let mut depth = 0;

    for s in steps {
        match s {
            Step::Up(x) => depth -= x,
            Step::Down(x) => depth += x,
            Step::Forward(x) => pos += x,
        }
    }
    pos * depth
}

fn part_two(steps: &[Step]) -> usize {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for s in steps {
        match s {
            Step::Up(x) => aim -= x,
            Step::Down(x) => aim += x,
            Step::Forward(x) => {
                pos += x;
                depth += aim * x;
            }
        }
    }
    pos * depth
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
