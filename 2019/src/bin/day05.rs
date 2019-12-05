use advent_of_code_2019::IntcodeComputer;
use anyhow::Result;

const INPUT: &str = include_str!("../../input/day05.txt");

fn part1(mut program: Vec<i32>) -> Result<()> {
    let mut computer = IntcodeComputer::new(&mut program);
    computer.input(1);
    computer.run()?;
    println!("Part 1: {:?}", computer.output());
    Ok(())
}

fn part2(mut program: Vec<i32>) -> Result<()> {
    let mut computer = IntcodeComputer::new(&mut program);
    computer.input(5);
    computer.run()?;
    println!("Part 1: {:?}", computer.output());
    Ok(())
}

fn main() -> Result<()> {
    let program = INPUT
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;

    part1(program.clone())?;
    part2(program)?;

    Ok(())
}
