use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let input = parse_input()?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_input() -> Result<Vec<i32>> {
    let mut out = Vec::new();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    for line in input.lines() {
        let n = line.parse()?;
        out.push(n);
    }

    Ok(out)
}

fn part1(input: &[i32]) -> Result<()> {
    let mut freq = 0;

    for n in input {
        freq += n;
    }

    println!("freq: {}", freq);
    Ok(())
}

fn part2(input: &[i32]) -> Result<()> {
    let mut freq = 0;
    let mut seen = HashSet::new();

    loop {
        for n in input {
            freq += n;
            if !seen.insert(freq) {
                println!("dup: {}", freq);
                return Ok(());
            }
        }
    }
}
