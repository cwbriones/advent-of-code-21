use crate::prelude::*;

const MIN_OVERLAP: usize = 12;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(isize, isize, isize);

impl Point {
    fn sub(&self, other: &Point) -> Point {
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

    fn manhattan(&self, other: &Self) -> isize {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        dx.abs() + dy.abs() + dz.abs()
    }
}

#[derive(Debug, Clone)]
struct Transform {
    p: Point,
    q: Point,
    rotation: Rotation,
}

impl Transform {
    fn apply(&self, p: &Point) -> Point {
        self.rotation.apply(p.sub(&self.p)).add(&self.q)
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

fn solve(scans: Vec<Vec<Point>>) -> (HashSet<Point>, Vec<Point>) {
    let mut culleda = HashSet::default();
    let mut culledb = HashSet::default();
    let mut remaining = (1..scans.len()).collect::<HashSet<_>>();

    let mut merged = scans[0].iter().cloned().collect::<HashSet<_>>();
    let mut distances = pairs(&scans[0])
        .map(|(p, q)| (p.square_distance(q), (*p, *q)))
        .collect::<HashMap<_, _>>();
    let mut scanner_positions = vec![Point(0, 0, 0)];
    while !remaining.is_empty() {
        // Find a scanner that has enough overlap to be merged.
        let (j, transform) = remaining
            .iter()
            .find_map(|j| {
                let other = &scans[*j];
                culleda.clear();
                culledb.clear();
                for (pb, qb) in pairs(other) {
                    let dis = pb.square_distance(qb);
                    if let Some((pa, qa)) = distances.get(&dis) {
                        culleda.insert(*pa);
                        culleda.insert(*qa);
                        culledb.insert(*pb);
                        culledb.insert(*qb);
                    }
                }
                if culledb.len() < MIN_OVERLAP {
                    return None;
                }
                orient(&culleda, &culledb).map(|transform| (*j, transform))
            })
            .expect("no scanners to merge");
        // Merge the scanner readings together by transforming the coordinate and then adding
        // to the merged set.
        for b in &scans[j] {
            let tb = transform.apply(b);
            for a in &merged {
                let dis = a.square_distance(&tb);
                distances.insert(dis, (*a, tb));
            }
            merged.insert(tb);
        }
        scanner_positions.push(transform.apply(&Point(0, 0, 0)));
        remaining.remove(&j);
    }
    (merged, scanner_positions)
}

fn orient(first: &HashSet<Point>, second: &HashSet<Point>) -> Option<Transform> {
    for p in first {
        for q in second {
            // Assume p and q are the same beacon in two different
            // coordinate systems and see if it works.
            //
            // Translate all of first to q, then try every rotation
            if let Some(t) = find_transform(second, *q, first, *p) {
                return Some(t);
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
            let match_count = source
                .iter()
                .map(|p| transform.apply(p))
                .filter(|p| dest.contains(p))
                .count();
            if match_count >= MIN_OVERLAP {
                return Some(transform);
            }
        }
    }
    None
}

fn pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice
        .iter()
        .enumerate()
        .flat_map(|(i, p)| slice[i + 1..].iter().map(move |q| (p, q)))
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
    solve(scans).0.len()
}

fn part_two(scans: Vec<Vec<Point>>) -> usize {
    let (_beacons, scanners) = solve(scans);
    pairs(&scanners)
        .map(|(a, b)| a.manhattan(b) as usize)
        .max()
        .unwrap()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
