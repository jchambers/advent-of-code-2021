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
            cave_map.path_risk_to_exit()
        );

        Ok(())
    } else {
        Err("Usage: day15 INPUT_FILE_PATH".into())
    }
}

struct CaveMap {
    risk_scores: Vec<Vec<u8>>,
}

impl CaveMap {
    pub fn path_risk_to_exit(&self) -> u32 {
        let mut visited_nodes = HashSet::new();
        let mut tentative_distances = BinaryHeap::new();

        tentative_distances.push(NodeAndDistance::new((0, 0), 0, self.risk_scores.len()));

        let exit = (
            self.risk_scores.len() - 1,
            self.risk_scores.last().unwrap().len() - 1,
        );

        while let Some(node_and_distance) = tentative_distances.pop() {
            if visited_nodes.contains(&(node_and_distance.row, node_and_distance.col)) {
                continue;
            }

            if (node_and_distance.row, node_and_distance.col) == exit {
                return node_and_distance.distance;
            }

            // Update the tentative distance to each unvisited neighbor
            self.neighbors(node_and_distance.row, node_and_distance.col)
                .iter()
                .filter(|&neighbor| !visited_nodes.contains(neighbor))
                .for_each(|&(neighbor_row, neighbor_col)| {
                    let tentative_distance = self.risk_scores[neighbor_row][neighbor_col] as u32
                        + node_and_distance.distance;

                    tentative_distances.push(NodeAndDistance::new(
                        (neighbor_row, neighbor_col),
                        tentative_distance,
                        self.risk_scores.len(),
                    ));
                });

            visited_nodes.insert((node_and_distance.row, node_and_distance.col));
        }

        u32::MAX
    }

    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if row > 0 {
            neighbors.push((row - 1, col));
        }

        if row < self.risk_scores.len() - 1 {
            neighbors.push((row + 1, col));
        }

        if col > 0 {
            neighbors.push((row, col - 1));
        }

        if col < self.risk_scores[row].len() - 1 {
            neighbors.push((row, col + 1));
        }

        neighbors
    }
}

#[derive(Eq, PartialEq)]
struct NodeAndDistance {
    distance: u32,
    row: usize,
    col: usize,

    // Borrowing from https://doc.rust-lang.org/std/collections/binary_heap/index.html, we need this
    // as a tie-breaker if two nodes have the same distance. We'll call "position" the array index
    // as if all of the nodes were laid out in a one-dimensional array, so this is
    // `row * ROW_WIDTH + col`.
    position: usize,
}

impl NodeAndDistance {
    pub fn new(position: (usize, usize), distance: u32, row_width: usize) -> Self {
        let (row, col) = position;

        Self {
            distance,
            row,
            col,
            position: (row * row_width) + col,
        }
    }
}

impl Ord for NodeAndDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        // We want two things that are slightly different from the default:
        //
        // 1. We want min-first ordering
        // 2. We want to be able to break ties with position
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
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

        assert_eq!(40, cave_map.path_risk_to_exit());
    }
}
