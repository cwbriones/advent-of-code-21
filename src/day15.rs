use std::collections::BinaryHeap;

use crate::prelude::*;

fn parse(r: &str) -> Vec<Vec<isize>> {
    r.lines()
        .map(|line| {
            line.chars()
                .map(|c| (c as isize) - ('0' as isize))
                .collect()
        })
        .collect()
}

struct SearchState {
    p: (isize, isize),
    path: Vec<(isize, isize)>,
    cost: isize,
}

struct CmpUsing<T, F> {
    f: F,
    t: T,
}

fn cmp_using<T, U, F>(t: T, f: F) -> CmpUsing<T, F>
where
    F: Fn(&T) -> U,
    U: PartialEq,
{
    CmpUsing { t, f }
}

impl<T, U, F> PartialEq for CmpUsing<T, F>
where
    F: Fn(&T) -> U,
    U: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let f = &self.f;
        f(&self.t).eq(&f(&other.t))
    }
}

impl<T, U, F> Eq for CmpUsing<T, F>
where
    F: Fn(&T) -> U,
    U: Eq,
{
}

impl<T, U, F> PartialOrd for CmpUsing<T, F>
where
    F: Fn(&T) -> U,
    U: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let f = &self.f;
        f(&self.t).partial_cmp(&f(&other.t))
    }
}

impl<T, U, F> Ord for CmpUsing<T, F>
where
    F: Fn(&T) -> U,
    U: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let f = &self.f;
        f(&self.t).cmp(&f(&other.t))
    }
}

fn part_one(nums: Vec<Vec<isize>>) -> usize {
    let state = search(nums);
    state.cost as usize
}

fn search(mut nums: Vec<Vec<isize>>) -> SearchState {
    let mut fringe = BinaryHeap::new();
    let cost_cmp = |s: &SearchState| s.cost;
    nums[0][0] = 0;
    fringe.push(std::cmp::Reverse(cmp_using(
        SearchState {
            p: (0, 0),
            path: vec![(0, 0)],
            cost: 0,
        },
        cost_cmp,
    )));
    let target = ((nums.len() - 1) as isize, (nums[0].len() - 1) as isize);
    let mut visited = HashSet::default();
    while let Some(mut state) = fringe.pop().map(|std::cmp::Reverse(t)| t.t) {
        if visited.contains(&state.p) {
            // Found a cheaper path
            continue;
        }
        visited.insert(state.p);

        let (y, x) = state.p;
        state.cost += nums[y as usize][x as usize];
        if state.p == target {
            return state;
        }
        let neighbors = [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];
        neighbors
            .iter()
            .filter(|(y, x)| {
                *y >= 0 && *x >= 0 && *y < nums.len() as isize && *x < nums[0].len() as isize
            })
            .for_each(|&p| {
                let mut path = state.path.clone();
                path.push(p);
                fringe.push(std::cmp::Reverse(cmp_using(
                    SearchState {
                        p,
                        path,
                        cost: state.cost,
                    },
                    cost_cmp,
                )));
            });
    }
    panic!("no path found");
}

fn part_two(nums: Vec<Vec<isize>>) -> usize {
    0
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
