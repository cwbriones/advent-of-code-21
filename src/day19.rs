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

// struct Matrix(Point, Point, Point);
//
// impl Matrix {
//     fn new(a: Point, b: Point, c: Point) -> Self {
//         Matrix(a, b, c)
//     }
//
//     fn mul(&self, p: Point) -> Point {
//         Point(
//             self.0 .0 * p.0 + self.0 .1 * p.1 + self.0 .2 * p.2,
//             self.1 .0 * p.0 + self.1 .1 * p.1 + self.1 .2 * p.2,
//             self.2 .0 * p.0 + self.2 .1 * p.1 + self.2 .2 * p.2,
//         )
//     }
// }
//
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
    let mut transforms = HashMap::<(usize, usize), Transform>::default();

    for ((i, a), (j, b)) in pairs_enumerate(&scans) {
        if let Some(transform) = orient(a, b) {
            if i < j {
                transforms.insert((i, j), transform);
            } else {
                transforms.insert((j, i), transform);
            }
        }
    }

    let mut resolved_beacons = HashSet::<Point>::default();
    for (i, beacons) in scans.iter().enumerate() {
        if i == 0 {
            resolved_beacons.extend(beacons);
            continue;
        }
        let path = find_path(i, scans.len(), &transforms);
        for b in beacons {
            let bt = path
                .iter()
                .map(|p| transforms.get(p).unwrap())
                .fold(*b, |b, t| t.apply(&b));
            resolved_beacons.insert(bt);
        }
        let mut origin = Point(0, 0, 0);
        for t in path.iter().filter_map(|p| transforms.get(p)) {
            origin = t.apply(&origin);
        }
        println!("{} relative to {}: {:?}", i, 0, origin);
    }
    resolved_beacons.len()
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
            let key = if j < p { (j, p) } else { (p, j) };
            if transforms.contains_key(&key) {
                let mut path = path.clone();
                path.push(key);
                fringe.push_back((j, path));
            }
        }
    }
    panic!("no path found");
}

// Produce a relative transform between the two scanner's coordinate systems based on their readings.
//
// If the scanners do not have overlapping beacons then returns None.
//
// The resulting transform will take a point in S2 to S1
fn orient(first: &[Point], second: &[Point]) -> Option<Transform> {
    // First find a set of at least 3 points that we know the distances between,
    // and where those distances are found in both `first` and `second`.
    let (origin1, ref1) = find_set(first, second)?;
    let pairs1 = ref1
        .iter()
        .map(|q| (origin1.square_distance(q), (origin1, q)))
        .collect::<HashMap<_, _>>();

    // FIXME: This can probably be computed at the same time within `find_set`
    let mut seen = HashMap::default();
    let mut pairs2 = HashMap::default();
    for (p, q) in pairs(second) {
        let d = p.square_distance(q);
        if pairs1.contains_key(&d) {
            pairs2.insert(d, (p, q));
            *seen.entry(p).or_insert(0) += 1;
            *seen.entry(q).or_insert(0) += 1;
        }
        if pairs2.len() == pairs1.len() {
            break;
        }
    }
    let (origin2, _) = seen.into_iter().max_by_key(|p| p.1).unwrap();

    // At this point we have, for each scanner, pairs of beacons with the same distances
    //
    // The first part p1 is the beacon that is common to both scans.
    let (d, pair1) = pairs1.iter().next().unwrap();
    let p1 = pair1.1.relative(&origin1);
    let p2 = {
        let pair2 = pairs2.get(d).unwrap();
        let p = pair2.0.relative(origin2);
        let q = pair2.1.relative(origin2);
        if p == Point(0, 0, 0) {
            q
        } else {
            p
        }
    };
    let rotation = find_rotation(p2, p1);
    let transform = Transform {
        rotation,
        origin1,
        origin2: *origin2,
    };
    Some(transform)
}

fn find_rotation(p: Point, q: Point) -> Rotation {
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
            // apply permutation
            let rotation = Rotation { perm, sign };
            let rotated = rotation.apply(p);

            if rotated == q {
                return rotation;
            }
        }
    }
    panic!("no transformation found");
}

struct Transform {
    origin1: Point,
    origin2: Point,
    rotation: Rotation,
}

impl Transform {
    fn apply(&self, p: &Point) -> Point {
        self.rotation
            .apply(p.relative(&self.origin2))
            .add(&self.origin1)
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

// Find a set of at least 3 distances and their points that are present in both scanner readings.
fn find_set(first: &[Point], second: &[Point]) -> Option<(Point, Vec<Point>)> {
    let mut distances1 = HashMap::default();
    for (p, q) in pairs(first) {
        let d = p.square_distance(q);
        distances1.insert(d, (p, q));
    }

    let mut overlap = HashMap::<Point, Vec<Point>>::default();
    for (p, q) in pairs(second) {
        let d = p.square_distance(q);
        if let Some((p1, q1)) = distances1.get(&d) {
            overlap.entry(**p1).or_insert_with(Vec::new).push(**q1);
            overlap.entry(**q1).or_insert_with(Vec::new).push(**p1);
        }
    }
    if overlap.len() < 11 {
        return None;
    }
    for (p, qs) in &overlap {
        if qs.len() >= 2 {
            return Some((*p, qs[..2].to_owned()));
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

fn pairs_enumerate<T>(slice: &[T]) -> impl Iterator<Item = ((usize, &T), (usize, &T))> {
    slice.iter().enumerate().flat_map(|(i, p)| {
        slice[i + 1..]
            .iter()
            .enumerate()
            .map(move |(j, q)| ((i, p), (i + j + 1, q)))
    })
}

fn part_two(scans: Vec<Vec<Point>>) -> usize {
    0
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
