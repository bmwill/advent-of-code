use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut records: Vec<&str> = input.lines().collect();
    records.sort_unstable();
    let sleep_minutes = get_sleep_minutes(&records)?;

    part1(&sleep_minutes)?;
    part2(&sleep_minutes)?;

    Ok(())
}

fn get_sleep_minutes(records: &[&str]) -> Result<HashMap<u32, Vec<u32>>> {
    let mut sleep_minutes: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut current_guard: u32 = 0;
    let mut fall_asleep_minute = 0;
    for record in records {
        if record.contains("#") {
            // Guard ID
            current_guard = record
                .trim_start_matches(|c| c != '#')
                .trim_start_matches('#')
                .trim_end_matches(" begins shift")
                .parse()?;
        } else if record.contains("falls asleep") {
            // The moment a guard falls asleep
            fall_asleep_minute = get_minutes(record)?;
        } else if record.contains("wakes up") {
            // When a guard wakes up we can record his slept time
            let wakes_up = get_minutes(record)?;
            let schedule = sleep_minutes.entry(current_guard).or_insert(vec![0; 60]);
            for m in fall_asleep_minute..wakes_up {
                schedule[m] += 1;
            }
        } else {
            return Err(From::from("invalid input line"));
        }
    }

    Ok(sleep_minutes)
}

fn get_minutes(record: &str) -> Result<usize> {
    Ok(record
        .trim_start_matches(|c| c != ':')
        .trim_start_matches(':')
        .trim_end_matches(|c| c != ']')
        .trim_end_matches(']')
        .parse()?)
}

fn part1(sleep_minutes: &HashMap<u32, Vec<u32>>) -> Result<()> {
    let (sleepiest_guard, _) = sleep_minutes
        .iter()
        .map(|(k, v)| (k, v.iter().sum()))
        //.for_each(|x| println!("{:?}", x));
        .max_by(|(_k1, v1): &(&u32, u32), (_k2, v2): &(&u32, u32)| v1.cmp(&v2))
        .unwrap();

    let (sleepiest_minute, _) = sleep_minutes
        .get(sleepiest_guard)
        .unwrap()
        .iter()
        .enumerate()
        .max_by(|(_i1, m1), (_i2, m2)| m1.cmp(m2))
        .unwrap();

    println!(
        "sleepiest guard: '{}' sleepiest minute: '{}' Answer = {}",
        sleepiest_guard,
        sleepiest_minute,
        sleepiest_guard * sleepiest_minute as u32
    );

    Ok(())
}

fn part2(sleep_minutes: &HashMap<u32, Vec<u32>>) -> Result<()> {
    let (sleepiest_guard, (sleepiest_minute, _)) = sleep_minutes
        .iter()
        .map(|(k, v)| {
            let (sleepiest_minute, time_asleep) = v
                .iter()
                .enumerate()
                .max_by(|(_i1, m1), (_i2, m2)| m1.cmp(m2))
                .unwrap();
            (k, (sleepiest_minute, time_asleep))
        })
        .max_by(|(_k1, (_m1, t1)), (_k2, (_m2, t2))| t1.cmp(&t2))
        .unwrap();

    println!(
        "sleepiest guard: '{}' sleepiest minute: '{}' Answer = {}",
        sleepiest_guard,
        sleepiest_minute,
        sleepiest_guard * sleepiest_minute as u32
    );

    Ok(())
}
