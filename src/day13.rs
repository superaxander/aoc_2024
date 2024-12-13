use crate::common;
use anyhow::Result;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize, SatResult};

pub fn main() -> Result<(u64, u64)> {
    let mut lines = common::read_lines("inputs/13.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    let cfg = Config::default();
    let ctx = Context::new(&cfg);
    let solver = Optimize::new(&ctx);
    let count_a = Int::new_const(&ctx, "count_a");
    let count_b = Int::new_const(&ctx, "count_b");
    let min_term = &count_a * 3u64 + &count_b;
    solver.minimize(&min_term);

    while let Some(line) = lines.next() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let (_, line) = line.split_once('+').unwrap();
        let (x, line) = line.split_once(',').unwrap();
        let (_, y) = line.split_once('+').unwrap();
        let ax = x.parse::<u64>()?;
        let ay = y.parse::<u64>()?;

        let line = lines.next().unwrap();
        let line = line?;
        let line = line.trim();

        let (_, line) = line.split_once('+').unwrap();
        let (x, line) = line.split_once(',').unwrap();
        let (_, y) = line.split_once('+').unwrap();
        let bx = x.parse::<u64>()?;
        let by = y.parse::<u64>()?;

        let line = lines.next().unwrap();
        let line = line?;
        let line = line.trim();

        let (_, line) = line.split_once('=').unwrap();
        let (x, line) = line.split_once(',').unwrap();
        let (_, y) = line.split_once('=').unwrap();
        let goal_x = x.parse::<u64>()?;
        let goal_y = y.parse::<u64>()?;

        if solver.check(&[
            Int::from_u64(&ctx, goal_x)._eq(&(&count_a * ax + &count_b * bx)),
            Int::from_u64(&ctx, goal_y)._eq(&(&count_a * ay + &count_b * by)),
        ]) == SatResult::Sat
        {
            let model = solver.get_model().unwrap();
            solution_a += model.eval(&min_term, true).unwrap().as_u64().unwrap();
        }

        if solver.check(&[
            Int::from_u64(&ctx, goal_x + 10_000_000_000_000)._eq(&(&count_a * ax + &count_b * bx)),
            Int::from_u64(&ctx, goal_y + 10_000_000_000_000)._eq(&(&count_a * ay + &count_b * by)),
        ]) == SatResult::Sat
        {
            let model = solver.get_model().unwrap();
            solution_b += model.eval(&min_term, true).unwrap().as_u64().unwrap();
        }
    }

    Ok((solution_a, solution_b))
}
