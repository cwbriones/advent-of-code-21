use std::ops::RangeInclusive;

use crate::prelude::*;

#[derive(Debug, Clone)]
struct TargetArea {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl TargetArea {
    fn contains(&self, p: &(isize, isize)) -> bool {
        self.x.contains(&p.0) && self.y.contains(&p.1)
    }
}

fn parse(input: &str) -> TargetArea {
    // target area: x=20..30, y=-10..-5
    let input = input
        .trim()
        .strip_prefix("target area: ")
        .expect("missing prefix");
    let mut split = input.split(", ");

    let x_range = split.next().expect("missing x-range");
    let x = parse_range(x_range, "x=").unwrap();

    let y_range = split.next().expect("missing y-range");
    let y = parse_range(y_range, "y=").unwrap();
    TargetArea { x, y }
}

fn parse_range(input: &str, prefix: &str) -> Result<RangeInclusive<isize>> {
    let mut xs = input
        .strip_prefix(prefix)
        .expect("missing prefix")
        .split("..")
        .map(|s| s.parse::<isize>());
    let start = xs.next().expect("missing range start")?;
    let end = xs.next().expect("missing range end")?;
    Ok(start..=end)
}

fn launch(
    mut vel_x: isize,
    mut vel_y: isize,
) -> impl Iterator<Item = ((isize, isize), (isize, isize))> {
    let mut pos = (0isize, 0isize);
    std::iter::from_fn(move || {
        pos.0 += vel_x;
        pos.1 += vel_y;
        let v = (vel_x, vel_y);
        vel_x -= vel_x.signum();
        vel_y -= 1;
        Some((pos, v))
    })
}

fn part_one(area: TargetArea) -> usize {
    // Since we want max height, the only dimension that really matters is y.
    //
    // Only considering y:
    //
    // 1. When the probe falls back down to y=0, it will have velocity -vi.
    // 2. In order to not pass the zone, |vi| must be at most |y_end|
    // 3. The distance traveled up until this point will be SUM(1..=vi-1){n} = (vi-1)*(vi)/2
    //
    // Example input sanity check: y=-10..-5
    //
    // v_y = -(-10) = 10, => h = SUM(1..=9){n} => 45
    //
    // What about x?
    //
    // Similar to the above, the probe will travel in the x direction SUM(1..=v_x){n} until it
    // stops. So as long as we can choose a number v_x where that sum ends up in the target area,
    // we're okay. With the example above is v_x=6 or 7, since then x_final = 21 or 28.
    //
    // If we cannot choose an x, that would make the problem impossible, which I hope it isn't :).
    // Since we can always choose a number, it doesn't matter.
    //
    let min_y = *area.y.start() as usize;
    (min_y * (min_y + 1)) / 2
}

fn part_two(area: TargetArea) -> usize {
    // Compute the boundaries, but otherwise brute force.
    //
    // Read above for computing min_x and min_y.
    // x_end = area.x.end() because otherwise t=1 the probe would immediately go past the target.
    let mut min_x = 1;
    while !area.x.contains(&(min_x * (min_x + 1) / 2)) {
        min_x += 1;
    }
    let min_y = *area.y.start();
    (min_x..=*area.x.end())
        .flat_map(|dx| (min_y..-min_y).map(move |dy| (dx, dy)))
        .filter(|vel| {
            launch(vel.0, vel.1)
                .take_while(|(p, _)| p.1 >= *area.y.start())
                .any(|(p, _)| area.contains(&p))
        })
        .count()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
