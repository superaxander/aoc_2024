use crate::common;
use anyhow::Result;

pub fn main() -> Result<(i64, i64)> {
    let lines = common::read_lines("inputs/7.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (test_value, operands) = line.split_once(": ").unwrap();
        let test_value = test_value.parse::<i64>()?;
        let operands = operands
            .split(' ')
            .map(str::parse)
            .collect::<std::result::Result<Vec<_>, _>>()?;
        if test_operands(test_value, &operands, false) {
            solution_a += test_value;
        } else if test_operands(test_value, &operands, true) {
            solution_b += test_value;
        }
    }

    solution_b += solution_a;

    Ok((solution_a, solution_b))
}

fn test_operands(test_value: i64, operands: &[i64], do_b: bool) -> bool {
    let operand = operands[operands.len() - 1];
    if operands.len() == 1 {
        return test_value == operand;
    }

    (test_value % operand == 0
        && test_operands(test_value / operand, &operands[..operands.len() - 1], do_b))
        || test_operands(test_value - operand, &operands[..operands.len() - 1], do_b)
        || (do_b && {
            let factor = 10i64.pow(operand.ilog10() + 1);
            test_value % factor == operand
                && test_operands(test_value / factor, &operands[..operands.len() - 1], do_b)
        })
}
