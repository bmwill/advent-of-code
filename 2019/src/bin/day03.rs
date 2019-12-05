use anyhow::{bail, Result};
use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../input/day03.txt");

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn draw(&self, direction: Point, magnitude: isize) -> Point {
        Self::new(
            self.x + direction.x * magnitude,
            self.y + direction.y * magnitude,
        )
    }

    fn _distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// Return a direciton and magnitude
fn parse_direction(d: &str) -> Result<(Point, isize)> {
    let direction = match &d[..1] {
        "R" => Point::new(1, 0),
        "L" => Point::new(-1, 0),
        "U" => Point::new(0, 1),
        "D" => Point::new(0, -1),
        c => bail!("unknown direction: {}", c),
    };

    let magnitude = d[1..].parse()?;
    Ok((direction, magnitude))
}

fn run(input: &str) -> Result<()> {
    let wires: Vec<Vec<Point>> = input
        .lines()
        .map(|line| {
            line.trim().split(',').map(parse_direction).fold_results(
                vec![Point::new(0, 0)],
                |mut acc, (d, m)| {
                    for _ in 1..=m {
                        let next = acc.last().unwrap().draw(d, 1);
                        acc.push(next);
                    }
                    acc
                },
            )
        })
        .collect::<Result<_>>()?;

    let set1: HashSet<Point> = wires[0].iter().skip(1).cloned().collect();
    let set2: HashSet<Point> = wires[1].iter().skip(1).cloned().collect();

    let closest = set1
        .intersection(&set2)
        .map(|p| p.x.abs() + p.y.abs())
        .min()
        .unwrap();

    println!("part 1: {}", closest);

    let min_steps = set1
        .intersection(&set2)
        .map(|p| {
            let steps0 = wires[0]
                .iter()
                .enumerate()
                .find(|(_, q)| *q == p)
                .unwrap()
                .0;
            let steps1 = wires[1]
                .iter()
                .enumerate()
                .find(|(_, q)| *q == p)
                .unwrap()
                .0;
            steps0 + steps1
        })
        .min()
        .unwrap();
    println!("part 2: {}", min_steps);
    Ok(())
}

fn main() -> Result<()> {
    run(INPUT)?;
    Ok(())
}

#[test]
fn example() {
    let test = "R8,U5,L5,D3\nU7,R6,D4,L4";
    run(test).unwrap();
    let test = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    run(test).unwrap();
    let test = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    run(test).unwrap();
}
