use anyhow::Result;

fn parse(r: &str) -> Result<Vec<usize>> {
    r.lines()
        .map(|line| {
            line.parse::<usize>()
                .map_err(anyhow::Error::from)
        })
        .collect::<Result<Vec<_>, _>>()
}

fn part_one(nums: &[usize]) -> usize {
    let len = nums.len();
    for i in 0..len {
        for j in (i+1)..len {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j]
            }
        }
    }
    panic!("no solution found");
}

fn part_two(nums: &[usize]) -> usize {
    let len = nums.len();
    for i in 0..len {
        for j in (i+1)..len {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j]
            }
        }
    }
    panic!("no solution found");
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
        println!("Day 1 - Part 1: {}", out);
    }
    if let Some(2) | None = part {
        let out = part_two(&input);
        println!("Day 1 - Part 2: {}", out);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Context;
    use anyhow::Result;

    // example-2021-1-1.in
    // example-2021-1-1.out
    //
    // test_example!(2021, 1, 1);
    #[test]
    fn test_2021_day_1_example_1() -> Result<()> {
        let input_filename = format!("example-{}-{}-{}.in", 2021, 1, 1);
        let output_filename = format!("example-{}-{}-{}.out", 2021, 1, 1);

        let input_path = Path::new("tests").join(input_filename);
        let output_path = Path::new("tests").join(output_filename);

        let input = std::fs::read_to_string(&input_path)
            .with_context(|| format!("read: {}", input_path.display()))?;

        let expected_output = std::fs::read_to_string(&output_path)
            .with_context(|| format!("read: {}", output_path.display()))?;

        let parsed = super::parse(&input)?;
        let out = super::part_one(&parsed).to_string();

        assert_eq!(out, expected_output.trim_end());

        Ok(())
    }
}
