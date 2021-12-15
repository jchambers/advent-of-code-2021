use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let cave_map = CaveMap::from_str(std::fs::read_to_string(path)?.as_str()).unwrap();

        println!("Total risk score along path to exit: {}", cave_map.path_risk_to_exit());

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
        let mut distances: HashMap<(usize, usize), u32> = HashMap::new();

        distances.insert((0, 0), 0);

        let exit = (
            self.risk_scores.len() - 1,
            self.risk_scores.last().unwrap().len() - 1,
        );

        while !visited_nodes.contains(&exit) {
            // Find the position of and best distance to the closest unexplored node
            // TODO This would be better with a priority queue
            let (&(row, col), &distance) = distances
                .iter()
                .filter(|&(position, _)| !visited_nodes.contains(position))
                .min_by(|(_, distance_a), (_, distance_b)| distance_a.cmp(distance_b))
                .unwrap();

            // Update the tentative distance to each unvisited neighbor
            self.neighbors(row, col)
                .iter()
                .filter(|&neighbor| !visited_nodes.contains(neighbor))
                .for_each(|&(neighbor_row, neighbor_col)| {
                    let distance_through_current_node =
                        self.risk_scores[neighbor_row][neighbor_col] as u32 + distance;

                    distances
                        .entry((neighbor_row, neighbor_col))
                        .and_modify(|d| *d = min(*d, distance_through_current_node))
                        .or_insert(distance + self.risk_scores[neighbor_row][neighbor_col] as u32);
                });

            visited_nodes.insert((row, col));
        }

        *distances.get(&exit).unwrap_or(&0u32)
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
