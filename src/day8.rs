use once_cell::sync::Lazy;

use crate::iter::search_permutations;
use crate::prelude::*;

static DIGITS: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    let mut digits = HashMap::default();
    digits.insert("abcefg", 0);
    digits.insert("cf", 1);
    digits.insert("acdeg", 2);
    digits.insert("acdfg", 3);
    digits.insert("bcdf", 4);
    digits.insert("abdfg", 5);
    digits.insert("abdefg", 6);
    digits.insert("acf", 7);
    digits.insert("abcdefg", 8);
    digits.insert("abcdfg", 9);
    digits
});

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[derive(Debug, Clone)]
struct Case {
    examples: Vec<String>,
    output: Vec<String>,
}

fn parse(input: &str) -> Vec<Case> {
    input
        .lines()
        .map(|l| parse_case(l.trim()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn parse_case(line: &str) -> Result<Case> {
    let mut parts = line.split(" | ");

    let examples = parts
        .next()
        .expect("invalid")
        .split(' ')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    let output = parts
        .next()
        .expect("invalid")
        .split(' ')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    Ok(Case { examples, output })
}

fn part_one(cases: Vec<Case>) -> usize {
    cases
        .iter()
        .flat_map(|c| c.output.iter())
        .map(|s| s.len())
        .filter(|&l| l == 2 || l == 3 || l == 4 || l == 7)
        .sum()
}

fn part_two(cases: Vec<Case>) -> usize {
    cases
        .iter()
        .map(|case| {
            let sol = solve(&case.examples).expect("no solution found");
            to_digits(&case.output, &sol)
        })
        .sum()
}

fn to_digits(seen: &[String], mapping: &[char]) -> usize {
    let digits = seen.iter().flat_map(|s| {
        let mut mapped = s
            .chars()
            .map(|c| mapping[(c as usize) - 'a' as usize])
            .collect::<Vec<char>>();
        mapped.sort_unstable();
        let mapped = mapped.iter().collect::<String>();
        DIGITS.get(mapped.as_str())
    });
    digits.fold(0, |acc, elem| acc * 10 + elem)
}

fn consistent(seen: &[String], mapping: &[char]) -> bool {
    seen.iter().all(|s| {
        let mut mapped = s
            .chars()
            .map(|c| mapping[(c as usize) - 'a' as usize])
            .collect::<Vec<char>>();
        mapped.sort_unstable();
        let mapped = mapped.iter().collect::<String>();
        DIGITS.contains_key(mapped.as_str())
    })
}

fn solve(seen: &[String]) -> Option<Vec<char>> {
    let mut alphabet = "abcdefg".chars().collect::<Vec<_>>();
    if search_permutations(&mut alphabet, |perm| consistent(seen, perm)) {
        Some(alphabet)
    } else {
        None
    }
}
