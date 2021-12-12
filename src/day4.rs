use crate::prelude::*;

#[derive(Debug, Clone)]
struct Board {
    entries: Vec<bool>,
    idx: HashMap<usize, usize>,
    len: usize,

    row_counts: Vec<usize>,
    col_counts: Vec<usize>,
}

impl Board {
    pub fn new(rows: Vec<Vec<usize>>) -> Self {
        let len = rows[0].len();
        let entries = vec![false; len * rows.len()];

        let row_counts = vec![len; len];
        let col_counts = vec![len; len];

        let mut idx = HashMap::default();
        for (i, r) in rows.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                idx.insert(*c, i * len + j);
            }
        }
        Board {
            idx,
            entries,
            len,
            row_counts,
            col_counts,
        }
    }

    /// Fill a space on the board matching n, if any.
    pub fn fill(&mut self, n: usize) -> bool {
        // Lookup the index, if any.
        if let Some(i) = self.idx.get(&n) {
            self.entries[*i] = true;

            let c = *i % self.len;
            let r = (*i - c) / self.len;

            self.row_counts[r] -= 1;
            if self.row_counts[r] == 0 {
                return true;
            }

            self.col_counts[c] -= 1;
            self.col_counts[c] == 0
        } else {
            false
        }
    }

    fn sum_empty(&self) -> usize {
        self.idx
            .iter()
            .filter_map(|(k, v)| if self.entries[*v] { None } else { Some(k) })
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Game {
    nums: Vec<usize>,
    boards: Vec<Board>,
}

impl Game {
    fn run(&mut self) -> usize {
        for n in &self.nums {
            for board in self.boards.iter_mut() {
                if board.fill(*n) {
                    return board.sum_empty() * n;
                }
            }
        }
        0
    }

    fn run_all(&mut self) -> usize {
        let mut wins_left = self.boards.len();
        let mut won = vec![false; wins_left];

        for n in &self.nums {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if won[i] {
                    continue;
                }
                if board.fill(*n) {
                    won[i] = true;
                    wins_left -= 1;
                }
                if wins_left == 0 {
                    return board.sum_empty() * n;
                }
            }
        }
        0
    }
}

fn parse(input: &str) -> Game {
    let mut lines = input.lines().peekable();
    let nums = parse_split::<usize>(lines.next().expect("nums"), ',').expect("parse nums");

    lines.next(); // skip newline

    let mut boards = Vec::new();
    while lines.peek().is_some() {
        boards.push(parse_board(&mut lines));
    }
    Game { nums, boards }
}

fn parse_board<'a, I>(lines: &mut std::iter::Peekable<I>) -> Board
where
    I: Iterator<Item = &'a str>,
{
    let mut rows = Vec::new();
    loop {
        let line = match lines.next() {
            Some(line) if !line.is_empty() => line,
            _ => break,
        };
        let row = parse_split::<usize>(line, ' ').unwrap();
        rows.push(row)
    }
    Board::new(rows)
}

fn part_one(mut game: Game) -> usize {
    game.run()
}

fn part_two(mut game: Game) -> usize {
    game.run_all()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[cfg(test)]
mod test {
    const INPUT: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn part_one() {
        let mut input = super::parse(INPUT).expect("parse");
        assert_eq!(super::part_one(&mut input), 4512)
    }

    #[test]
    fn part_two() {
        let mut input = super::parse(INPUT).expect("parse");
        assert_eq!(super::part_two(&mut input), 1924)
    }
}
