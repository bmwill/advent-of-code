use anyhow::Result;
use itertools::Itertools;
use std::ops::Range;

const INPUT: &str = include_str!("../../input/day04.txt");

fn range() -> Option<Range<usize>> {
    let input = INPUT.trim();
    let idx = input.find('-')?;
    let start = input[..idx].parse().ok()?;
    let end = input[idx + 1..].parse().ok()?;

    Some(Range { start, end })
}

fn to_digits(i: usize) -> Option<[u8; 6]> {
    let mut digits = [0; 6];
    let mut idx = 0;
    for digit in i.to_string().chars().flat_map(|c| c.to_digit(10)) {
        if idx >= 6 {
            return None;
        }

        digits[idx] = digit as u8;

        idx += 1;
    }

    if idx < 6 {
        return None;
    }

    Some(digits)
}

fn never_decreses(digits: &[u8; 6]) -> bool {
    for (d1, d2) in digits.iter().tuple_windows() {
        if d2 < d1 {
            return false;
        }
    }
    true
}

fn has_double(digits: &[u8; 6]) -> bool {
    for (d1, d2) in digits.iter().tuple_windows() {
        if d1 == d2 {
            return true;
        }
    }
    false
}

fn has_unique_double(digits: &[u8; 6]) -> bool {
    digits.iter().enumerate().any(|(idx, digit)| {
        let count = digits[..idx]
            .iter()
            .rev()
            .take_while(|&d| d == digit)
            .count()
            + digits[idx + 1..].iter().take_while(|&d| d == digit).count();
        count == 1
    })
}

fn part1(range: Range<usize>) -> usize {
    range
        .into_iter()
        .flat_map(to_digits)
        .filter(never_decreses)
        .filter(has_double)
        .count()
}

fn part2(range: Range<usize>) -> usize {
    range
        .into_iter()
        .flat_map(to_digits)
        .filter(never_decreses)
        .filter(has_unique_double)
        .count()
}

fn main() -> Result<()> {
    let range = range().unwrap();
    println!("Part 1: {}", part1(range.clone()));
    println!("Part 2: {}", part2(range));
    Ok(())
}

#[test]
fn example() {}
