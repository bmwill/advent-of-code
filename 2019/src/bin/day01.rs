use anyhow::Result;
use std::iter;

const INPUT: &str = include_str!("../../input/day01.txt");

fn main() -> Result<()> {
    let mass = INPUT
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;
    let fuel_sum: u64 = mass.iter().map(|mass| (mass / 3) - 2).sum();
    println!("fuel_sum: {}", fuel_sum);

    let real_fuel_sum: u64 = mass
        .iter()
        .flat_map(|&mass| iter::successors(Some(mass), |fuel| (fuel / 3).checked_sub(2)).skip(1))
        .sum();
    println!("real_fuel_sum: {}", real_fuel_sum);
    Ok(())
}
