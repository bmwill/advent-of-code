use advent_of_code_2019::IntcodeComputer;
use anyhow::Result;

const INPUT: &str = include_str!("../../input/day09.txt");

fn part1(program: &[i64]) -> Result<()> {
    let mut computer = IntcodeComputer::new(&program);
    computer.input(1);
    computer.run()?;
    println!("Part 1: {:?}", computer.output());
    Ok(())
}

fn part2(program: &[i64]) -> Result<()> {
    let mut computer = IntcodeComputer::new(&program);
    computer.input(2);
    computer.run()?;
    println!("Part 2: {:?}", computer.output());
    Ok(())
}

fn main() -> Result<()> {
    let program = INPUT
        .trim()
        .split(',')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    part1(&program)?;
    part2(&program)?;

    Ok(())
}

#[test]
fn quine() {
    let input = [
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];

    let mut computer = IntcodeComputer::new(&input);
    computer.run().unwrap();
    assert_eq!(computer.output(), input);
}
