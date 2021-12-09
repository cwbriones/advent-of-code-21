use std::borrow::Cow;
use std::io::Read;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
    let input_ref = input.as_ref().map(Cow::from);
    return match args.day {
        Some(d) if (1..=25).contains(&d) => dispatch(d, args.part, input_ref.clone()),
        Some(d) => Err(anyhow!("invalid value for day: {}", d)),
        None => (1..25)
            .map(|d| dispatch(d, args.part, input_ref.clone()))
            .collect::<Result<Vec<()>>>()
            .map(|_| ()),
    };
}

fn dispatch(day: usize, part: Option<usize>, input: Option<Cow<str>>) -> Result<()> {
    let cache_key = format!("input/20{}/{}", YEAR, day);
    let input = match input {
        Some(i) => i,
        None => cached(&cache_key, || fetch_input(day))?.into(),
    };
    let runner = Runner { day, part };
    match day {
        1 => day1::run(&input, &runner),
        2 => day2::run(&input, &runner),
        3 => day3::run(&input, &runner),
        4 => day4::run(&input, &runner),
        5 => day5::run(&input, &runner),
        6 => day6::run(&input, &runner),
        7 => day7::run(&input, &runner),
        8 => day8::run(&input, &runner),
        9 => day9::run(&input, &runner),
        d => return Err(anyhow!("day {} is not implemented", d)),
    }
}

pub struct Runner {
    day: usize,
    part: Option<usize>,
}

impl Runner {
    pub fn part_one<F, D>(&self, f: F)
    where
        F: FnOnce() -> D,
        D: std::fmt::Display,
    {
        if let Some(1) | None = self.part {
            self.run_part(1, f);
        }
    }

    pub fn part_two<F, D>(&self, f: F)
    where
        F: FnOnce() -> D,
        D: std::fmt::Display,
    {
        if let Some(2) | None = self.part {
            self.run_part(2, f);
        }
    }

    fn run_part<F, D>(&self, part: usize, f: F)
    where
        F: FnOnce() -> D,
        D: std::fmt::Display,
    {
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
