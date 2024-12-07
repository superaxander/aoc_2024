use crate::common;
use crate::common::Facing;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/6.txt")?;

    let mut grid = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    let mut width = 0;

    for (i, line) in lines.into_iter().enumerate() {
        let line = line?;
        let line = line.trim();

        grid.push(line.chars().collect::<Vec<_>>());
        if let Some(j) = line.chars().position(|c| c == '^') {
            start_x = j;
            start_y = i;
            width = line.len();
        }
    }
    let width = width as i64;
    let height = grid.len() as i64;

    let mut positions = Vec::new();
    let mut facings = Vec::new();
    let mut facing = Facing::North;
    let mut x = start_x as i64;
    let mut y = start_y as i64;

    loop {
        if x < 0 || x >= width || y < 0 || y >= height {
            break;
        }

        if !positions.contains(&(x, y)) {
            positions.push((x, y));
            facings.push(facing);
        }
        do_move(&grid, &mut x, &mut y, &mut facing);
    }

    let solution_a = positions.len();

    let mut solution_b = 0;

    for (((prev_x, prev_y), prev_facing), ((sx, sy), _)) in positions
        .into_iter()
        .zip(facings.into_iter())
        .tuple_windows()
    {
        grid[sy as usize][sx as usize] = '#';
        let mut facing = prev_facing;
        let mut x = prev_x;
        let mut y = prev_y;
        let mut states = HashSet::new();
        let mut exited = false;
        while states.insert((x, y, facing)) {
            if x < 0 || x >= width || y < 0 || y >= height {
                exited = true;
                break;
            }

            do_move(&grid, &mut x, &mut y, &mut facing);
        }

        if !exited {
            solution_b += 1;
        }
        grid[sy as usize][sx as usize] = '.';
    }
    Ok((solution_a, solution_b))
}

fn is_occupied(grid: &[Vec<char>], x: i64, y: i64) -> bool {
    if x < 0 || x >= grid[0].len() as i64 || y < 0 || y >= grid.len() as i64 {
        false
    } else {
        grid[y as usize][x as usize] == '#'
    }
}

fn do_move(grid: &[Vec<char>], x: &mut i64, y: &mut i64, facing: &mut Facing) {
    match facing {
        Facing::North if is_occupied(grid, *x, *y - 1) => {
            *facing = facing.right();
        }
        Facing::East if is_occupied(grid, *x + 1, *y) => {
            *facing = facing.right();
        }
        Facing::South if is_occupied(grid, *x, *y + 1) => {
            *facing = facing.right();
        }
        Facing::West if is_occupied(grid, *x - 1, *y) => {
            *facing = facing.right();
        }
        Facing::North => *y -= 1,
        Facing::East => *x += 1,
        Facing::South => *y += 1,
        Facing::West => *x -= 1,
    }
}
