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
    let mut days = 0;
    loop {
        // Walk through the list to the first non-zero entry.
        let (i, n) = fish
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, f)| *f > 0)
            .unwrap();

        // Process i + 1 days by shifting all the counters downward
        fish.rotate_left(1);
        fish[(9 - i - 1)..9].fill(0);
        // Finally, update i=6 to reset the fish at the start and i=8
        // to account for the newly-created fish.
        fish[6] += n;
        fish[8] += n;

        days += i + 1;
        if days >= n_days {
            // We overshot, and that means a full cycle wouldn't
            // have completed anyway.
            if days > n_days {
                fish[6] -= n;
                fish[8] -= n;
            }
            return;
        }
    }
}

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let input = parse(input);
    runner.part_one(|| part_one(input));
    runner.part_two(|| part_two(input));
    Ok(())
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
