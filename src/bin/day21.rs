use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let p1_position = u32::from_str(args[1].as_str())?;
        let p2_position = u32::from_str(args[2].as_str())?;

        println!(
            "Outcome with deterministic die: {}",
            play_deterministic_game(p1_position, p2_position)
        );

        Ok(())
    } else {
        Err("Usage: day15 P1_START P2START".into())
    }
}

struct GameState {
    active_player: usize,
    positions: [u32; 2],
    scores: [u32; 2],
}

impl GameState {
    pub fn new(p1_position: u32, p2_position: u32) -> Self {
        GameState {
            active_player: 0,
            positions: [p1_position - 1, p2_position - 1],
            scores: [0, 0],
        }
    }

    pub fn advance(&self, roll_total: u32) -> Self {
        let mut updated_positions = self.positions.clone();
        updated_positions[self.active_player] += roll_total;
        updated_positions[self.active_player] %= 10;

        let mut updated_scores = self.scores.clone();
        updated_scores[self.active_player] += updated_positions[self.active_player] + 1;

        GameState {
            active_player: self.active_player ^ 1,
            positions: updated_positions,
            scores: updated_scores,
        }
    }
}

fn play_deterministic_game(p1_position: u32, p2_position: u32) -> u32 {
    let mut game_state = GameState::new(p1_position, p2_position);
    let mut die = DeterministicDie::new();

    while *game_state.scores.iter().max().unwrap() < 1_000 {
        let mut roll_total = 0;

        for _ in 0..3 {
            roll_total += die.roll();
        }

        game_state = game_state.advance(roll_total);
    }

    *game_state.scores.iter().min().unwrap() * die.roll_count()
}

struct DeterministicDie {
    roll_count: u32,
}

impl DeterministicDie {
    pub fn new() -> Self {
        DeterministicDie { roll_count: 0 }
    }

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
    fn test_play_deterministic_game() {
        assert_eq!(739785, play_deterministic_game(4, 8));
    }
}
