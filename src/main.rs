use std::io::Read;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod iter;
mod search;

mod prelude {
    use std::str::FromStr;

    pub use anyhow::{
        anyhow,
        Context,
        Result,
    };
    pub use fxhash::FxHashMap as HashMap;
    pub use fxhash::FxHashSet as HashSet;

    pub use crate::Runner;

    pub fn parse_split<T>(nums: &str, delim: char) -> Result<Vec<T>, <T as FromStr>::Err>
    where
        T: FromStr,
    {
        nums.split(delim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<T>())
            .collect()
    }
}

const CACHE_DIR: &str = "/Users/cwbriones/.advent-of-code";
const YEAR: usize = 21;

fn cached<F>(key: &str, f: F) -> Result<String>
where
    F: Fn() -> Result<String>,
{
    let full_path = Path::new(CACHE_DIR).join(key);
    let dir = full_path.parent().expect("non-empty path");

    if full_path.exists() {
        return std::fs::read_to_string(full_path).map_err(Into::into);
    }
    std::fs::create_dir_all(dir)?;
    let content =
        f().with_context(|| format!("populate cache entry for {}", full_path.display()))?;
    std::fs::write(full_path, &content)?;
    Ok(content)
}

fn fetch_input(day: usize) -> Result<String> {
    let token = get_session_token()?;
    let url = format!("https://adventofcode.com/20{}/day/{}/input", YEAR, day);
    let res = ureq::get(&url)
        .set("Cookie", &format!("session={}", token.trim()))
        .call()?;
    if res.status() != 200 {
        return Err(anyhow!("unexpected status code: {}", res.status()));
    }
    let mut body = String::new();
    if let Some(size) = res.header("Content-Length") {
        let size = size.parse::<usize>().context("parse content-length")?;
        body.reserve(size);
    }
    res.into_reader()
        .read_to_string(&mut body)
        .context("read response body")?;
    Ok(body)
}

fn get_session_token() -> Result<String> {
    if let Ok(val) = std::env::var("AOC_TOKEN") {
        return Ok(val);
    }
    let full_path = Path::new(CACHE_DIR).join("session");
    if !full_path.exists() {
        return Err(anyhow!(
            "session token not found at '{}'",
            full_path.display()
        ));
    }
    std::fs::read_to_string(full_path).map_err(Into::into)
}

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    day: Option<usize>,
    #[structopt(short, long)]
    part: Option<usize>,
    #[structopt(short, long)]
    input: Option<String>,
}

fn read_input(path: &str) -> Result<String> {
    if path == "-" {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read_to_string(path).map_err(Into::into)
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    if let Some(p) = args.part {
        if p != 1 && p != 2 {
            return Err(anyhow!("invalid value for part: {}", p));
        }
    }
    let input = if let Some(ref path) = args.input {
        Some(read_input(path)?)
    } else {
        None
    };
    return match args.day {
        Some(d) if (1..=25).contains(&d) => dispatch(d, args.part, input),
        Some(d) => Err(anyhow!("invalid value for day: {}", d)),
        None => (1..25)
            .map(|d| dispatch(d, args.part, None))
            .collect::<Result<Vec<()>>>()
            .map(|_| ()),
    };
}

fn dispatch(day: usize, part: Option<usize>, input: Option<String>) -> Result<()> {
    let entry_points = [
        day1::run,
        day2::run,
        day3::run,
        day4::run,
        day5::run,
        day6::run,
        day7::run,
        day8::run,
        day9::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
        day21::run,
        day22::run,
        day23::run,
        day24::run,
        day25::run,
    ];
    let run = match entry_points.get(day - 1) {
        Some(r) => r,
        None => return Ok(()),
    };
    let cache_key = format!("input/20{}/{}", YEAR, day);
    let input = match input {
        Some(i) => i,
        None => cached(&cache_key, || fetch_input(day))?,
    };
    let runner = Runner {
        input: input.trim(),
        day,
        part,
    };
    run(&runner);
    Ok(())
}

pub struct Runner<'a> {
    input: &'a str,
    day: usize,
    part: Option<usize>,
}

impl<'a> Runner<'a> {
    pub fn run<I, P, F1, F2>(&self, parse: P, part_one: F1, part_two: F2)
    where
        I: Clone,
        P: Fn(&'a str) -> I,
        F1: Fn(I) -> usize,
        F2: Fn(I) -> usize,
    {
        let i = parse(self.input);
        self.run_part(1, || part_one(i.clone()));
        self.run_part(2, || part_two(i));
    }

    fn run_part<F>(&self, part: usize, f: F)
    where
        F: FnOnce() -> usize,
    {
        match self.part {
            Some(p) if p != part => return,
            _ => {}
        }
        use std::time::Instant;

        let clock = Instant::now();
        let output = f();
        let elapsed = clock.elapsed();
        if self.day < 10 {
            print!(" ");
        }
        println!("Day {}, Part {}: {}", self.day, part, output);
        println!("                {}", display_duration(elapsed));
        println!();
    }
}

fn display_duration(duration: std::time::Duration) -> String {
    let val = duration.as_nanos();
    let mut divisor = 1;

    let mut unit = "ns";
    let units: &[&str] = &["µs", "ms", "s"];

    for u in units {
        if val / divisor < 1000 {
            break;
        }
        divisor *= 1000;
        unit = u;
    }
    return format!("{}{}", val / divisor, unit);
}
