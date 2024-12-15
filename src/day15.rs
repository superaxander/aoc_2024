use crate::common;
use crate::common::Facing;
use anyhow::Result;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/15.txt")?;

    let mut grid_a = Vec::new();
    let mut moves = Vec::new();
    let mut grid_done = false;
    let mut start_x = 0;
    let mut start_y = 0;

    for (ly, line) in lines.enumerate() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            grid_done = true;
        } else if grid_done {
            moves.extend(line.chars().map(|c| match c {
                '<' => Facing::West,
                '>' => Facing::East,
                '^' => Facing::North,
                'v' => Facing::South,
                c => panic!("unexpected direction {c}"),
            }));
        } else {
            if let Some(lx) = line.chars().position(|c| c == '@') {
                start_x = lx;
                start_y = ly;
            }
            grid_a.push(
                line.chars()
                    .map(|c| if c == '@' { '.' } else { c })
                    .collect::<Vec<_>>(),
            );
        }
    }

    let mut grid_b = grid_a
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| if *c == 'O' { ['[', ']'] } else { [*c, *c] })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut x = start_x;
    let mut y = start_y;
    for m in &moves {
        match m {
            Facing::North => {
                if move_boxes(&mut grid_a, x, y - 1, 0, -1) {
                    y -= 1;
                }
            }
            Facing::East => {
                if move_boxes(&mut grid_a, x + 1, y, 1, 0) {
                    x += 1;
                }
            }
            Facing::South => {
                if move_boxes(&mut grid_a, x, y + 1, 0, 1) {
                    y += 1;
                }
            }
            Facing::West => {
                if move_boxes(&mut grid_a, x - 1, y, -1, 0) {
                    x -= 1;
                }
            }
        }
    }

    let solution_a = sum_coordinates(&grid_a, 'O');

    let mut x = start_x * 2;
    let mut y = start_y;
    for m in moves {
        match m {
            Facing::North => {
                if check_boxes_vertical(&grid_b, x, y - 1, -1) {
                    move_boxes_vertical(&mut grid_b, x, y - 1, -1);
                    y -= 1;
                }
            }
            Facing::East => {
                if move_boxes_horizontal(&mut grid_b, x + 1, y) {
                    x += 1;
                }
            }
            Facing::South => {
                if check_boxes_vertical(&grid_b, x, y + 1, 1) {
                    move_boxes_vertical(&mut grid_b, x, y + 1, 1);
                    y += 1;
                }
            }
            Facing::West => {
                if move_boxes_horizontal(&mut grid_b, x - 1, y) {
                    x -= 1;
                }
            }
        }
    }

    let solution_b = sum_coordinates(&grid_b, '[');

    Ok((solution_a, solution_b))
}

fn sum_coordinates(grid_b: &[Vec<char>], box_char: char) -> usize {
    let mut solution = 0;
    for (y, row) in grid_b.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == box_char {
                solution += y * 100 + x;
            }
        }
    }
    solution
}

fn move_boxes(grid: &mut [Vec<char>], x: usize, y: usize, dx: isize, dy: isize) -> bool {
    match grid[y][x] {
        'O' => {
            let to_x = x.wrapping_add_signed(dx);
            let to_y = y.wrapping_add_signed(dy);
            if move_boxes(grid, to_x, to_y, dx, dy) {
                grid[to_y][to_x] = 'O';
                grid[y][x] = '.';
                true
            } else {
                false
            }
        }
        c => c == '.',
    }
}

fn move_boxes_horizontal(grid: &mut [Vec<char>], x: usize, y: usize) -> bool {
    match grid[y][x] {
        '[' => {
            if move_boxes_horizontal(grid, x + 2, y) {
                grid[y][x + 1] = '[';
                grid[y][x + 2] = ']';
                grid[y][x] = '.';
                true
            } else {
                false
            }
        }
        ']' => {
            if move_boxes_horizontal(grid, x - 2, y) {
                grid[y][x - 1] = ']';
                grid[y][x - 2] = '[';
                grid[y][x] = '.';
                true
            } else {
                false
            }
        }
        c => c == '.',
    }
}

fn check_boxes_vertical(grid: &[Vec<char>], x: usize, y: usize, dy: isize) -> bool {
    let to_y = y.wrapping_add_signed(dy);
    match grid[y][x] {
        '[' => {
            check_boxes_vertical(grid, x, to_y, dy) && check_boxes_vertical(grid, x + 1, to_y, dy)
        }
        ']' => {
            check_boxes_vertical(grid, x, to_y, dy) && check_boxes_vertical(grid, x - 1, to_y, dy)
        }
        c => c == '.',
    }
}

fn move_boxes_vertical(grid: &mut [Vec<char>], x: usize, y: usize, dy: isize) {
    let to_y = y.wrapping_add_signed(dy);
    match grid[y][x] {
        '[' => {
            move_boxes_vertical(grid, x, to_y, dy);
            move_boxes_vertical(grid, x + 1, to_y, dy);
            grid[y][x] = '.';
            grid[y][x + 1] = '.';
            grid[to_y][x] = '[';
            grid[to_y][x + 1] = ']';
        }
        ']' => {
            move_boxes_vertical(grid, x, to_y, dy);
            move_boxes_vertical(grid, x - 1, to_y, dy);
            grid[y][x] = '.';
            grid[y][x - 1] = '.';
            grid[to_y][x] = ']';
            grid[to_y][x - 1] = '[';
        }
        _ => {}
    }
}
