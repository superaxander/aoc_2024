use crate::common;
use crate::common::Facing;
use anyhow::Result;
use std::collections::HashSet;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/12.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let mut grid = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        grid.push(line.chars().collect::<Vec<_>>());
    }

    let size = grid.len();

    let mut regions = Vec::new();
    let mut inside = Vec::new();
    let mut outside = Vec::new();
    let mut visited = HashSet::new();

    outside.push((0, 0));
    while let Some((x, y)) = outside.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        let current_char = grid[y][x];
        let mut area = 0;
        let mut edges = HashSet::new();
        inside.push((x, y));
        while let Some((x, y)) = inside.pop() {
            if !visited.insert((x, y)) {
                continue;
            }
            area += 1;
            if x > 0 {
                if grid[y][x - 1] == current_char {
                    inside.push((x - 1, y));
                } else {
                    outside.push((x - 1, y));
                    edges.insert((x, y, Facing::West));
                }
            } else {
                edges.insert((x, y, Facing::West));
            }
            if y > 0 {
                if grid[y - 1][x] == current_char {
                    inside.push((x, y - 1));
                } else {
                    outside.push((x, y - 1));
                    edges.insert((x, y, Facing::North));
                }
            } else {
                edges.insert((x, y, Facing::North));
            }
            if x < size - 1 {
                if grid[y][x + 1] == current_char {
                    inside.push((x + 1, y));
                } else {
                    outside.push((x + 1, y));
                    edges.insert((x, y, Facing::East));
                }
            } else {
                edges.insert((x, y, Facing::East));
            }
            if y < size - 1 {
                if grid[y + 1][x] == current_char {
                    inside.push((x, y + 1));
                } else {
                    outside.push((x, y + 1));
                    edges.insert((x, y, Facing::South));
                }
            } else {
                edges.insert((x, y, Facing::South));
            }
        }
        solution_a += area * edges.len();
        regions.push((area, edges));
    }

    for (area, mut edges) in regions {
        let mut sides_a = 0;
        while !edges.is_empty() {
            let (mut x, mut y, mut facing) = edges.iter().copied().next().unwrap();
            let mut first = true;
            loop {
                if first {
                    first = false;
                } else {
                    edges.remove(&(x, y, facing));
                }

                match facing {
                    Facing::North => {
                        if x > 0 && edges.contains(&(x - 1, y, Facing::North)) {
                            x -= 1;
                        } else if y > 0 && x > 0 && edges.contains(&(x - 1, y - 1, Facing::East)) {
                            //  +
                            // O|
                            //  +--
                            //   O
                            sides_a += 1;
                            x -= 1;
                            y -= 1;
                            facing = Facing::East;
                        } else if edges.contains(&(x, y, Facing::West)) {
                            // +--
                            // |O
                            // +
                            sides_a += 1;
                            facing = Facing::West;
                        } else {
                            // Must've reached the start again
                            break;
                        }
                    }
                    Facing::East => {
                        if y > 0 && edges.contains(&(x, y - 1, Facing::East)) {
                            y -= 1;
                        } else if y > 0
                            && x < size - 1
                            && edges.contains(&(x + 1, y - 1, Facing::South))
                        {
                            //   O
                            //  +-+
                            // O|
                            sides_a += 1;
                            x += 1;
                            y -= 1;
                            facing = Facing::South;
                        } else if edges.contains(&(x, y, Facing::North)) {
                            // +-+
                            //  O|
                            sides_a += 1;
                            facing = Facing::North;
                        } else {
                            break;
                        }
                    }
                    Facing::South => {
                        if x < size - 1 && edges.contains(&(x + 1, y, Facing::South)) {
                            x += 1;
                        } else if x < size - 1
                            && y < size - 1
                            && edges.contains(&(x + 1, y + 1, Facing::West))
                        {
                            // O
                            //--+
                            //  |O
                            sides_a += 1;
                            x += 1;
                            y += 1;
                            facing = Facing::West;
                        } else if edges.contains(&(x, y, Facing::East)) {
                            // O|
                            //--+
                            sides_a += 1;
                            facing = Facing::East;
                        } else {
                            break;
                        }
                    }
                    Facing::West => {
                        if y < size - 1 && edges.contains(&(x, y + 1, Facing::West)) {
                            y += 1;
                        } else if y < size - 1
                            && x > 0
                            && edges.contains(&(x - 1, y + 1, Facing::North))
                        {
                            // |O
                            //-+
                            //O
                            sides_a += 1;
                            x -= 1;
                            y += 1;
                            facing = Facing::North;
                        } else if edges.contains(&(x, y, Facing::South)) {
                            // |
                            // |O
                            // +-
                            sides_a += 1;
                            facing = Facing::South;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        solution_b += sides_a * area;
    }

    Ok((solution_a, solution_b))
}
