use crate::common;
use anyhow::Result;
use regex::Regex;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/3.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)")?;

    let mut doit = true;
    for line in lines {
        let line = line?;
        let line = line.trim();

        for captures in re.captures_iter(line) {
            match &captures[0] {
                "do()" => doit = true,
                "don't()" => doit = false,
                _ if doit => {
                    solution_b += captures[1].parse::<usize>()? * captures[2].parse::<usize>()?
                }
                _ => solution_a += captures[1].parse::<usize>()? * captures[2].parse::<usize>()?,
            }
        }
    }

    solution_a += solution_b;

    Ok((solution_a, solution_b))
}
