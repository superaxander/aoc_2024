use crate::common;
use anyhow::Result;
use rayon::yield_now;
use std::collections::HashSet;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/10.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut grid = Vec::new();
    let mut trailheads = Vec::new();

    for (y, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();

        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '0' {
                trailheads.push((x, y));
            }

            row.push(c as usize - '0' as usize);
        }
        grid.push(row);
    }

    let mut size = grid.len();

    for (x, y) in trailheads {
        let mut position = vec![(x, y, 0)];
        let mut visited = HashSet::new();

        while let Some((x, y, last_height)) = position.pop() {
            if last_height == 9 {
                if visited.insert((x, y)) {
                    solution_a += 1;
                }
                solution_b += 1;
                continue;
            }

            if x > 0 && grid[y][x - 1] == last_height + 1 {
                position.push((x - 1, y, last_height + 1));
            }
            if y > 0 && grid[y - 1][x] == last_height + 1 {
                position.push((x, y - 1, last_height + 1));
            }
            if x < size - 1 && grid[y][x + 1] == last_height + 1 {
                position.push((x + 1, y, last_height + 1));
            }
            if y < size - 1 && grid[y + 1][x] == last_height + 1 {
                position.push((x, y + 1, last_height + 1));
            }
        }
    }

    Ok((solution_a, solution_b))
}
