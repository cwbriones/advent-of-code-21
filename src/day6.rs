use crate::prelude::*;

fn parse(input: &str) -> [usize; 9] {
    let mut counts = [0; 9];
    let fish = input.trim().split(',').map(|s| s.parse::<usize>().unwrap());

    for f in fish {
        counts[f] += 1;
    }
    counts
}

fn part_one(mut fish: [usize; 9]) -> usize {
    iterate(&mut fish, 80);
    fish.iter().sum::<usize>()
}

fn part_two(mut fish: [usize; 9]) -> usize {
    iterate(&mut fish, 256);
    fish.iter().sum::<usize>()
}

fn iterate(fish: &mut [usize; 9], n_days: usize) {
    for _ in 0..n_days {
        // Process a day by shifting all the counters downward
        let n = fish[0];
        fish[..7].rotate_left(1);
        fish[6] += fish[7];
        fish[7] = fish[8];
        fish[8] = n;
    }
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[cfg(test)]
mod test {
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_one() {
        let input = super::parse(INPUT);
        assert_eq!(super::part_one(input), 5934)
    }

    #[test]
    fn part_two() {
        let input = super::parse(INPUT);
        assert_eq!(super::part_two(input), 26984457539);
    }
}
