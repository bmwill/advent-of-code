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
    circle: Vec<u32>,
    current_marble: usize,
    scores: Vec<u32>,
    current_player: usize,

    num_marbles: u32,
}

impl Game {
    fn new(num_players: u32, num_marbles: u32) -> Self {
        Self {
            circle: vec![0],
            current_marble: 0,
            scores: vec![0; num_players as usize],
            current_player: 0,
            num_marbles,
        }
    }

    fn play(&mut self) {
        for marble in 1..=self.num_marbles {
            if (marble % 100000) == 0 {
                println!("iteration {}", marble);
            }

            if (marble % 23) == 0 {
                // Scoring turn
                // TODO take score
                let next = Self::next_take_location(self.current_marble, self.circle.len());
                self.scores[self.current_player] += self.circle.remove(next) + marble;

                self.current_marble = if next == self.circle.len() { 0 } else { next }
            } else {
                // Normal turn
                let next = Self::next_place_location(self.current_marble, self.circle.len());
                self.circle.insert(next, marble);
                self.current_marble = next;
            }

            //println!("{}", self);
            self.current_player = Self::next_player(self.current_player, self.scores.len());
        }
    }

    fn next_place_location(current_marble: usize, circle_len: usize) -> usize {
        let next = current_marble + 2;
        if next > circle_len {
            next - circle_len
        } else {
            next
        }
    }

    fn next_take_location(current_marble: usize, circle_len: usize) -> usize {
        let next = current_marble as i32 - 7;
        if next < 0 {
            (next + circle_len as i32) as usize
        } else {
            next as usize
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

    fn top_score(&self) -> u32 {
        *self.scores.iter().max().unwrap()
    }
}

impl ::std::fmt::Display for Game {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut output = String::new();

        for (index, marble) in self.circle.iter().enumerate() {
            if index == self.current_marble {
                output.push_str(&format!(" ({})", marble));
            } else {
                output.push_str(&format!(" {}", marble));
            }
        }
        write!(f, "{}", output)
    }
}
