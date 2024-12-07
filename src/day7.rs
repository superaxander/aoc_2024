use crate::common;
use anyhow::Result;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/7.txt")?;

    let mut solution_a = 0;
    let mut solution_b = 0;

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (test_value, operands) = line.split_once(": ").unwrap();
        let test_value = test_value.parse::<usize>()?;
        let operands = operands
            .split(" ")
            .map(|operand| operand.parse::<usize>())
            .collect::<std::result::Result<Vec<_>, _>>()?;
        if test_operands(test_value, operands[0], &operands[1..], false) {
            solution_a += test_value;
        } else if test_operands(test_value, operands[0], &operands[1..], true) {
            solution_b += test_value;
        }
    }

    solution_b += solution_a;

    Ok((solution_a, solution_b))
}

fn test_operands(test_value: usize, current_value: usize, operands: &[usize], do_b: bool) -> bool {
    if current_value > test_value {
        return false;
    }
    if operands.is_empty() {
        return current_value == test_value;
    }

    test_operands(
        test_value,
        current_value + operands[0],
        &operands[1..],
        do_b,
    ) || test_operands(
        test_value,
        current_value * operands[0],
        &operands[1..],
        do_b,
    ) || (do_b
        && test_operands(
            test_value,
            current_value * 10usize.pow(operands[0].ilog10() + 1) + operands[0],
            &operands[1..],
            do_b,
        ))
}
