use std::collections::VecDeque;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let numbers = input
        .split(" ")
        .filter_map(|word| word.parse().ok())
        .collect::<Vec<u32>>();
    let players = numbers[0];
    let num_marbles = numbers[1];

    part1(players, num_marbles);
    part2(players, num_marbles);

    Ok(())
}

fn part1(num_players: u32, num_marbles: u32) {
    let mut game = Game::new(num_players, num_marbles);
    game.play();

    println!("Top score: {}", game.top_score());
}

fn part2(num_players: u32, num_marbles: u32) {
    let mut game = Game::new(num_players, num_marbles * 100);
    game.play();

    println!("Top score: {}", game.top_score());
}

struct Game {
    circle: VecDeque<u32>,
    scores: Vec<u64>,
    current_player: usize,

    num_marbles: u32,
}

impl Game {
    fn new(num_players: u32, num_marbles: u32) -> Self {
        let mut circle = VecDeque::new();
        circle.push_back(0);
        Self {
            circle,
            scores: vec![0; num_players as usize],
            current_player: 0,
            num_marbles,
        }
    }

    fn play(&mut self) {
        for marble in 1..=self.num_marbles {
            if (marble % 23) == 0 {
                // Scoring turn
                for _ in 0..7 {
                    let m = self.circle.pop_back().unwrap();
                    self.circle.push_front(m);
                }
                self.scores[self.current_player] +=
                    self.circle.pop_front().unwrap() as u64 + marble as u64;
            } else {
                for _ in 0..2 {
                    let m = self.circle.pop_front().unwrap();
                    self.circle.push_back(m);
                }
                self.circle.push_front(marble);
            }

            self.current_player = Self::next_player(self.current_player, self.scores.len());
        }
    }

    fn next_player(current_player: usize, num_players: usize) -> usize {
        let next = current_player + 1;
        if next >= num_players {
            next - num_players
        } else {
            next
        }
    }

    fn top_score(&self) -> u64 {
        *self.scores.iter().fold(&0, ::std::cmp::max)
    }
}
