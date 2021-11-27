use anyhow::Result;

fn parse(r: &str) -> Result<Vec<usize>> {
    todo!();
}

fn part_one(nums: &[usize]) -> usize {
    todo!();
}

fn part_two(nums: &[usize]) -> usize {
    todo!();
}

//
// Everything below this point can be moved into a template
//

pub fn run(
    input: &str,
    part: Option<usize>
) -> Result<()> {
    let input = parse(input)?;
    if let Some(1) | None = part {
        let out = part_one(&input);
        println!("Day <AOCDAY> - Part 1: {}", out);
    }
    if let Some(2) | None = part {
        let out = part_two(&input);
        println!("Day <AOCDAY> - Part 2: {}", out);
    }
    Ok(())
}
