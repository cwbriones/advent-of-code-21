use crate::prelude::*;

#[derive(Clone)]
enum Step {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse(input: &str) -> Vec<Step> {
    input
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
        .collect()
}

fn part_one(steps: Vec<Step>) -> usize {
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

fn part_two(steps: Vec<Step>) -> usize {
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

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
