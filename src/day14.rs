use crate::prelude::*;

#[derive(Debug, Clone)]
struct Input<'a> {
    start: &'a str,
    rules: HashMap<&'a str, [String; 2]>,
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let start = lines.next().unwrap().trim();

    let rules = lines
        .skip(1)
        .map(|line| {
            let mut split = line.split(" -> ");
            let from = split.next().unwrap();
            let to = split.flat_map(|s| s.chars()).next().unwrap();

            let mut fromchars = from.chars();
            let mut out1 = String::new();
            out1.push(fromchars.next().unwrap());
            out1.push(to);
            let mut out2 = String::new();
            out2.push(to);
            out2.push(fromchars.next().unwrap());

            (from, [out1, out2])
        })
        .collect();

    Input { start, rules }
}

fn part_one(input: Input) -> usize {
    solve(input, 10)
}

fn part_two(input: Input) -> usize {
    solve(input, 40)
}

fn solve(input: Input, iterations: usize) -> usize {
    let mut counts = HashMap::default();
    for w in windows(input.start) {
        *counts.entry(w.to_owned()).or_insert(0) += 1;
    }
    let mut next_counts = HashMap::default();
    for _ in 0..iterations {
        for (k, c) in counts.iter() {
            for out in input.rules.get(k.as_str()).iter().flat_map(|s| s.iter()) {
                if let Some(count) = next_counts.get_mut(out) {
                    *count += c;
                } else {
                    next_counts.insert(out.clone(), *c);
                }
            }
        }
        std::mem::swap(&mut counts, &mut next_counts);
        next_counts.clear()
    }

    // Convert back to counts of individual characters
    let mut char_counts = to_char_counts(counts, input.start)
        .into_iter()
        .collect::<Vec<_>>();
    char_counts.sort_by_key(|p| std::cmp::Reverse(p.1));
    (char_counts[0].1 - char_counts[char_counts.len() - 1].1) as usize
}

fn to_char_counts(pair_counts: HashMap<String, u64>, start: &str) -> HashMap<char, u64> {
    let mut char_counts = HashMap::default();
    for (k, count) in pair_counts.iter() {
        for c in k.chars() {
            *char_counts.entry(c).or_insert(0) += count;
        }
    }
    // compensate for double counting
    for v in char_counts.values_mut() {
        *v /= 2;
    }
    let start_chars = start.chars().collect::<Vec<_>>();
    *char_counts
        .entry(*start_chars.first().unwrap())
        .or_insert(0) += 1;
    *char_counts.entry(*start_chars.last().unwrap()).or_insert(0) += 1;
    char_counts
}

fn windows(s: &str) -> impl Iterator<Item = &str> {
    s.as_bytes()
        .windows(2)
        .map(|b| std::str::from_utf8(b).unwrap())
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
