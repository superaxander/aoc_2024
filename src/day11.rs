use crate::common;
use anyhow::Result;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/11.txt")?;


    let mut stones = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        stones.extend(line.split(' ').map(|x| x.parse::<usize>().unwrap()));
    }

    let mut stones2 = Vec::new();
    for _ in 0..25 {
        for stone in stones.drain(0..) {
            match stone {
                0 => stones2.push(1),
                s if (s.ilog10() + 1) % 2 == 0 => {
                    let factor = 10usize.pow((s.ilog10() + 1) / 2);
                    stones2.push(s / factor);
                    stones2.push(s % factor);
                }
                s => stones2.push(s * 2024),
            }
        }
        std::mem::swap(&mut stones, &mut stones2);
    }
    let solution_a = stones.len();
    let mut solution_b = 0;
    let mut known = HashMap::new();
    for stone in stones {
        solution_b += iterate(50, stone, &mut known);
    }

    Ok((solution_a, solution_b))
}

fn iterate(i: usize, stone: usize, known: &mut HashMap<(usize, usize), usize>) -> usize {
    if i == 0 {
        return 1;
    }

    if known.contains_key(&(i, stone)) {
        known[&(i, stone)]
    } else {
        match stone {
            0 => {
                let result = iterate(i - 1, 1, known);
                known.insert((i, stone), result);
                result
            }
            s if (s.ilog10() + 1) % 2 == 0 => {
                let factor = 10usize.pow((s.ilog10() + 1) / 2);
                let mut result = iterate(i - 1, s / factor, known);
                result += iterate(i - 1, s % factor, known);
                known.insert((i, stone), result);
                result
            }
            s => {
                let result = iterate(i - 1, s * 2024, known);
                known.insert((i, stone), result);
                result
            }
        }
    }
}
