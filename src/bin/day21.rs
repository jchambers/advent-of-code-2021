use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let game = DiceGame::new(
            u32::from_str(args[1].as_str())?,
            u32::from_str(args[2].as_str())?,
            DeterministicDie::new(),
        );

        println!("Outcome with deterministic die: {}", game.play());

        Ok(())
    } else {
        Err("Usage: day15 P1_START P2START".into())
    }
}

trait Die {
    fn roll(&mut self) -> u32;
    fn roll_count(&self) -> u32;
}

struct DiceGame<'a> {
    die: Box<dyn Die + 'a>,
    player_positions: [u32; 2],
    player_scores: [u32; 2],
}

impl<'a> DiceGame<'a> {
    pub fn new(p1_position: u32, p2_position: u32, die: impl Die + 'a) -> Self {
        DiceGame {
            die: Box::new(die),
            player_positions: [p1_position - 1, p2_position - 1],
            player_scores: [0; 2],
        }
    }

    pub fn play(mut self) -> u32 {
        let mut turn = 0;

        while *self.player_scores.iter().max().unwrap() < 1_000 {
            let current_player = turn % 2;

            for _ in 0..3 {
                self.player_positions[current_player] += self.die.roll();
            }

            self.player_positions[current_player] %= 10;
            self.player_scores[current_player] += self.player_positions[current_player] + 1;

            turn += 1;
        }

        *self.player_scores.iter().min().unwrap() * self.die.roll_count()
    }
}

struct DeterministicDie {
    roll_count: u32,
}

impl DeterministicDie {
    pub fn new() -> Self {
        DeterministicDie { roll_count: 0 }
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u32 {
        let roll = (self.roll_count % 100) + 1;
        self.roll_count += 1;

        roll
    }

    fn roll_count(&self) -> u32 {
        self.roll_count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deterministic_die() {
        let mut die = DeterministicDie::new();

        for expected in 1..=100 {
            assert_eq!(expected, die.roll());
        }

        assert_eq!(1, die.roll());
        assert_eq!(101, die.roll_count);
    }

    #[test]
    fn test_game_with_deterministic_die() {
        let game = DiceGame::new(4, 8, DeterministicDie::new());
        assert_eq!(739785, game.play());
    }
}
