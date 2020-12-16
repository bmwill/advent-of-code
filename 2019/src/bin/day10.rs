use anyhow::Result;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input/day10.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn angle(&self, other: &Self) -> OrderedFloat<f64> {
        let x = other.x as f64 - self.x as f64;
        let y = other.y as f64 - self.y as f64;

        OrderedFloat(x.atan2(y))
    }
}

fn asteroids(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn part1(asteroids: &[Point]) {
    let mut counts = HashMap::new();
    let mut angles = HashSet::new();

    for a in asteroids {
        angles.clear();
        for b in asteroids {
            // Skip if its the same one
            if a == b {
                continue;
            }

            //let x = b.x as i32 - a.x as i32;
            //let y = b.y as i32 - a.y as i32;

            //if !angles.iter().any(|&(x1, y1)| {
            //    let x_mod = if x1 == 0 || x == 0 {
            //        0
            //    } else if x1 > x {
            //        x1 % x
            //    } else {
            //        x % x1
            //    };
            //    let y_mod = if y1 == 0 || y == 0 {
            //        0
            //    } else if y1 > y {
            //        y1 % y
            //    } else {
            //        y % y1
            //    };

            //    x_mod == 0 && y_mod == 0
            //}) {
            //    angles.insert((x, y));
            //}

            angles.insert(a.angle(b));
        }
        counts.insert(a, angles.len());
    }

    let max = counts.iter().map(|(_p, count)| count).max().unwrap();

    println!("Part 1: {}", max);
}

fn main() -> Result<()> {
    let asteroids: Vec<Point> = INPUT
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point::new(x, y))
                } else {
                    None
                }
            })
        })
        .collect();

    part1(&asteroids);

    Ok(())
}

#[test]
fn t() {
    let input = r#"......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####"#;

    let a = asteroids(input);

    part1(&a);

    let input = r#".#..#
                .....
                #####
                ....#
                ...##"#;

    let a = asteroids(input);

    part1(&a);
}
