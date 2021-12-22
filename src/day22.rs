use crate::prelude::*;

fn parse(input: &str) -> Vec<(bool, Cube)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ').map(|s| s.trim_end());
            let state = parts.next().unwrap();
            if state != "on" && state != "off" {
                panic!("bad state: {}", state);
            }
            let state = state == "on";
            let cube = parts.next().unwrap().parse::<Cube>().unwrap();
            (state, cube)
        })
        .collect::<Vec<_>>()
}

fn parse_range(input: &str, prefix: &str) -> Result<Range> {
    let mut xs = input
        .strip_prefix(prefix)
        .expect("missing prefix")
        .split("..")
        .map(|s| s.parse::<isize>());
    let start = xs.next().expect("missing range start")?;
    let end = xs.next().expect("missing range end")?;
    Ok(Range { start, end })
}

fn part_one(nums: Vec<(bool, Cube)>) -> usize {
    let init = nums
        .iter()
        .filter(|(_, s)| is_init(&s.x) && is_init(&s.y) && is_init(&s.z))
        .cloned()
        .collect::<Vec<_>>();
    solve(init)
}

fn is_init(range: &Range) -> bool {
    range.start >= -50 && range.end <= 50
}

fn part_two(nums: Vec<(bool, Cube)>) -> usize {
    solve(nums)
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Cube {
    x: Range,
    y: Range,
    z: Range,
}

impl Cube {
    fn intersect(&self, other: &Self) -> Option<Cube> {
        intersect(&self.x, &other.x).and_then(|x| {
            intersect(&self.y, &other.y)
                .and_then(move |y| intersect(&self.z, &other.z).map(move |z| Cube { x, y, z }))
        })
    }

    fn volume(&self) -> isize {
        (self.x.end - self.x.start + 1)
            * (self.y.end - self.y.start + 1)
            * (self.z.end - self.z.start + 1)
    }
}

impl std::str::FromStr for Cube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut ranges = s.split(',');
        let x = parse_range(ranges.next().unwrap(), "x=")?;
        let y = parse_range(ranges.next().unwrap(), "y=")?;
        let z = parse_range(ranges.next().unwrap(), "z=")?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn contains(&self, p: isize) -> bool {
        self.start <= p && p <= self.end
    }
}

fn intersect(a: &Range, b: &Range) -> Option<Range> {
    // a leads
    // a0              a1
    // <---------------->
    //        <-------|-------->
    //        b0      b1  or  b1
    if a.contains(b.start) {
        let end = b.end.min(a.end);
        return Some(Range {
            start: b.start,
            end,
        });
    }
    // b leads
    //        <-------|-------->
    //        a0      a1  or  a1
    // <---------------->
    // b0              b1
    if b.contains(a.start) {
        let end = b.end.min(a.end);
        return Some(Range {
            start: a.start,
            end,
        });
    }
    None
}

fn solve(cubes: Vec<(bool, Cube)>) -> usize {
    let mut weights = HashMap::<Cube, isize>::default();

    // for each new cuboid
    for (on, cube) in cubes {
        let intersections: Option<Vec<(Cube, isize)>> = {
            let inter = weights
                .iter()
                .flat_map(|(s, w)| s.intersect(&cube).map(|inter| (inter, *w)))
                .collect::<Vec<_>>();
            if inter.is_empty() {
                None
            } else {
                Some(inter)
            }
        };
        if on {
            *weights.entry(cube).or_insert(0) += 1;
        }
        for (intersection, weight) in intersections.iter().flat_map(|i| i.iter()) {
            let entry = weights.entry(*intersection).or_insert(0);
            *entry -= weight;
            if *entry == 0 {
                weights.remove(intersection);
            }
        }
    }
    weights.iter().map(|(s, w)| s.volume() * w).sum::<isize>() as usize
}
