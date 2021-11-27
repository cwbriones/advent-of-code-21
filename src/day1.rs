use std::io::BufRead;

use anyhow::anyhow;
use anyhow::Result;

use super::Solution;

pub struct Day1;

impl Default for Day1 {
    fn default() -> Self { Day1 }
}

impl Solution for Day1 {
    type Input = Vec<usize>;
    type Output = usize;

    fn parse(&self, r: impl BufRead) -> Result<Self::Input> {
        r.lines()
            .map(|line| {
                line
                    .map_err(anyhow::Error::from)
                    .and_then(|s|
                        s.parse::<usize>()
                            .map_err(anyhow::Error::from)
                    )
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn part_one(&mut self, nums: Self::Input) -> Result<Self::Output> {
        let len = nums.len();
        for i in 0..len {
            for j in (i+1)..len {
                if nums[i] + nums[j] == 2020 {
                    return Ok(nums[i] * nums[j])
                }
            }
        }
        Err(anyhow!("no solution found"))
    }

    fn part_two(&mut self, nums: Self::Input) -> Result<Self::Output> {
        let len = nums.len();
        for i in 0..len {
            for j in (i+1)..len {
                if nums[i] + nums[j] == 2020 {
                    return Ok(nums[i] * nums[j])
                }
            }
        }
        Err(anyhow!("no solution found"))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::Day1;

    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;

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

        let infile = File::open(&input_path)
            .with_context(|| format!("read: {}", input_path.display()))?;
        let file = BufReader::new(infile);

        let expected_output = std::fs::read_to_string(&output_path)
            .with_context(|| format!("read: {}", output_path.display()))?;

        let mut sol = Day1::default();
        let out = sol.parse_and_run(file)?;

        assert_eq!(out, expected_output.trim_end());

        Ok(())
    }
}
