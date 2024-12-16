use crate::common;
use anyhow::Result;
use tracing::{debug, enabled, Level};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}
pub fn main() -> Result<(usize, usize)> {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const MID_X: i32 = WIDTH / 2;
    const MID_Y: i32 = HEIGHT / 2;

    let lines = common::read_lines("inputs/14.txt")?;

    let mut robots = Vec::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (start, line) = line.split_once(',').unwrap();
        let px = start[2..].parse::<i32>()?;
        let (py, line) = line.split_once(' ').unwrap();
        let py = py.parse::<i32>()?;
        let (start, vy) = line.split_once(',').unwrap();
        let vx = start[2..].parse::<i32>()?;
        let vy = vy.parse::<i32>()?;
        robots.push(Robot { px, py, vx, vy });
    }

    for _ in 0..100 {
        for robot in &mut robots {
            robot.px = (robot.px + robot.vx).rem_euclid(WIDTH);
            robot.py = (robot.py + robot.vy).rem_euclid(HEIGHT);
        }
    }

    let mut q0 = 0;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    for robot in &robots {
        match (robot.px, robot.py) {
            (MID_X, _) | (_, MID_Y) => {}
            (x, y) if x < MID_X && y < MID_Y => q0 += 1,
            (x, y) if x > MID_X && y < MID_Y => q1 += 1,
            (x, y) if x < MID_X && y > MID_Y => q2 += 1,
            (_, _) => q3 += 1,
        }
    }

    let solution_a = q0 * q1 * q2 * q3;
    let mut solution_b = 0;

    for i in 101.. {
        for robot in &mut robots {
            robot.px = (robot.px + robot.vx).rem_euclid(WIDTH);
            robot.py = (robot.py + robot.vy).rem_euclid(HEIGHT);
        }
        if robots
            .iter()
            .enumerate()
            .map(|(i, r)| {
                robots
                    .iter()
                    .skip(i + 1)
                    .filter(|r2| {
                        (r.py == r2.py) && (r.px.abs_diff(r2.px) == 1)
                            || (r.px == r2.px) && (r.py.abs_diff(r2.py) == 1)
                    })
                    .count()
            })
            .filter(|n| *n > 2)
            .count()
            > 50
        {
            solution_b = i;
            if enabled!(Level::DEBUG) {
                let mut string = String::new();
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        if robots.iter().any(|robot| robot.px == x && robot.py == y) {
                            string.push('#');
                        } else {
                            string.push('.');
                        }
                    }
                    string.push('\n');
                }
                debug!("{string}");
            }
            break;
        }
    }

    Ok((solution_a, solution_b))
}
