use crate::prelude::*;

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part_one(mut fish: Vec<usize>) -> usize {
    // Brute force
    let mut new_fish = Vec::new();
    for _ in 0..80 {
        new_fish.clear();
        for f in &mut fish {
            if *f == 0 {
                *f = 6;
                new_fish.push(8);
                continue;
            }
            *f -= 1;
        }
        fish.extend_from_slice(&new_fish[..]);
    }
    fish.len()
}

fn part_two(fish: Vec<usize>) -> usize {
    fish[0]
}

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let input = parse(input);
    runner.part_one(|| part_one(input.clone()));
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
