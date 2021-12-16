use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Line {
    start: (isize, isize),
    end: (isize, isize),
}

impl Line {
    fn new(start: (isize, isize), end: (isize, isize)) -> Self {
        Self { start, end }
    }

    fn is_simple(&self) -> bool {
        let dx = self.end.0.cmp(&self.start.0) as isize;
        let dy = self.end.1.cmp(&self.start.1) as isize;
        dx == 0 || dy == 0
    }

    fn points(&self) -> impl Iterator<Item = (isize, isize)> {
        let dx = self.end.0.cmp(&self.start.0) as isize;
        let dy = self.end.1.cmp(&self.start.1) as isize;

        let len = if dx != 0 {
            (self.end.0 - self.start.0) / dx
        } else {
            (self.end.1 - self.start.1) / dy
        };

        let start = self.start;
        (0isize..=len).map(move |i| (start.0 + dx * i, start.1 + dy * i))
    }
}

fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|l| parse_line(l.trim())).collect()
}

fn parse_line(line: &str) -> Line {
    let mut inums = line
        .split(" -> ")
        .flat_map(|t| t.split(','))
        .map(|p| p.parse::<isize>());
    let a1 = inums.next().unwrap().unwrap();
    let a2 = inums.next().unwrap().unwrap();
    let b1 = inums.next().unwrap().unwrap();
    let b2 = inums.next().unwrap().unwrap();
    Line::new((a1, a2), (b1, b2))
}

fn part_one(lines: Vec<Line>) -> usize {
    let mut counts = HashMap::default();
    lines
        .iter()
        .filter(|l| l.is_simple())
        .flat_map(|l| l.points())
        .for_each(|p| {
            let c = counts.entry(p).or_insert(0);
            *c += 1;
        });

    counts.values().filter(|v| **v > 1).count()
}

fn part_two(lines: Vec<Line>) -> usize {
    let mut counts = HashMap::default();
    lines.iter().flat_map(|l| l.points()).for_each(|p| {
        let c = counts.entry(p).or_insert(0);
        *c += 1;
    });

    counts.values().filter(|v| **v > 1).count()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[cfg(test)]
mod test {
    const INPUT: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn part_one() {
        let input = super::parse(INPUT);
        assert_eq!(super::part_one(input), 5)
    }

    #[test]
    fn part_two() {
        let input = super::parse(INPUT);
        assert_eq!(super::part_two(input), 12)
    }
}
