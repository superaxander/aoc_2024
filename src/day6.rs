use crate::common;
use crate::common::Facing;
use anyhow::Result;
use bit_vec::BitVec;
use itertools::Itertools;
use std::collections::HashSet;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/6.txt")?;

    let mut grid: Vec<BitVec> = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;

    for (i, line) in lines.into_iter().enumerate() {
        let line = line?;
        let line = line.trim();

        grid.push(line.chars().map(|c| c == '#').collect());
        if let Some(j) = line.chars().position(|c| c == '^') {
            start_x = j;
            start_y = i;
        }
    }
    let size = grid.len() as i64;
    let mut grid = grid.into_iter().flatten().collect();

    let mut positions = Vec::new();
    let mut facings = Vec::new();
    let mut facing = Facing::North;
    let mut x = start_x as i64;
    let mut y = start_y as i64;

    loop {
        if x < 0 || x >= size || y < 0 || y >= size {
            break;
        }

        if !positions.contains(&(x, y)) {
            positions.push((x, y));
            facings.push(facing);
        }
        do_move(&grid, &mut x, &mut y, &mut facing, size);
    }

    let solution_a = positions.len();

    let mut solution_b = 0;

    for (((prev_x, prev_y), prev_facing), ((sx, sy), _)) in positions
        .into_iter()
        .zip(facings.into_iter())
        .tuple_windows()
    {
        grid.set(sy as usize * size as usize + sx as usize, true);
        let mut facing = prev_facing;
        let mut x = prev_x;
        let mut y = prev_y;
        let mut states = HashSet::new();
        let mut exited = false;
        loop {
            if x < 0 || x >= size || y < 0 || y >= size {
                exited = true;
                break;
            }

            if do_move(&grid, &mut x, &mut y, &mut facing, size) && !states.insert((x, y, facing)) {
                break;
            }
        }

        if !exited {
            solution_b += 1;
        }
        grid.set(sy as usize * size as usize + sx as usize, false);
    }
    Ok((solution_a, solution_b))
}

fn is_occupied(grid: &BitVec, x: i64, y: i64, size: i64) -> bool {
    (x >= 0 && x < size && y >= 0 && y < size) && grid[y as usize * size as usize + x as usize]
}

fn do_move(grid: &BitVec, x: &mut i64, y: &mut i64, facing: &mut Facing, size: i64) -> bool {
    match facing {
        Facing::North if is_occupied(grid, *x, *y - 1, size) => {
            *facing = facing.right();
            true
        }
        Facing::East if is_occupied(grid, *x + 1, *y, size) => {
            *facing = facing.right();
            false
        }
        Facing::South if is_occupied(grid, *x, *y + 1, size) => {
            *facing = facing.right();
            false
        }
        Facing::West if is_occupied(grid, *x - 1, *y, size) => {
            *facing = facing.right();
            false
        }
        Facing::North => {
            *y -= 1;
            false
        }
        Facing::East => {
            *x += 1;
            false
        }
        Facing::South => {
            *y += 1;
            false
        }
        Facing::West => {
            *x -= 1;
            false
        }
    }
}
