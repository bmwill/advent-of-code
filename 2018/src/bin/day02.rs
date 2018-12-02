use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let input = parse_input()?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn parse_input() -> Result<Vec<String>> {
    let mut stdin = String::new();
    io::stdin().read_to_string(&mut stdin)?;

    let input = stdin.lines().map(|line| line.to_owned()).collect();

    Ok(input)
}

fn part1(ids: &[String]) -> Result<()> {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let mut freq: HashMap<char, i32> = HashMap::new();

        id.chars().for_each(|c| *freq.entry(c).or_default() += 1);

        twos += freq.values().any(|count| *count == 2) as i32;
        threes += freq.values().any(|count| *count == 3) as i32;
    }

    println!("checksum: '{}'", twos * threes);

    Ok(())
}

fn part2(ids: &[String]) -> Result<()> {
    for i in 0..ids.len() {
        let a = &ids[i];
        for j in i + 1..ids.len() {
            let b = &ids[j];

            let differences = a
                .chars()
                .zip(b.chars())
                .fold(0, |acc, (a, b)| (a != b) as i32 + acc);

            if differences == 1 {
                let letters: String = a
                    .chars()
                    .zip(b.chars())
                    .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                    .collect();

                println!("common letters: '{}'", letters);
                return Ok(());
            }
        }
    }
    Ok(())
}
