use crate::prelude::*;

pub fn run(input: &str, runner: &Runner) -> Result<()> {
    let input = parse(input.trim());
    runner.part_one(|| part_one(&input));
    runner.part_two(|| part_two(&input));
    Ok(())
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| ((c as usize) - ('0' as usize)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_one(input: &[Vec<usize>]) -> usize {
    let mut risk = 0;
    for (x, y) in low_points(input) {
        risk += input[y][x] + 1;
    }
    risk
}

fn part_two(input: &[Vec<usize>]) -> usize {
    let points = low_points(input);

    let mut stack = Vec::new();
    let mut visited = HashSet::default();
    let mut visited_all = HashSet::default();
    let mut basins = Vec::new();

    for point in points {
        stack.clear();
        visited.clear();

        stack.push(point);

        while let Some(point @ (x, y)) = stack.pop() {
            if visited.contains(&point) {
                continue;
            }
            visited.insert(point);
            let p = input[y][x];
            if x + 1 < input[y].len() && adjacent(input[y][x + 1], p) {
                stack.push((x + 1, y));
            }
            if y + 1 < input.len() && adjacent(input[y + 1][x], p) {
                stack.push((x, y + 1));
            }
            if x > 0 && adjacent(input[y][x - 1], p) {
                stack.push((x - 1, y));
            }
            if y > 0 && adjacent(input[y - 1][x], p) {
                stack.push((x, y - 1));
            }
        }
        basins.push(visited.len());
        visited_all.extend(visited.iter().cloned());
    }
    // display_basins(input, &visited_all);
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn low_points(input: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for (y, row) in input.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            if x + 1 < row.len() && row[x + 1] <= *p {
                continue;
            }
            if y + 1 < input.len() && input[y + 1][x] <= *p {
                continue;
            }
            if x > 0 && row[x - 1] <= *p {
                continue;
            }
            if y > 0 && input[y - 1][x] <= *p {
                continue;
            }
            points.push((x, y));
        }
    }
    points
}

#[inline]
fn adjacent(a: usize, b: usize) -> bool {
    a < 9 && a > b
}

// Used for debugging. Output a picture of the input with all the basins colored.
//
// fn display_basins(input: &[Vec<usize>], visited_all: &HashSet<(usize, usize)>) {
//     let strings = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
//     let mut rowbuf = String::new();
//     for (y, row) in input.iter().enumerate() {
//         let mut cur_color = false;
//         for (x, &p) in row.iter().enumerate() {
//             let next_color = visited_all.contains(&(x, y));
//             if next_color != cur_color {
//                 let seq = if next_color {
//                     "\u{001b}[31m"
//                 } else {
//                     "\u{001b}[0m"
//                 };
//                 rowbuf.push_str(seq);
//             }
//             rowbuf.push(strings[p]);
//             cur_color = next_color;
//         }
//         rowbuf.push_str("\u{001b}[0m");
//         println!("{}", &rowbuf);
//         rowbuf.clear();
//     }
// }
