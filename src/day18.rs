use crate::prelude::*;

#[derive(Debug, Clone)]
struct Number(Inner, Inner);

impl Number {
    fn mag(&self) -> usize {
        3 * self.0.mag() + 2 * self.1.mag()
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(1).is_some() {
                continue;
            }
            if self.split().is_some() {
                continue;
            }
            return;
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        // left exploded
        let exp = self.0.explode(depth);
        if let (Some((lefte, righte)), Number(_, right)) = (exp, &mut *self) {
            if righte > 0 {
                right.add_left(righte);
            }
            // println!("left exploded!: {:?}", (lefte, righte));
            return Some((lefte, 0));
        }
        if exp.is_some() {
            return exp;
        }
        // right exploded
        let exp = self.1.explode(depth);
        if let (Some((lefte, righte)), Number(left, _)) = (exp, &mut *self) {
            if lefte > 0 {
                left.add_right(lefte);
            }
            // println!("right exploded!: {:?}", (lefte, righte));
            return Some((0, righte));
        }
        exp
    }

    fn split(&mut self) -> Option<()> {
        let left = self.0.split();
        if left.is_some() {
            return left;
        }
        self.1.split()
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
enum Inner {
    Val(usize),
    Number(Box<Number>),
}

use std::fmt::Display;

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Inner::Val(n) => {
                write!(f, "{}", n)
            }
            Inner::Number(n) => {
                write!(f, "[{},{}]", n.0, n.1)
            }
        }
    }
}

impl Inner {
    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Inner::Number(n) => match &**n {
                Number(Inner::Val(left), Inner::Val(right)) if depth == 4 => {
                    let mut cleared = Inner::Val(0);
                    let retval = (*left, *right);
                    std::mem::swap(&mut cleared, self);
                    Some(retval)
                }
                _ if depth == 4 => panic!("exploding irregular numbers"),
                _ => n.explode(depth + 1),
            },
            Inner::Val(_) => None,
        }
    }

    fn add_right(&mut self, v: usize) {
        match self {
            Inner::Number(n) => {
                let Number(_, right) = &mut **n;
                right.add_right(v);
            }
            Inner::Val(ref mut n) => *n += v,
        }
    }

    fn add_left(&mut self, v: usize) {
        match self {
            Inner::Number(n) => {
                let Number(left, _) = &mut **n;
                left.add_left(v);
            }
            Inner::Val(ref mut n) => *n += v,
        }
    }

    fn mag(&self) -> usize {
        match self {
            Inner::Number(n) => 3 * n.0.mag() + 2 * n.1.mag(),
            Inner::Val(n) => *n,
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Inner::Number(n) => {
                if let s @ Some(_) = n.0.split() {
                    return s;
                }
                n.1.split()
            }
            Inner::Val(n) if *n >= 10 => {
                // println!("SPLITS: {}", n);
                let left = *n / 2;
                let right = left + (*n % 2);
                *self = Inner::Number(Box::new(Number(Inner::Val(left), Inner::Val(right))));
                Some(())
            }
            _ => None,
        }
    }
}

fn parse(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();
    for line in input.lines() {
        let mut iter = line.trim_end().chars().peekable();
        let n = parse_number(&mut iter).unwrap();
        numbers.push(n);
    }
    numbers
}

fn parse_number<I>(s: &mut std::iter::Peekable<I>) -> Result<Number>
where
    I: Iterator<Item = char>,
{
    match s.next() {
        Some('[') => {}
        _ => return Err(anyhow!("expected '['")),
    }
    let left = parse_inner(s).context("left")?;
    match s.next() {
        Some(',') => {}
        _ => return Err(anyhow!("expected ','")),
    }
    let right = parse_inner(s).context("right")?;
    match s.next() {
        Some(']') => {}
        _ => return Err(anyhow!("expected ']'")),
    }
    Ok(Number(left, right))
}

fn parse_inner<I>(s: &mut std::iter::Peekable<I>) -> Result<Inner>
where
    I: Iterator<Item = char>,
{
    match s.peek().cloned() {
        Some('[') => parse_number(s).map(|p| Inner::Number(Box::new(p))),
        Some(c) if c.is_digit(10) => {
            s.next();
            let n = c.to_digit(10).expect("guard");
            Ok(Inner::Val(n as usize))
        }
        _ => Err(anyhow!("expected '[' or 0-9")),
    }
}

fn add(a: Number, b: Number) -> Number {
    let mut sum = Number(Inner::Number(Box::new(a)), Inner::Number(Box::new(b)));
    sum.reduce();
    sum
}

fn part_one(ns: Vec<Number>) -> usize {
    let sum = ns.into_iter().reduce(add).unwrap();
    sum.mag()
}

fn part_two(ns: Vec<Number>) -> usize {
    ns.iter()
        .enumerate()
        .flat_map(|(i, a)| ns[i + 1..].iter().map(move |b| (a, b)))
        .map(|(a, b)| {
            let suma = add(a.clone(), b.clone()).mag();
            let sumb = add(b.clone(), a.clone()).mag();
            suma.max(sumb)
        })
        .max()
        .unwrap()
}

pub fn run(runner: &Runner) {
    runner.run(parse, part_one, part_two);
}
