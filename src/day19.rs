use std::collections::VecDeque;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(isize, isize, isize);

impl Point {
    fn relative(&self, other: &Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    fn square_distance(&self, other: &Point) -> isize {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        dx * dx + dy * dy + dz * dz
    }

    fn add(&self, other: &Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    let mut lines = input.lines().peekable();
    let mut scanners = Vec::new();
    while lines.peek().is_some() {
        let scanner = parse_scanner(&mut lines).unwrap();
        scanners.push(scanner);
    }
    scanners
}

fn parse_scanner<'a, I>(mut lines: I) -> Option<Vec<Point>>
where
    I: Iterator<Item = &'a str>,
{
    let header = lines.next()?;
    if !header.starts_with("---") {
        return None;
    }
    let mut points = Vec::new();
    loop {
        let line = match lines.next() {
            Some(l) if l.is_empty() => break,
            Some(l) => l,
            None => break,
        };
        let mut split = line.split(',').filter_map(|s| s.parse::<isize>().ok());
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        let z = split.next().unwrap();
        points.push(Point(x, y, z));
    }
    Some(points)
}

fn part_one(scans: Vec<Vec<Point>>) -> usize {
    let mut transforms = HashMap::default();
    let mut culleda = HashSet::default();
    let mut culledb = HashSet::default();
    for (i, a) in scans.iter().enumerate() {
        let distances = pairs(a)
            .map(|(p, q)| (p.square_distance(q), (p, q)))
            .collect::<HashMap<_, _>>();

        for (j, b) in scans[i + 1..].iter().enumerate() {
            culleda.clear();
            culledb.clear();

            for (pb, qb) in pairs(b) {
                let dis = pb.square_distance(qb);
                if let Some((pa, qa)) = distances.get(&dis) {
                    culleda.insert(**pa);
                    culleda.insert(**qa);
                    culledb.insert(*pb);
                    culledb.insert(*qb);
                }
            }
            if culledb.len() < 12 {
                continue;
            }
            if let Some(transform) = orient(&culleda, &culledb) {
                transforms.insert((i, i + j + 1), transform.inv());
                transforms.insert((i + j + 1, i), transform);
            }
        }
    }
    let mut all_beacons = HashSet::default();
    all_beacons.extend(&scans[0]);
    for (i, scan) in scans.iter().enumerate().skip(1) {
        let path = find_path(i, scans.len(), &transforms);
        println!("{:?}", path);
        for b in scan {
            let b = path
                .iter()
                .map(|k| transforms.get(k).unwrap())
                .fold(*b, |b, t| t.apply(&b));
            all_beacons.insert(b);
        }
        let origin = path
            .iter()
            .filter_map(|k| transforms.get(k))
            .fold(Point(0, 0, 0), |b, t| t.apply(&b));
        println!("{} is at {:?}", i, origin);
    }
    all_beacons.len()
}

fn find_path(
    i: usize,
    n: usize,
    transforms: &HashMap<(usize, usize), Transform>,
) -> Vec<(usize, usize)> {
    let mut fringe = VecDeque::new();
    fringe.push_back((i, Vec::new()));
    let mut seen = HashSet::default();
    while let Some((p, path)) = fringe.pop_front() {
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);
        if p == 0 {
            return path;
        }
        let next = (0..n).filter(|j| *j != p);
        for j in next {
            let k = (p, j);
            if transforms.contains_key(&k) {
                let mut path = path.clone();
                path.push(k);
                fringe.push_back((j, path));
            }
        }
    }
    panic!("no path found");
}

fn orient(first: &HashSet<Point>, second: &HashSet<Point>) -> Option<Transform> {
    for p in first {
        for q in second {
            // Assume p and q are the same beacon in two different
            // coordinate systems and see if it works.
            //
            // Translate all of first to q, then try every rotation
            let transform = find_transform(second, *q, first, *p);
            if transform.is_some() {
                return transform;
            }
        }
    }
    None
}

fn find_transform(
    source: &HashSet<Point>,
    p: Point,
    dest: &HashSet<Point>,
    q: Point,
) -> Option<Transform> {
    // This implies 48 but there's actually only 24 possible outcomes.
    // The permutations combined with a rotation actually are not unique
    let permutations = [
        [(0, 0), (0, 0)],
        [(0, 0), (1, 2)],
        [(0, 0), (0, 1)],
        [(0, 0), (0, 2)],
        [(0, 2), (0, 1)],
        [(0, 1), (0, 2)],
    ];
    let signs = [
        (1, 1, 1),
        (1, -1, 1),
        (1, 1, -1),
        (1, -1, -1),
        (-1, 1, 1),
        (-1, -1, 1),
        (-1, 1, -1),
        (-1, -1, -1),
    ];
    for perm in permutations {
        for sign in signs {
            let transform = Transform {
                p,
                q,
                rotation: Rotation { perm, sign },
            };
            let matched = source
                .iter()
                .map(|p| transform.apply(p))
                .all(|p| dest.contains(&p));
            if matched {
                return Some(transform);
            }
        }
    }
    None
}

#[derive(Debug, Clone)]
struct Transform {
    p: Point,
    q: Point,
    rotation: Rotation,
}

impl Transform {
    fn apply(&self, p: &Point) -> Point {
        self.rotation.apply(p.relative(&self.p)).add(&self.q)
    }

    fn inv(&self) -> Self {
        Transform {
            p: self.q,
            q: self.p,
            rotation: self.rotation.inv(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    perm: [(usize, usize); 2],
    sign: (isize, isize, isize),
}

impl Rotation {
    fn apply(&self, p: Point) -> Point {
        let mut transformed = swap(p, &self.perm);
        transformed.0 *= self.sign.0;
        transformed.1 *= self.sign.1;
        transformed.2 *= self.sign.2;
        transformed
    }

    fn inv(&self) -> Rotation {
        let mut perm = self.perm;
        perm.swap(0, 1);
        Rotation {
            perm,
            sign: self.sign,
        }
    }
}

fn swap(mut p: Point, swaps: &[(usize, usize)]) -> Point {
    for swap in swaps {
        let (from, to) = match swap {
            (0, 0) => continue,
            (0, 1) => (&mut p.0, &mut p.1),
            (0, 2) => (&mut p.0, &mut p.2),
            (1, 2) => (&mut p.1, &mut p.2),
            _ => panic!("bad perm"),
        };
        std::mem::swap(from, to);
    }
    p
}

fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice
        .iter()
        .enumerate()
        .flat_map(|(i, p)| slice[i + 1..].iter().map(move |q| (p, q)))
}

fn part_two(_scans: Vec<Vec<Point>>) -> usize {
    0
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
