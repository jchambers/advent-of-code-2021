use std::str::FromStr;
use std::{env, error};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let mut grid = OctopusGrid::from_str(std::fs::read_to_string(path)?.as_str())?;

            let mut flashes = 0;

            for _ in 0..100 {
                flashes += grid.advance();
            }

            println!("Flashes after 100 steps: {}", flashes);
        }

        {
            let mut grid = OctopusGrid::from_str(std::fs::read_to_string(path)?.as_str())?;

            let mut step = 0;

            loop {
                step += 1;

                if grid.advance() == 100 {
                    break;
                }
            }

            println!("Synchronization on step: {}", step);
        }

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
    pub fn advance(&mut self) -> u32 {
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

        let min_row = if row > 0 {
            row - 1
        } else {
            row
        };

        let max_row = if row < 9 {
            row + 1
        } else {
            row
        };

        let min_col = if col > 0 {
            col - 1
        } else {
            col
        };

        let max_col = if col < 9 {
            col + 1
        } else {
            col
        };

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
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_octopus_grid_from_string() {
        let grid_string = indoc!{"
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

        assert_eq!(expected, OctopusGrid::from_str(grid_string).unwrap());
    }

    #[test]
    fn test_advance() {
        let mut grid = OctopusGrid::from_str(indoc! {"
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
        "}).unwrap();

        let after_first_step = OctopusGrid::from_str(indoc! {"
            6594254334
            3856965822
            6375667284
            7252447257
            7468496589
            5278635756
            3287952832
            7993992245
            5957959665
            6394862637
        "}).unwrap();

        let after_second_step = OctopusGrid::from_str(indoc! {"
            8807476555
            5089087054
            8597889608
            8485769600
            8700908800
            6600088989
            6800005943
            0000007456
            9000000876
            8700006848
        "}).unwrap();

        assert_eq!(0, grid.advance());
        assert_eq!(after_first_step, grid);

        assert_eq!(35, grid.advance());
        assert_eq!(after_second_step, grid);
    }

    #[test]
    fn test_advance_repeated() {
        let mut grid = OctopusGrid::from_str(indoc! {"
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
        "}).unwrap();

        let mut flashes = 0;

        for _ in 0..100 {
            flashes += grid.advance();
        }

        assert_eq!(1656, flashes);
    }
}
