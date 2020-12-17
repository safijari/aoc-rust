#[macro_use]
use anyhow::{Context, Result, Error, anyhow};
use std::collections::HashSet;

#[cfg(test)]
#[test]
fn test_day8() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(
        run_machine(&Op::from_input(input)).unwrap(),
        TerminalCond::InfiniteLoop(5)
    )
}

fn test_day8_part2() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(fix_code(&Op::from_input(input)).unwrap(), 8)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, PartialEq)]
enum TerminalCond {
    InfiniteLoop(i32),
    Correct(i32),
}

impl Op {
    fn from_line(input: &str) -> Result<Op> {
        let input: Vec<_> = input.split(" ").collect();
        let op = input.get(0).context("couldn't get op")?;
        let val: i32 = input.get(1).context("couldn't get value")?.parse()?;
        return match *op {
            "acc" => Ok(Op::Acc(val)),
            "nop" => Ok(Op::Nop(val)),
            "jmp" => Ok(Op::Jmp(val)),
            _ => Err(anyhow!("hey")),
        };
    }

    fn from_input(input: &str) -> Vec<Op> {
        let ops: Vec<_> = input
            .lines()
            .filter_map(|l| Op::from_line(l).ok())
            .collect();
        ops
    }
}

fn fix_code(ops: &Vec<Op>) -> Result<i32> {
    for (idx, op) in ops.iter().enumerate() {
        let patched_op = match op {
            Op::Nop(val) => Op::Jmp(*val),
            Op::Jmp(val) => Op::Nop(*val),
            _ => continue,
        };
        let new_ops: Vec<_> = ops
            .iter()
            .enumerate()
            .map(|(idx2, op2)| if idx2 == idx { patched_op } else { *op2 })
            .collect();

        return match run_machine(&new_ops) {
            Ok(TerminalCond::Correct(val)) => Ok(val),
            _ => continue,
        };
    }
    Err(anyhow!("could not fix code"))
}

fn run_machine(ops: &Vec<Op>) -> Result<TerminalCond> {
    let mut acc = 0;
    let mut idx: i32 = 0;
    let mut visited = HashSet::new();

    loop {
        let op = &ops[idx as usize];
        if visited.contains(&idx) {
            return Ok(TerminalCond::InfiniteLoop(acc));
        }
        visited.insert(idx);
        match op {
            Op::Nop(_) => idx += 1,
            Op::Acc(val) => {
                acc += val;
                idx += 1
            }
            Op::Jmp(val) => idx += val,
            _ => {}
        }

        if idx as usize >= ops.len() {
            return Ok(TerminalCond::Correct(acc));
        }
    }
}

fn main() {
    let input = include_str!("day8_input");
    println!("{:?}", run_machine(&Op::from_input(&input)).unwrap());
    println!("{}", fix_code(&Op::from_input(input)).unwrap())
}
