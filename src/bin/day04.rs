use std::{env, error, io};
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use self::Cell::{Marked, Unmarked};

const BOARD_SIZE: usize = 5;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let mut lines = io::BufReader::new(file).lines();

        let selections: Vec<u8> = lines.next().unwrap()?
            .split(",")
            .filter_map(|n| n.parse().ok())
            .collect();

        let boards: Vec<BingoBoard> = {
            let board_lines: Vec<String> = lines
                .filter_map(|line| line.ok())
                .filter(|line| !line.is_empty())
                .collect();

            board_lines.chunks_exact(BOARD_SIZE)
                .map(|chunk| BingoBoard::try_from(TryInto::<&[String; BOARD_SIZE]>::try_into(chunk).unwrap()).unwrap())
                .collect()
        };

        if let Some(score) = get_score_from_first_winner(boards.clone(), &selections) {
            println!("Score from first winner: {}", score);
        }

        if let Some(score) = get_score_from_last_winner(boards, &selections) {
            println!("Score from last winner: {}", score);
        }

        Ok(())
    } else {
        Err("Usage: day04 INPUT_FILE_PATH".into())
    }
}

fn get_score_from_first_winner(mut boards: Vec<BingoBoard>, selections: &[u8]) -> Option<u32> {
    for selection in selections {
        for board in boards.iter_mut() {
            board.mark(*selection);

            if board.is_winner() {
                return Some(board.unmarked_cell_sum() * (*selection as u32));
            }
        }
    }

    None
}

fn get_score_from_last_winner(mut boards: Vec<BingoBoard>, selections: &[u8]) -> Option<u32> {
    for selection in selections {
        for board in boards.iter_mut() {
            board.mark(*selection);
        }

        if boards.len() == 1 && boards[0].is_winner() {
            return Some(boards[0].unmarked_cell_sum() * (*selection as u32));
        }

        boards.retain(|board| !board.is_winner());
    }

    None
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Unmarked(u8),
    Marked(u8)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BingoBoard {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE]
}

impl BingoBoard {
    pub fn mark(&mut self, number: u8) {
        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                if self.cells[row][col] == Unmarked(number) {
                    self.cells[row][col] = Marked(number);
                    return;
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        // Check for matches across rows
        for row in self.cells {
            if row.iter().all(|cell| matches!(cell, Marked(_))) {
                return true;
            }
        }

        // Check for matches down columns
        for col in 0..BOARD_SIZE {
            if self.cells.iter()
                .map(|row| row[col])
                .all(|cell| matches!(cell, Marked(_))) {

                return true;
            }
        }

        false
    }

    pub fn unmarked_cell_sum(&self) -> u32 {
        self.cells.iter()
            .flat_map(|row| row)
            .map(|cell| match cell {
                Unmarked(number) => *number as u32,
                _ => 0
            })
            .sum()
    }
}

impl TryFrom<&[String; BOARD_SIZE]> for BingoBoard {
    type Error = &'static str;

