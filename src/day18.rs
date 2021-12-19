use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Clone)]
struct Number(Inner, Inner);

impl Number {
    fn mag(&self) -> usize {
        3 * self.0.mag() + 2 * self.1.mag()
    }

    fn reduce(&mut self) {
        while self
            .explode(1)
            .map(|_| ())
            .or_else(|| self.split())
            .is_some()
        {}
    }

    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        let exp = self.0.explode(depth);
        if let (Some((a, b)), Number(_, right)) = (exp, &mut *self) {
            // left sibling exploded
            right.add_left(b);
            return Some((a, 0));
        }
        if exp.is_some() {
            return exp;
        }
        let exp = self.1.explode(depth);
        if let (Some((a, b)), Number(left, _)) = (exp, &mut *self) {
            // right sibling exploded
            left.add_right(a);
            return Some((0, b));
        }
        exp
    }

    fn split(&mut self) -> Option<()> {
        if let left @ Some(_) = self.0.split() {
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

fn add(a: Number, b: Number) -> Number {
    let mut sum = Number(Inner::Number(Box::new(a)), Inner::Number(Box::new(b)));
    sum.reduce();
    sum
}

#[derive(Debug, Clone)]
enum Inner {
    Val(usize),
    Number(Box<Number>),
}

impl Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Inner::Val(n) => write!(f, "{}", n),
            Inner::Number(n) => write!(f, "[{},{}]", n.0, n.1),
        }
    }
}

impl Inner {
    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        match self {
            Inner::Number(n) => match (&n.0, &n.1) {
                (&Inner::Val(left), &Inner::Val(right)) if depth == 4 => {
                    let mut cleared = Inner::Val(0);
                    let retval = (left, right);
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
            Inner::Number(n) => n.1.add_right(v),
            Inner::Val(ref mut n) => *n += v,
        }
    }

    fn add_left(&mut self, v: usize) {
        match self {
            Inner::Number(n) => n.0.add_left(v),
            Inner::Val(ref mut n) => *n += v,
        }
    }

    fn mag(&self) -> usize {
        match self {
            Inner::Number(n) => n.mag(),
            Inner::Val(n) => *n,
        }
    }

    fn split(&mut self) -> Option<()> {
        match self {
            Inner::Number(n) => n.split(),
            Inner::Val(n) if *n >= 10 => {
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
    input
        .lines()
        .map(|line| {
            let mut iter = line.trim_end().chars().peekable();
            parse_number(&mut iter).unwrap()
        })
        .collect()
}

fn parse_number<I>(s: &mut std::iter::Peekable<I>) -> Result<Number>
where
    I: Iterator<Item = char>,
{
    expect_char(s, '[')?;
    let left = parse_inner(s).context("left")?;
    expect_char(s, ',')?;
    let right = parse_inner(s).context("right")?;
    expect_char(s, ']')?;
    Ok(Number(left, right))
}

fn expect_char<I: Iterator<Item = char>>(iter: &mut I, c: char) -> Result<()> {
    iter.next()
        .ok_or_else(|| anyhow!("unexpected eof"))
        .and_then(|c2| {
            if c == c2 {
                Ok(())
            } else {
                Err(anyhow!("expected '{}', got '{}'", c, c2))
            }
        })
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
