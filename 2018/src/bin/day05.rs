use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(input.as_bytes())?;
    part2(input.as_bytes())?;

    Ok(())
}

fn part1(input: &[u8]) -> Result<()> {
    println!("polymer len: {}", reacted_polymer_len(input));

    Ok(())
}

fn reacts(a: u8, b: u8) -> bool {
    (a != b) && (a.to_ascii_lowercase() == b.to_ascii_lowercase())
}

fn reacted_polymer_len(unreacted_polymer: &[u8]) -> usize {
    let mut polymer: Vec<u8> = Vec::new();
    let mut prev_char = None;

    for curr_char in unreacted_polymer {
        if prev_char.is_none() {
            prev_char = polymer.pop();
        }

        if let Some(c) = prev_char {
            if reacts(c, *curr_char) {
                prev_char = None;
            } else {
                polymer.push(c);
                prev_char = Some(*curr_char);
            }
        } else {
            prev_char = Some(*curr_char);
        }
    }

    polymer.len()
}

fn part2(input: &[u8]) -> Result<()> {
    let min_len = (b'a'..b'z')
        .into_iter()
        .map(|c| {
            let polymer: Vec<u8> = input
                .iter()
                .filter(|&&p| (p != c) && (p != c.to_ascii_uppercase()))
                .map(|c| *c)
                .collect();
            reacted_polymer_len(&polymer)
        })
        .min();

    if let Some(len) = min_len {
        println!("Min polymer len: {}", len);
    } else {
        return Err(From::from("Error finding min len"));
    }

    Ok(())
}