    fn try_from(rows: &[String; BOARD_SIZE]) -> Result<Self, Self::Error> {
        let mut cells = [[Unmarked(0); BOARD_SIZE]; BOARD_SIZE];

        for row in 0..BOARD_SIZE {
            let numbers: Vec<u8> = rows[row].split_whitespace()
                .filter_map(|number| u8::from_str(number).ok())
                .collect();

            if numbers.len() != BOARD_SIZE {
                return Err("Bad row length".into());
            }

            for col in 0..numbers.len() {
                cells[row][col] = Unmarked(numbers[col]);
            }
        }

        Ok(BingoBoard { cells })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_from_strings() {
        let board_strings = [
            String::from("22 13 17 11  0"),
            String::from("8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from("6 10  3 18  5"),
            String::from("1 12 20 15 19"),
        ];

        assert_eq!(BingoBoard {
            cells: [
                [Unmarked(22), Unmarked(13), Unmarked(17), Unmarked(11), Unmarked(0)],
                [Unmarked(8), Unmarked(2), Unmarked(23), Unmarked(4), Unmarked(24)],
                [Unmarked(21), Unmarked(9), Unmarked(14), Unmarked(16), Unmarked(7)],
                [Unmarked(6), Unmarked(10), Unmarked(3), Unmarked(18), Unmarked(5)],
                [Unmarked(1), Unmarked(12), Unmarked(20), Unmarked(15), Unmarked(19)]]
        }, BingoBoard::try_from(&board_strings).unwrap());
    }

    #[test]
    fn test_mark_board() {
        let mut board = BingoBoard {
            cells: [
                [Unmarked(22), Unmarked(13), Unmarked(17), Unmarked(11), Unmarked(0)],
                [Unmarked(8), Unmarked(2), Unmarked(23), Unmarked(4), Unmarked(24)],
                [Unmarked(21), Unmarked(9), Unmarked(14), Unmarked(16), Unmarked(7)],
                [Unmarked(6), Unmarked(10), Unmarked(3), Unmarked(18), Unmarked(5)],
                [Unmarked(1), Unmarked(12), Unmarked(20), Unmarked(15), Unmarked(19)]]
        };

        board.mark(14);

        assert_eq!(BingoBoard {
            cells: [
                [Unmarked(22), Unmarked(13), Unmarked(17), Unmarked(11), Unmarked(0)],
                [Unmarked(8), Unmarked(2), Unmarked(23), Unmarked(4), Unmarked(24)],
                [Unmarked(21), Unmarked(9), Marked(14), Unmarked(16), Unmarked(7)],
                [Unmarked(6), Unmarked(10), Unmarked(3), Unmarked(18), Unmarked(5)],
                [Unmarked(1), Unmarked(12), Unmarked(20), Unmarked(15), Unmarked(19)]]
        }, board);
    }

    #[test]
    fn test_is_winner() {
        {
            let mut board = BingoBoard {
                cells: [
                    [Unmarked(14), Unmarked(21), Unmarked(17), Unmarked(24), Unmarked(4)],
                    [Unmarked(10), Unmarked(16), Unmarked(15), Unmarked(9), Unmarked(19)],
                    [Unmarked(18), Unmarked(8), Unmarked(23), Unmarked(26), Unmarked(20)],
                    [Unmarked(22), Unmarked(11), Unmarked(13), Unmarked(6), Unmarked(5)],
                    [Unmarked(2), Unmarked(0), Unmarked(12), Unmarked(3), Unmarked(7)]]
            };

            assert!(!board.is_winner());

            for number in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21] {
                board.mark(number);
            }

            assert!(!board.is_winner());

            board.mark(24);

            assert!(board.is_winner());
        }

        {
            let mut board = BingoBoard {
                cells: [
                    [Unmarked(14), Unmarked(21), Unmarked(17), Unmarked(24), Unmarked(4)],
                    [Unmarked(10), Unmarked(16), Unmarked(15), Unmarked(9), Unmarked(19)],
                    [Unmarked(18), Unmarked(8), Unmarked(23), Unmarked(26), Unmarked(20)],
                    [Unmarked(22), Unmarked(11), Unmarked(13), Unmarked(6), Unmarked(5)],
                    [Unmarked(2), Unmarked(0), Unmarked(12), Unmarked(3), Unmarked(7)]]
            };

            assert!(!board.is_winner());

            for number in [24, 9, 26, 6, 3] {
                board.mark(number);
            }

            assert!(board.is_winner());
        }
    }

    #[test]
    fn test_unmarked_cell_sum() {
        let mut board = BingoBoard {
            cells: [
                [Unmarked(14), Unmarked(21), Unmarked(17), Unmarked(24), Unmarked(4)],
                [Unmarked(10), Unmarked(16), Unmarked(15), Unmarked(9), Unmarked(19)],
                [Unmarked(18), Unmarked(8), Unmarked(23), Unmarked(26), Unmarked(20)],
                [Unmarked(22), Unmarked(11), Unmarked(13), Unmarked(6), Unmarked(5)],
                [Unmarked(2), Unmarked(0), Unmarked(12), Unmarked(3), Unmarked(7)]]
        };

        for number in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24] {
            board.mark(number);
        }

        assert_eq!(188, board.unmarked_cell_sum());
    }
}