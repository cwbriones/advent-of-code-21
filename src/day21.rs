use crate::prelude::*;

fn parse(input: &str) -> (usize, usize) {
    let mut line = input.lines().flat_map(|l| {
        l.split(':')
            .nth(1)
            .and_then(|s| s.trim().parse::<usize>().ok())
    });

    let one = line.next().unwrap();
    let two = line.next().unwrap();
    (one - 1, two - 1)
}

fn part_one(start: (usize, usize)) -> usize {
    let (mut pos1, mut pos2) = start;
    let (mut score1, mut score2) = (0, 0);

    let mut die = deterministic();
    for turn in 0.. {
        let player = turn % 2;
        let (pos, score) = if player == 0 {
            (&mut pos1, &mut score1)
        } else {
            (&mut pos2, &mut score2)
        };
        let roll = die.by_ref().take(3).sum::<usize>();
        *pos = (*pos + roll) % 10;
        *score += *pos + 1;
        if *score >= 1000 {
            let loser = if player == 0 { score2 } else { score1 };
            return loser * (turn + 1) * 3;
        }
    }
    unreachable!("");
}

fn deterministic() -> impl Iterator<Item = usize> {
    let mut die = 0;
    std::iter::from_fn(move || {
        let roll = die + 1;
        die = (die + 1) % 100;
        Some(roll)
    })
}

fn part_two(start: (usize, usize)) -> usize {
    let (p1, p2) = start;
    let mut cache = HashMap::default();

    let (wins, losses) = ways(p1, 0, p2, 0, &mut cache);
    wins.max(losses) as usize
}

// Compute # of ways for player 1 to result in outcome `win`
fn ways(
    pos1: usize,
    score1: usize,
    pos2: usize,
    score2: usize,
    cache: &mut HashMap<(usize, usize, usize, usize), (usize, usize)>,
) -> (usize, usize) {
    if score2 >= 21 {
        return (0, 1);
    }
    let key = (pos1, score1, pos2, score2);
    if let Some(val) = cache.get(&key) {
        return *val;
    }
    // All possible rolls.
    //
    // # of ways to roll
    // value of roll is idx + 3
    let rolls = [1, 3, 6, 7, 6, 3, 1];
    let total = rolls
        .iter()
        .enumerate()
        .map(|(i, roll_ways)| {
            let p1 = (pos1 + i + 3) % 10;
            let (losses, wins) = ways(pos2, score2, p1, score1 + p1 + 1, cache);
            (roll_ways * wins, roll_ways * losses)
        })
        .fold((0, 0), |acc, i| (acc.0 + i.0, acc.1 + i.1));
    cache.insert(key, total);
    total
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
