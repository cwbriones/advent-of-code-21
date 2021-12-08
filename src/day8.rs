use once_cell::sync::Lazy;

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

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let input = parse(input.trim())?;
    runner.part_one(|| part_one(input.clone()));
    runner.part_two(|| part_two(input));
    Ok(())
}

#[derive(Debug, Clone)]
struct Case {
    examples: Vec<String>,
    output: Vec<String>,
}

fn parse(input: &str) -> Result<Vec<Case>> {
    input.lines().map(|l| parse_case(l.trim())).collect()
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
    let mut total = 0;
    for case in &cases {
        let candidates = build_candidates(&case.examples);
        let sol = solve(&case.examples, candidates, HashMap::default()).expect("no solution found");
        total += to_digits(&case.output, &sol);
    }
    total
}

fn to_digits(seen: &[String], mapping: &HashMap<char, char>) -> usize {
    let digits = seen.iter().flat_map(|s| {
        let mut mapped = s
            .chars()
            .filter_map(|c| mapping.get(&c))
            .cloned()
            .collect::<Vec<char>>();
        mapped.sort_unstable();
        let mapped = mapped.iter().collect::<String>();
        DIGITS.get(mapped.as_str())
    });
    digits.fold(0, |acc, elem| acc * 10 + elem)
}

fn consistent(seen: &[String], mapping: &HashMap<char, char>) -> bool {
    seen.iter().all(|s| {
        let mut mapped = s
            .chars()
            .filter_map(|c| mapping.get(&c))
            .cloned()
            .collect::<Vec<char>>();
        mapped.sort_unstable();
        let mapped = mapped.iter().collect::<String>();
        DIGITS.contains_key(mapped.as_str())
    })
}

fn solve(
    seen: &[String],
    available: HashMap<char, HashSet<char>>,
    mapping: HashMap<char, char>,
) -> Option<HashMap<char, char>> {
    if available.is_empty() {
        if consistent(seen, &mapping) {
            return Some(mapping);
        }
        return None;
    }

    let (wire, candidates) = available
        .iter()
        .min_by_key(|&(_, v)| v.len())
        .expect("nonempty");

    for c in candidates {
        let mut new_mapping = mapping.clone();
        new_mapping.insert(*wire, *c);
        let mut new_remaining = available
            .iter()
            .filter(|(&k, _)| k != *wire)
            .map(|(&k, v)| (k, v.clone()))
            .collect::<HashMap<char, HashSet<char>>>();
        run_inferences(&mut new_remaining, &mut new_mapping);

        if let Some(sol) = solve(seen, new_remaining, new_mapping) {
            return Some(sol);
        }
    }
    None
}

fn run_inferences(remaining: &mut HashMap<char, HashSet<char>>, mapping: &mut HashMap<char, char>) {
    loop {
        let v = {
            let (k, vs) = match remaining.iter_mut().find(|(_, v)| v.len() == 1) {
                Some(found) => found,
                None => return,
            };
            let v = vs.drain().next().expect("nonempty");
            mapping.insert(*k, v);
            v
        };
        for other in remaining.values_mut() {
            other.remove(&v);
        }
    }
}

fn build_candidates(seen: &[String]) -> HashMap<char, HashSet<char>> {
    let mut candidates = HashMap::default();
    let all = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    for c in all {
        let set = all.iter().cloned().collect::<HashSet<_>>();
        candidates.insert(c, set);
    }
    let mut temp: HashMap<char, HashSet<char>> = HashMap::default();
    for s in seen {
        temp.clear();

        for d in DIGITS.keys() {
            if d.len() != s.len() {
                continue;
            }
            for c in s.chars() {
                let entry = temp.entry(c).or_insert_with(HashSet::default);
                entry.extend(d.chars());
            }
        }

        for (c, tpossible) in &temp {
            if let Some(possible) = candidates.get_mut(c) {
                possible.retain(|p| tpossible.contains(p));
            }
        }
    }
    candidates
}
