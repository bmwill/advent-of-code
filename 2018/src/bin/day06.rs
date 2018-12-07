use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Read};
use std::str::FromStr;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let points = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Point>>>()?;

    part1(&points)?;
    part2(&points)?;

    Ok(())
}

fn part2(points: &[Point]) -> Result<()> {
    let max_x = points.iter().map(|p| p.x).max().ok_or("input is empty")?;
    let max_y = points.iter().map(|p| p.y).max().ok_or("input is empty")?;

    let area = (0..=max_x)
        .flat_map(|x| (0..=max_y).map(move |y| Point { x, y }))
        .map(|p| points.iter().map(|point| point.distance(p)).sum::<i32>())
        .filter(|sum| *sum < 10000)
        .count();

    println!("area: {}", area);
    Ok(())
}

fn part1(points: &[Point]) -> Result<()> {
    let max_x = points.iter().map(|p| p.x).max().ok_or("input is empty")?;
    let max_y = points.iter().map(|p| p.y).max().ok_or("input is empty")?;

    // Calculate distances on our grid
    let distances = calculate_distances(points, max_x, max_y);
    let mut eligable_locations = points.iter().map(|p| *p).collect::<HashSet<Point>>();
    remove_boarder_locations(&distances, &mut eligable_locations, max_x, max_y);

    let max_area = eligable_locations
        .iter()
        .map(|p| distances.values().filter(|x| p == *x).count())
        .max()
        .ok_or("input is empty")?;

    println!("max area: {}", max_area);
    Ok(())
}

fn remove_boarder_locations(
    distances: &HashMap<Point, Point>,
    locations: &mut HashSet<Point>,
    max_x: i32,
    max_y: i32,
) {
    // Top
    let top = (0..=max_x).into_iter().map(|x| Point { x, y: 0 });
    let bottom = (0..=max_x).into_iter().map(|x| Point { x, y: max_y });
    let left = (0..=max_y).into_iter().map(|y| Point { x: 0, y });
    let right = (0..=max_y).into_iter().map(|y| Point { x: max_x, y });

    top.chain(bottom).chain(left).chain(right).for_each(|p| {
        if let Some(point) = distances.get(&p) {
            locations.remove(point);
        }
    });
}

fn calculate_distances(points: &[Point], max_x: i32, max_y: i32) -> HashMap<Point, Point> {
    // Find the max x and y coordinates to find the bounding box
    let mut grid = HashMap::new();

    for x in 0..=max_x {
        for y in 0..=max_y {
            let point = Point { x, y };
            if let Some(closest) = closest_point(point, points) {
                grid.insert(point, closest);
            }
        }
    }

    grid
}

fn closest_point(point: Point, points: &[Point]) -> Option<Point> {
    let mut min = &points[0];
    let mut unique = true;
    for p in &points[1..] {
        if min.distance(point) == p.distance(point) {
            unique = false;
        } else if min.distance(point) > p.distance(point) {
            unique = true;
            min = p;
        }
    }

    if unique {
        Some(*min)
    } else {
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl FromStr for Point {
    type Err = Box<::std::error::Error>;

    fn from_str(s: &str) -> Result<Point> {
        let x = s
            .trim_end_matches(|c| c != ',')
            .trim_end_matches(',')
            .parse()?;
        let y = s
            .trim_start_matches(|c| c != ',')
            .trim_start_matches(',')
            .trim()
            .parse()?;
        Ok(Point { x, y })
    }
}
