use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut claims: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut all_ids = HashSet::new();

    for line in input.lines() {
        let (id, (x, y), (w, h)) = process_input_line(line)?;
        //println!("({}, ({}, {}), ({}, {}))", id, x, y, w, h);

        for x in x..x + w {
            for y in y..y + h {
                claims.entry((x, y)).or_default().push(id);
            }
        }
        all_ids.insert(id);
    }

    // Part 1
    let multiple_claims = claims.values().filter(|ids| ids.len() > 1).count();
    println!("Inches claimed multiple times: '{}'", multiple_claims);

    // Part 2
    // Find the one claim which doesn't overlap with any others
    for ids in claims.values().filter(|ids| ids.len() > 1) {
        for id in ids {
            all_ids.remove(id);
        }
    }

    if all_ids.len() != 1 {
        return Err(From::from("Counld find one id which didn't overlap"));
    }

    all_ids
        .iter()
        .for_each(|id| println!("Claim ids which doesn't overlap: '{}'", id));

    Ok(())
}

fn process_input_line(line: &str) -> Result<(i32, (i32, i32), (i32, i32))> {
    let vec: Vec<i32> = line
        .trim_left_matches("#")
        .split(|c| (c == '@') || (c == ':') || (c == ',') || (c == 'x'))
        .filter_map(|x| x.trim().parse().ok())
        .collect();
    if vec.len() != 5 {
        return Err(From::from("incomplete input line"));
    }

    Ok((vec[0], (vec[1], vec[2]), (vec[3], vec[4])))
}
