use std::cmp::{max, min};
use std::collections::VecDeque;
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

        println!(
            "Outcome with quantum die: {}",
            play_quantum_game(p1_position, p2_position)
        );

        Ok(())
    } else {
        Err("Usage: day21 P1_START P2_START".into())
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
        let mut updated_positions = self.positions;
        updated_positions[self.active_player] += roll_total;
        updated_positions[self.active_player] %= 10;

        let mut updated_scores = self.scores;
        updated_scores[self.active_player] += updated_positions[self.active_player] + 1;

        GameState {
            active_player: self.active_player ^ 1,
            positions: updated_positions,
            scores: updated_scores,
        }
    }

    pub fn min_score(&self) -> u32 {
        min(self.scores[0], self.scores[1])
    }

    pub fn max_score(&self) -> u32 {
        max(self.scores[0], self.scores[1])
    }
}

fn play_deterministic_game(p1_position: u32, p2_position: u32) -> u32 {
    let mut game_state = GameState::new(p1_position, p2_position);
    let mut die = 0;

    while game_state.max_score() < 1_000 {
        let mut roll_total = 0;

        for _ in 0..3 {
            roll_total += (die + 1) % 100;
            die += 1;
        }

        game_state = game_state.advance(roll_total);
    }

    game_state.min_score() * die
}

fn play_quantum_game(p1_position: u32, p2_position: u32) -> u64 {
    const ROLLS_AND_FREQUENCIES: [(u32, u64); 7] =
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut wins = [0, 0];
    let mut state_queue = VecDeque::new();

    state_queue.push_back((GameState::new(p1_position, p2_position), 1));

    while let Some((state, paths_to_state)) = state_queue.pop_front() {
        for (roll, frequency) in ROLLS_AND_FREQUENCIES {
            let next_state = state.advance(roll);

            if next_state.max_score() >= 21 {
                wins[state.active_player] += paths_to_state * frequency
            } else {
                state_queue.push_back((next_state, paths_to_state * frequency))
            }
        }
    }

    max(wins[0], wins[1])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_play_deterministic_game() {
        assert_eq!(739785, play_deterministic_game(4, 8));
    }

    #[test]
    fn test_play_quantum_game() {
        assert_eq!(444356092776315, play_quantum_game(4, 8));
    }
}
