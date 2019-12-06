use anyhow::Result;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/day06.txt");

fn get_count<'a>(
    orbits: &HashMap<&'a str, &'a str>,
    counts: &mut HashMap<&'a str, usize>,
    object: &'a str,
) -> usize {
    if let Some(cnt) = counts.get(object) {
        return *cnt;
    }

    if object == "COM" {
        counts.insert(object, 0);
        return 0;
    }

    let o = orbits.get(object).unwrap();
    let cnt = get_count(orbits, counts, o) + 1;
    counts.insert(object, cnt);
    cnt
}

fn get_path<'a>(orbits: &HashMap<&'a str, &'a str>, mut object: &'a str) -> Vec<&'a str> {
    let mut path = Vec::new();

    while let Some(o) = orbits.get(object) {
        path.push(*o);
        object = *o;
    }
    path.reverse();
    path
}

fn find_common<'a>(orbits: &HashMap<&'a str, &'a str>, o1: &'a str, o2: &'a str) -> &'a str {
    let p1 = get_path(orbits, o1);
    let p2 = get_path(orbits, o2);
    p1.iter()
        .zip(p2.iter())
        .fold("", |acc, (o1, o2)| if o1 == o2 { o1 } else { acc })
}

fn main() -> Result<()> {
    let orbits: HashMap<&str, &str> = INPUT
        .lines()
        .map(|line| {
            let index = line.find(')').unwrap();
            (&line[index + 1..], &line[..index])
        })
        .collect::<HashMap<_, _>>();

    let mut counts: HashMap<&str, usize> = HashMap::new();

    let checksum: usize = orbits
        .iter()
        .map(|(object, _)| get_count(&orbits, &mut counts, object))
        .sum();
    println!("Part 1: {}", checksum);

    let common = find_common(&orbits, "YOU", "SAN");
    let base = counts.get(common).unwrap();
    let distance =
        (counts.get("YOU").unwrap() - 1 - base) + (counts.get("SAN").unwrap() - 1 - base);
    println!("Part 2: {}", distance);

    Ok(())
}
