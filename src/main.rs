use std::io::Read;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

mod day1;

const CACHE_DIR: &str = "/Users/cwbriones/.advent-of-code";

fn cached<F>(
    key: &str,
    f: F,
) -> Result<String>
    where F: Fn() -> Result<String>
{
    let full_path = Path::new(CACHE_DIR).join(key);
    let dir = full_path.parent().expect("non-empty path");

    if full_path.exists() {
        return std::fs::read_to_string(full_path)
            .map_err(Into::into);
    }
    std::fs::create_dir_all(dir)?;
    let content = f().with_context(||
        format!("populate cache entry for {}", full_path.display())
    )?;
    std::fs::write(full_path, &content)?;
    Ok(content)
}

fn fetch_input(
    year: usize,
    day: usize,
) -> Result<String> {
    let token = get_session_token()?;
    let url = format!("https://adventofcode.com/20{}/day/{}/input", year, day);
    let res = ureq::get(&url)
        .set("Cookie", &format!("session={}", token))
        .call()?;
    if res.status() != 200 {
        return Err(anyhow!("unexpected status code: {}", res.status()));
    }
    let mut body = String::new();
    if let Some(size) = res.header("Content-Length") {
        let size = size.parse::<usize>().context("parse content-length")?;
        body.reserve(size);
    }
    res.into_reader().read_to_string(&mut body).context("read response body")?;
    Ok(body)
}

fn get_session_token() -> Result<String> {
    if let Ok(val) = std::env::var("AOC_TOKEN") {
        return Ok(val)
    }
    let full_path = Path::new(CACHE_DIR).join("session");
    if !full_path.exists() {
        return Err(anyhow!("session token not found at '{}'", full_path.display()));
    }
    std::fs::read_to_string(full_path)
        .map_err(Into::into)
}

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    // #[structopt(parse(from_os_str))]
    // input:  Option<PathBuf>,

    #[structopt(short, long, default_value = "21")]
    year: usize,
    #[structopt(short, long, default_value = "1")]
    day: usize,
    #[structopt(short, long)]
    part: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    if args.year != 20 {
        return Err(anyhow!("only year=20 is supported"));
    }
    if args.day < 1 || args.day > 25 {
        return Err(anyhow!("invalid value for day: {}", args.day));
    }
    let cache_key = format!("input-20{}-{}", args.year, args.day);
    let input = cached(
        &cache_key,
        || fetch_input(args.year, args.day),
    )?;
    match args.day {
        1 => day1::run(&input, args.part),
        d => return Err(anyhow!("day {} is not implemented", d)),
    }?;
    Ok(())
}
