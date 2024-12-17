use crate::common;
use anyhow::Result;
use itertools::Itertools;
use z3::ast::{Ast, BV};
use z3::{Config, Context, SatResult, Solver};

pub fn main() -> Result<(String, u64)> {
    let mut lines = common::read_lines("inputs/17.txt")?;

    let r_a = lines
        .next()
        .unwrap()?
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()?;
    let r_b = lines
        .next()
        .unwrap()?
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()?;
    let r_c = lines
        .next()
        .unwrap()?
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()?;
    assert!(lines.next().unwrap()?.trim().is_empty());
    let instructions = lines.next().unwrap()?.trim()[9..]
        .split(',')
        .map(str::parse)
        .collect::<std::result::Result<Vec<_>, _>>()?;

    let solution_a = run(r_a, r_b, r_c, &instructions);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let r_a = (0..instructions.len())
        .map(|i| BV::new_const(&ctx, format!("r_a_{i}"), 64))
        .collect::<Vec<_>>();
    let solver = Solver::new(&ctx);
    let seven = &BV::from_u64(&ctx, 7, 64);
    let three = &BV::from_u64(&ctx, 3, 64);

    for (i, instruction) in instructions.into_iter().enumerate().dropping_back(1) {
        let r_b = &r_a[i].bvand(seven);
        solver.assert(
            &BV::from_u64(&ctx, instruction as u64, 64)
                ._eq(&(r_a[i].bvlshr(&r_b.bvxor(three)) & seven ^ r_b)),
        );
        solver.assert(&r_a[i + 1]._eq(&r_a[i].bvlshr(three)));
    }

    assert_eq!(solver.check(), SatResult::Sat);
    let solution_b = solver
        .get_model()
        .unwrap()
        .get_const_interp(&r_a[0])
        .unwrap()
        .as_u64()
        .unwrap();

    Ok((solution_a, solution_b))
}

fn run(mut r_a: usize, mut r_b: usize, mut r_c: usize, instructions: &[usize]) -> String {
    let mut result = String::new();

    let mut rip = 0;

    macro_rules! combo {
        ($idx:expr) => {
            match instructions[$idx] {
                0..=3 => instructions[$idx],
                4 => r_a,
                5 => r_b,
                6 => r_c,
                _ => panic!("Not a 3-bit value"),
            }
        };
    }

    while rip < instructions.len() {
        match instructions[rip] {
            0 => {
                // adv
                r_a /= 2usize.pow(combo!(rip + 1) as u32);
            }
            1 => {
                // bxl(
                r_b ^= instructions[rip + 1];
            }
            2 => {
                // bst
                r_b = combo!(rip + 1) % 8;
            }
            3 => {
                // jnz
                if r_a != 0 {
                    rip = instructions[rip + 1];
                    continue;
                }
            }
            4 => {
                // bxc
                r_b ^= r_c;
            }
            5 => {
                // out
                result.push_str(&format!("{},", combo!(rip + 1) % 8));
            }
            6 => {
                // bdv
                r_b = r_a / 2usize.pow(combo!(rip + 1) as u32);
            }
            7 => {
                // cdv
                r_c = r_a / 2usize.pow(combo!(rip + 1) as u32);
            }
            _ => panic!("Not a 3-bit number"),
        }
        rip += 2;
    }

    result.pop();

    result
}
