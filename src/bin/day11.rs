use std::collections::HashSet;
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        println!(
            "Flashes after 100 steps: {}",
            OctopusGrid::from_str(std::fs::read_to_string(path)?.as_str())?
                .flashes_after_steps(100)
        );

        println!(
            "Steps until synchronization: {}",
            OctopusGrid::from_str(std::fs::read_to_string(path)?.as_str())?
                .steps_until_synchronization()
        );

        Ok(())
    } else {
        Err("Usage: day11 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct OctopusGrid {
    energy_levels: [[u8; 10]; 10],
}

impl OctopusGrid {
    pub fn flashes_after_steps(mut self, steps: u32) -> u32 {
        let mut flashes = 0;

        for _ in 0..steps {
            flashes += self.advance();
        }

        flashes
    }

    pub fn steps_until_synchronization(mut self) -> u32 {
        let mut step = 0;

        loop {
            step += 1;

            if self.advance() == 100 {
                break step;
            }
        }
    }

    fn advance(&mut self) -> u32 {
        for row in 0..10 {
            for col in 0..10 {
                self.energy_levels[row][col] += 1;
            }
        }

        let mut flashed = HashSet::new();

        loop {
            let mut converged = true;

            for row in 0..10 {
                for col in 0..10 {
                    if self.energy_levels[row][col] > 9 {
                        if flashed.insert((row, col)) {
                            // This octopus just crossed the energy threshold and hasn't already
                            // flashed
                            converged = false;

                            for (neighbor_row, neighbor_col) in Self::neighbors(row, col) {
                                self.energy_levels[neighbor_row][neighbor_col] += 1;
                            }
                        }
                    }
                }
            }

            if converged {
                break;
            }
        }

        for row in 0..10 {
            for col in 0..10 {
                if self.energy_levels[row][col] > 9 {
                    self.energy_levels[row][col] = 0;
                }
            }
        }

        flashed.len() as u32
    }

    fn neighbors(row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        let min_row = if row > 0 { row - 1 } else { row };
        let max_row = if row < 9 { row + 1 } else { row };
        let min_col = if col > 0 { col - 1 } else { col };
        let max_col = if col < 9 { col + 1 } else { col };

        for neighbor_row in min_row..=max_row {
            for neighbor_col in min_col..=max_col {
                if neighbor_row != row || neighbor_col != col {
                    neighbors.push((neighbor_row, neighbor_col));
                }
            }
        }

        neighbors
    }
}

impl FromStr for OctopusGrid {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let energy_levels: [[u8; 10]; 10] = string
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c as u8 - '0' as u8)
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[u8; 10]>>()
            .try_into()
            .unwrap();

        Ok(OctopusGrid { energy_levels })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_GRID_STRING: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn test_octopus_grid_from_string() {
        let expected = OctopusGrid {
            energy_levels: [
                [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
            ],
        };

        assert_eq!(expected, OctopusGrid::from_str(TEST_GRID_STRING).unwrap());
    }

    #[test]
    fn test_advance() {
        let mut grid = OctopusGrid::from_str(TEST_GRID_STRING).unwrap();

        assert_eq!(0, grid.advance());
        assert_eq!(
            OctopusGrid {
                energy_levels: [
                    [6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                    [3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                    [6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                    [7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                    [7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                    [5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                    [3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                    [7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                    [5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                    [6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
                ]
            },
            grid
        );

        assert_eq!(35, grid.advance());
        assert_eq!(
            OctopusGrid {
                energy_levels: [
                    [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                    [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                    [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                    [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                    [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                    [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                    [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                    [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                    [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                    [8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
                ]
            },
            grid
        );
    }

    #[test]
    fn test_flashes_after_steps() {
        assert_eq!(
            1656,
            OctopusGrid::from_str(TEST_GRID_STRING)
                .unwrap()
                .flashes_after_steps(100)
        );
    }

    #[test]
    fn test_steps_until_synchronization() {
        assert_eq!(
            195,
            OctopusGrid::from_str(TEST_GRID_STRING)
                .unwrap()
                .steps_until_synchronization()
        );
    }
}
