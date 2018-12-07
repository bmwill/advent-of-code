use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let edges = parse_input(&input);

    part1(&edges)?;
    part2(&edges)?;

    Ok(())
}

fn part1(edges: &[(char, char)]) -> Result<()> {
    let mut dependencies = create_dependencies(edges);

    let mut ready = get_ready(&dependencies);
    ready.sort();
    ready.reverse();

    let mut order = String::new();
    while let Some(step) = ready.pop() {
        order.push(step);

        unblock_and_make_ready(step, &mut dependencies, &mut ready);

        ready.sort();
        ready.reverse();
    }

    println!("Order: {}", order);

    Ok(())
}

fn unblock_and_make_ready(
    blocker: char,
    dependencies: &mut HashMap<char, (HashSet<char>, HashSet<char>)>,
    ready: &mut Vec<char>,
) {
    let blocking = dependencies.get(&blocker).unwrap().1.clone();
    for s in blocking {
        let blocked_by = &mut dependencies.get_mut(&s).unwrap().0;
        blocked_by.remove(&blocker);
        if blocked_by.is_empty() {
            ready.push(s);
        }
    }
}
// A hashmap which maps a Step to a list of steps it is blocked on (0)
// and a list of steps it blocks (1)
fn create_dependencies(edges: &[(char, char)]) -> HashMap<char, (HashSet<char>, HashSet<char>)> {
    let mut dependencies: HashMap<char, (HashSet<char>, HashSet<char>)> = HashMap::new();

    for (x, y) in edges {
        dependencies.entry(*x).or_default().1.insert(*y);
        dependencies.entry(*y).or_default().0.insert(*x);
    }

    dependencies
}
fn get_ready(dependencies: &HashMap<char, (HashSet<char>, HashSet<char>)>) -> Vec<char> {
    dependencies
        .iter()
        .filter(|(_step, (blocked_by, _blocking))| blocked_by.is_empty())
        .map(|(step, (_blocked_by, _blocking))| *step)
        .collect()
}

fn part2(edges: &[(char, char)]) -> Result<()> {
    let mut dependencies = create_dependencies(edges);
    // Vector of the 5 workers (time, step)
    let mut workers = vec![(0, None); 5];

    let mut ready = get_ready(&dependencies);
    ready.sort();
    ready.reverse();

    let mut time = 0;
    loop {
        // Unblock everyone that is finished
        for worker in workers.iter_mut() {
            if worker.0 <= time && worker.1.is_some() {
                let step = worker.1.take().unwrap();
                unblock_and_make_ready(step, &mut dependencies, &mut ready);
            }
        }

        // Sort the list of readys
        ready.sort();
        ready.reverse();

        // Assign ready tasks to available workers
        for worker in workers.iter_mut() {
            if worker.1.is_none() {
                if let Some(step) = ready.pop() {
                    let step_time = (step as u8) - b'A' + 1 + 60;
                    worker.0 = time + step_time as u32;
                    worker.1 = Some(step);
                }
            }
        }

        if ready.is_empty() && all_workers_done(&workers, time) {
            break;
        }

        time += 1;
    }

    println!("time to finish: {}", time);

    Ok(())
}

fn all_workers_done(workers: &[(u32, Option<char>)], time: u32) -> bool {
    workers
        .iter()
        .fold(true, |sum, worker| (worker.0 <= time) && sum)
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            let x = line.trim_start_matches("Step ").chars().next().unwrap();
            let y = line
                .trim_end_matches(" can begin.")
                .chars()
                .rev()
                .next()
                .unwrap();
            (x, y)
        })
        .collect()
}
