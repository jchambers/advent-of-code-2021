use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let cave_map = CaveMap::from_str(std::fs::read_to_string(path)?.as_str()).unwrap();

        println!(
            "Total risk score along path to exit: {}",
            cave_map.path_risk_to_exit((
                cave_map.risk_scores.len() - 1,
                cave_map.risk_scores.last().unwrap().len() - 1
            ))
        );

        println!(
            "Total risk score along path to exit (with extended grid): {}",
            cave_map.path_risk_to_exit((
                (cave_map.risk_scores.len() * 5) - 1,
                (cave_map.risk_scores.last().unwrap().len() * 5) - 1
            ))
        );

        Ok(())
    } else {
        Err("Usage: day15 INPUT_FILE_PATH".into())
    }
}

type RowCol = (usize, usize);

struct CaveMap {
    risk_scores: Vec<Vec<u8>>,
}

impl CaveMap {
    pub fn path_risk_to_exit(&self, exit: RowCol) -> u32 {
        let mut visited_nodes = HashSet::new();
        let mut tentative_distances = BinaryHeap::new();

        tentative_distances.push(NodeAndDistance {
            distance: 0,
            position: (0, 0)
        });

        while let Some(node_and_distance) = tentative_distances.pop() {
            if visited_nodes.contains(&(node_and_distance.position)) {
                continue;
            }

            if node_and_distance.position == exit {
                return node_and_distance.distance;
            }

            // Update the tentative distance to each unvisited neighbor
            self.neighbors(node_and_distance.position, exit.0, exit.1)
                .iter()
                .filter(|&neighbor| !visited_nodes.contains(neighbor))
                .for_each(|&neighbor| {
                    let tentative_distance =
                        self.risk_score(neighbor) + node_and_distance.distance;

                    tentative_distances.push(NodeAndDistance {
                        distance: tentative_distance,
                        position: neighbor
                    });
                });

            visited_nodes.insert(node_and_distance.position);
        }

        u32::MAX
    }

    fn neighbors(
        &self,
        position: RowCol,
        max_row: usize,
        max_col: usize,
    ) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        let (row, col) = position;

        if row > 0 {
            neighbors.push((row - 1, col));
        }

        if row < max_row {
            neighbors.push((row + 1, col));
        }

        if col > 0 {
            neighbors.push((row, col - 1));
        }

        if col < max_col {
            neighbors.push((row, col + 1));
        }

        neighbors
    }

    fn risk_score(&self, position: RowCol) -> u32 {
        let (row, col) = position;

        let row_within_tile = row % self.risk_scores.len();
        let col_within_tile = col % self.risk_scores[row_within_tile].len();

        let tile_distance =
            (row / self.risk_scores.len()) + (col / self.risk_scores[row_within_tile].len());

        ((self.risk_scores[row_within_tile][col_within_tile] as u32 - 1 + tile_distance as u32) % 9)
            + 1
    }
}

#[derive(Eq, PartialEq)]
struct NodeAndDistance {
    distance: u32,
    position: RowCol,
}

impl Ord for NodeAndDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        // Swap the "normal" order so we have a min-first heap
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for NodeAndDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for CaveMap {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(CaveMap {
            risk_scores: string
                .lines()
                .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_MAP_STRING: &str = indoc! {"
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581
    "};

    #[test]
    fn test_path_risk_to_exit() {
        let cave_map = CaveMap::from_str(TEST_MAP_STRING).unwrap();

        assert_eq!(40, cave_map.path_risk_to_exit((9, 9)));
        assert_eq!(315, cave_map.path_risk_to_exit((49, 49)));
    }

    #[test]
    fn test_risk_score() {
        let cave_map = CaveMap::from_str(TEST_MAP_STRING).unwrap();

        assert_eq!(1, cave_map.risk_score((0, 0)));
        assert_eq!(2, cave_map.risk_score((0, 10)));
        assert_eq!(2, cave_map.risk_score((10, 0)));
        assert_eq!(3, cave_map.risk_score((10, 10)));

        assert_eq!(9, cave_map.risk_score((9, 4)));
        assert_eq!(1, cave_map.risk_score((19, 4)));
        assert_eq!(1, cave_map.risk_score((9, 14)));
        assert_eq!(2, cave_map.risk_score((19, 14)));

        assert_eq!(9, cave_map.risk_score((49, 49)));
    }
}
