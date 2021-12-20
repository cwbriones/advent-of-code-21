use crate::prelude::*;

fn parse(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut lines = input.lines().map(|l| l.trim_end());
    let algorithm = parse_line(lines.next().unwrap());
    let blank = lines.next().unwrap();
    if !blank.trim_end().is_empty() {
        panic!("expected blank line");
    }
    let mut image = lines.map(parse_line).collect();
    pad(&mut image);
    (algorithm, image)
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars()
        .map(|c| (c == '#') as usize)
        .collect::<Vec<_>>()
}

fn pad(image: &mut Vec<Vec<usize>>) {
    let width = image[0].len();
    for row in image.iter_mut() {
        row.insert(0, 0);
        row.insert(0, 0);
        row.push(0);
        row.push(0);
    }
    let mut blank_row = Vec::new();
    blank_row.resize(width + 4, 0);
    image.insert(0, blank_row.clone());
    image.insert(0, blank_row.clone());
    image.push(blank_row.clone());
    image.push(blank_row);
}

fn enhance(mut image: Vec<Vec<usize>>, algorithm: &[usize], iters: usize) -> usize {
    let mut padding = 0;
    for _ in 0..iters {
        let (new_image, new_padding) = tick(&image, algorithm, padding);
        image = new_image;
        padding = new_padding;
    }
    image.iter().flat_map(|row| row.iter()).sum::<usize>()
}

fn part_one(input: (Vec<usize>, Vec<Vec<usize>>)) -> usize {
    let (algorithm, image) = input;
    enhance(image, &algorithm, 2)
}

fn part_two(input: (Vec<usize>, Vec<Vec<usize>>)) -> usize {
    let (algorithm, image) = input;
    enhance(image, &algorithm, 50)
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

// Debugging.
// fn display(image: &[Vec<usize>]) {
//     for row in image {
//         let line = row
//             .iter()
//             .map(|c| if *c == 0 { '.' } else { '#' })
//             .collect::<String>();
//         println!("{}", line);
//     }
//     println!()
// }

fn tick(image: &[Vec<usize>], algorithm: &[usize], pad: usize) -> (Vec<Vec<usize>>, usize) {
    let width = image[0].len();
    let mut blank_row = Vec::new();

    let pad_idx = if pad > 0 { 511 } else { 0 };
    let next_pad = algorithm[pad_idx];
    blank_row.resize(width + 2, next_pad);

    let mut out = vec![blank_row.clone(), blank_row.clone()];

    for window in image.windows(3) {
        let top = window[0].windows(3);
        let mid = window[1].windows(3);
        let bot = window[2].windows(3);

        let mut row_out = Vec::with_capacity(width + 2);
        row_out.push(next_pad);
        row_out.push(next_pad);
        for ((one, two), three) in top.zip(mid).zip(bot) {
            let mut idx = 0;
            for b in one.iter().chain(two).chain(three) {
                idx <<= 1;
                idx += *b;
            }
            row_out.push(algorithm[idx]);
        }
        row_out.push(next_pad);
        row_out.push(next_pad);
        out.push(row_out);
    }
    out.push(blank_row.clone());
    out.push(blank_row);
    (out, next_pad)
}
