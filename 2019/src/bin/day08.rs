use anyhow::Result;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/day08.txt");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part1(layers: &Vec<Vec<u8>>) {
    let (_zeros, ones, twos) = layers
        .iter()
        .map(|layer| {
            let mut zeros = 0;
            let mut ones = 0;
            let mut twos = 0;
            for &val in layer {
                if val == 0 {
                    zeros += 1;
                } else if val == 1 {
                    ones += 1;
                } else if val == 2 {
                    twos += 1;
                }
            }
            (zeros, ones, twos)
        })
        .min_by(|i1, i2| i1.0.cmp(&i2.0))
        .unwrap();

    println!("Part 1: {}", ones * twos);
}

fn part2(layers: &Vec<Vec<u8>>) {
    let mut image = [2; WIDTH * HEIGHT];

    for layer in layers {
        for (idx, color) in layer.iter().enumerate() {
            if image[idx] == 2 {
                image[idx] = *color;
            }
        }
    }

    println!("Part 2:");

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            if image[WIDTH * j + i] == 0 {
                print!("█");
            } else if image[WIDTH * j + i] == 1 {
                print!("░");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() -> Result<()> {
    let layers: Vec<Vec<u8>> = INPUT
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|i| i as u8))
        .chunks(WIDTH * HEIGHT)
        .into_iter()
        .map(|i| i.into_iter().collect::<Vec<u8>>())
        .collect();

    part1(&layers);
    part2(&layers);

    Ok(())
}
