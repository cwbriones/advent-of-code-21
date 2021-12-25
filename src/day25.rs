use crate::prelude::*;

type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct Board {
    rightward: HashSet<Pos>,
    downward: HashSet<Pos>,
    width: usize,
    height: usize,
}

impl Board {
    fn step(&mut self) -> bool {
        let mut moved = false;
        let mut scratch = HashSet::default();
        scratch.reserve(self.rightward.len());

        for (i, j) in self.rightward.iter().cloned() {
            let nextpos = ((i + 1) % self.width, j);
            if !self.rightward.contains(&nextpos) && !self.downward.contains(&nextpos) {
                moved = true;
                scratch.insert(nextpos);
            } else {
                scratch.insert((i, j));
            }
        }
        std::mem::swap(&mut self.rightward, &mut scratch);
        scratch.clear();
        for (i, j) in self.downward.iter().cloned() {
            let nextpos = (i, (j + 1) % self.height);
            if !self.rightward.contains(&nextpos) && !self.downward.contains(&nextpos) {
                moved = true;
                scratch.insert(nextpos);
            } else {
                scratch.insert((i, j));
            }
        }
        std::mem::swap(&mut self.downward, &mut scratch);
        moved
    }
}

fn parse(input: &str) -> Board {
    let mut downward = HashSet::default();
    let mut rightward = HashSet::default();
    let mut width = 0;
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.trim().chars().enumerate() {
            if c == '>' {
                rightward.insert((i, j));
            } else if c == 'v' {
                downward.insert((i, j));
            }
        }
        width = line.trim().len();
    }
    let height = input.lines().count();
    Board {
        rightward,
        downward,
        width,
        height,
    }
}

fn part_one(mut board: Board) -> usize {
    let mut count = 0;
    while board.step() {
        count += 1;
    }
    count + 1
}

fn part_two(_board: Board) -> usize {
    /* Merry Christmas */
    0
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
