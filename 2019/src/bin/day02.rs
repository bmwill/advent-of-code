use anyhow::Result;
use itertools::iproduct;

const INPUT: &str = include_str!("../../input/day02.txt");
const TARGET: usize = 19690720;

// Intcode computer
fn run_intcode(program: &mut [usize]) -> usize {
    let mut ip = 0;
    loop {
        let opcode = program[ip];
        let rs = program[ip + 1];
        let rt = program[ip + 2];
        let rd = program[ip + 3];

        match opcode {
            // Add
            1 => {
                program[rd] = program[rs] + program[rt];
            }
            // Mult
            2 => {
                program[rd] = program[rs] * program[rt];
            }
            // Halt
            99 => {
                return program[0];
            }
            _ => unreachable!(),
        }

        ip += 4;
    }
}

fn part2(program: &[usize], target: usize) -> (usize, usize) {
    let mut p = program.to_owned();
    for (noun, verb) in iproduct!(0..=99, 0..=99) {
        p.copy_from_slice(program);
        p[1] = noun;
        p[2] = verb;

        if run_intcode(&mut p) == target {
            return (noun, verb);
        }
    }
    unreachable!();
}

fn main() -> Result<()> {
    let program = INPUT
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;

    // Reset "1202 program alarm" state
    let mut part1 = program.clone();
    part1[1] = 12;
    part1[2] = 2;

    println!("Part 1: {}", run_intcode(&mut part1));

    let (noun, verb) = part2(&program, TARGET);
    println!(
        "Part 2: noun: {}, verb: {}, answer: {}",
        noun,
        verb,
        100 * noun + verb
    );

    Ok(())
}

#[test]
fn example() {
    let mut input = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

    run_intcode(&mut input);

    assert_eq!(input, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
}
