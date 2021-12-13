use crate::prelude::*;

#[derive(Debug, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

#[derive(Debug, Clone)]
struct Input {
    paper: Vec<(usize, usize)>,
    folds: Vec<Fold>,
}

impl Input {
    fn fold(&mut self) {
        let select_x: &Selector = &|p| &mut p.0;
        let select_y: &Selector = &|p| &mut p.1;
        for fold in &self.folds {
            let (selector, axis) = match *fold {
                Fold::X(axis) => (select_x, axis),
                Fold::Y(axis) => (select_y, axis),
            };
            for x in self.paper.iter_mut().map(selector) {
                if *x <= axis {
                    continue;
                }
                *x = 2 * axis - *x;
            }
        }
    }

    fn display_paper(&self) {
        let (max_x, max_y) = self.paper.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });
        let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
        for (x, y) in &self.paper {
            grid[*y][*x] = true;
        }
        for row in &grid {
            let rowstr = row
                .iter()
                .map(|b| if *b { '#' } else { ' ' })
                .collect::<String>();
            println!("{}", rowstr);
        }
        println!();
    }
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let paper = lines
        .by_ref()
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            let mut split = l.split(',').map(|c| c.parse::<usize>().unwrap());
            let x = split.next().unwrap();
            let y = split.next().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>();

    let folds = lines
        .map(|line| {
            let mut split = line["fold along ".len()..].split('=');
            let parts = split
                .next()
                .and_then(|s| split.next().map(|c| (s, c.parse::<usize>().unwrap())));
            match parts {
                Some(("x", x)) => Fold::X(x),
                Some(("y", y)) => Fold::Y(y),
                _ => panic!("bad fold: {}", line),
            }
        })
        .collect();

    Input { paper, folds }
}

type Selector = dyn Fn(&mut (usize, usize)) -> &mut usize;

fn part_one(mut input: Input) -> usize {
    input.folds.truncate(1);
    input.fold();
    input.paper.iter().collect::<HashSet<_>>().len()
}

fn part_two(mut input: Input) -> usize {
    input.fold();
    input.display_paper();
    1234
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
