use advent_of_code_2019::{IntcodeComputer, IntcodeError};
use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day07.txt");

fn part1(program: &[i32]) -> Result<()> {
    let mut computer = IntcodeComputer::new(&program);

    let max = [0, 1, 2, 3, 4]
        .iter()
        .permutations(5)
        .map(|perm| {
            let mut prev_out = 0;
            for i in perm {
                computer.reset(program);
                computer.input(*i);
                computer.input(prev_out);
                computer.run().unwrap();
                prev_out = computer.output()[0];
            }
            prev_out
        })
        .max()
        .unwrap();
    println!("Part 1: {}", max);
    Ok(())
}

fn part2(program: &[i32]) -> Result<()> {
    let mut computer = vec![IntcodeComputer::new(&program); 5];

    let max = (5..=9)
        .into_iter()
        .permutations(5)
        .map(|perm| {
            perm.into_iter()
                .zip(computer.iter_mut())
                .for_each(|(i, c)| {
                    c.reset(program);
                    c.input(i);
                });

            let mut prev_out = 0;
            while computer.iter().fold(false, |acc, c| acc || c.status()) {
                for c in &mut computer {
                    c.input(prev_out);
                    match c.run() {
                        Ok(()) => {}
                        Err(IntcodeError::WaitingForInput) => {}
                        Err(e) => panic!("error: {:?}", e),
                    }

                    prev_out = *c.output().last().unwrap();
                }
            }
            prev_out
        })
        .max()
        .unwrap();
    println!("Part 2: {}", max);
    Ok(())
}

fn main() -> Result<()> {
    let program = INPUT
        .trim()
        .split(',')
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;

    part1(&program)?;
    part2(&program)?;

    Ok(())
}
