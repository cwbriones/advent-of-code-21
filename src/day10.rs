use crate::prelude::*;

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.trim()).collect()
}

fn part_one(lines: Vec<&str>) -> usize {
    let points = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)];
    lines
        .iter()
        .filter_map(|line| check(line).err())
        .filter_map(|c| points.iter().find(|(k, _)| *k == c).map(|(_, v)| v))
        .sum::<usize>()
}

fn part_two(lines: Vec<&str>) -> usize {
    let points = [(')', 1), (']', 2), ('}', 3), ('>', 4)];
    let mut scores = lines
        .iter()
        .filter_map(|line| check(line).ok())
        .map(|line| {
            line.iter()
                .flat_map(|c| points.iter().find(|(k, _)| *k == *c).map(|(_, v)| v))
                .fold(0, |acc, s| acc * 5 + s)
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn check(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    let pairs = [('(', ')'), ('[', ']'), ('<', '>'), ('{', '}')];
    for c in line.chars() {
        if let '(' | '[' | '<' | '{' = c {
            stack.push(c);
            continue;
        }
        let last = stack.last().expect("nonempty");
        match pairs.iter().find(|(k, _)| *k == *last) {
            Some((_, expected)) if c == *expected => {
                stack.pop();
            }
            Some(_) => return Err(c),
            None => panic!("bad input"),
        }
    }
    let complete = stack
        .iter()
        .rev()
        .flat_map(|&k| pairs.iter().find(|(k2, _)| k == *k2))
        .map(|(_, v)| *v)
        .collect::<Vec<_>>();
    Ok(complete)
}
