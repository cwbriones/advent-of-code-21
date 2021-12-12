use crate::prelude::*;

fn parse(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.trim()).collect()
}

fn part_one(nums: Vec<&str>) -> usize {
    let len = nums[0].len();
    let mut counts = vec![0; len];
    for n in &nums {
        for (i, c) in n.chars().enumerate() {
            if c == '1' {
                counts[i] += 1
            }
        }
    }
    let mut gamma = vec!['0'; len];
    for (i, &c) in counts.iter().enumerate() {
        if c > nums.len() - c {
            gamma[i] = '1';
        }
    }
    let gamma = from_binary(&gamma.iter().collect::<String>());
    let epsilon = !gamma & ((1 << len) - 1);
    gamma * epsilon
}

fn part_two(nums: Vec<&str>) -> usize {
    let mut oxy = nums.iter().copied().collect::<Vec<_>>();
    let mut co = nums.iter().copied().collect::<Vec<_>>();
    filter(&mut oxy, false);
    filter(&mut co, true);
    from_binary(oxy[0]) * from_binary(co[0])
}

fn filter(nums: &mut Vec<&str>, reverse: bool) {
    let len = nums[0].len();
    let mut counts = vec![0; len];

    let ta = if reverse { '1' } else { '0' };
    let tb = if reverse { '0' } else { '1' };

    for i in 0..len {
        counts.iter_mut().for_each(|c| *c = 0);
        for n in nums.iter() {
            for (i, c) in n.chars().enumerate() {
                if c == '1' {
                    counts[i] += 1
                }
            }
        }
        let target = if 2 * counts[i] < nums.len() { ta } else { tb };
        nums.retain(|n| n.chars().nth(i).unwrap() == target);
        if nums.len() <= 1 {
            return;
        }
    }
}

fn from_binary(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[cfg(test)]
mod test {
    const INPUT: &str = "00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010";

    #[test]
    fn part_one() {
        let input = super::parse(INPUT).expect("parse");
        assert_eq!(super::part_one(&input), 198)
    }

    #[test]
    fn part_two() {
        let input = super::parse(INPUT).expect("parse");
        assert_eq!(super::part_two(&input), 230)
    }
}
