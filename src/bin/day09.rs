use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let height_map = HeightMap::from_str(std::fs::read_to_string(path)?.as_str())?;

        println!(
            "Total risk level at local minima: {}",
            height_map.combined_risk_level_at_local_minima()
        );

        println!("Product of 3 largest basin sizes: {}", height_map.largest_basin_size_product(3));

        Ok(())
    } else {
        Err("Usage: day09 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct HeightMap {
    heights: Vec<Vec<u8>>,
}

impl HeightMap {
    fn get_neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if row > 0 {
            neighbors.push((row - 1, col));
        }

        if row < self.heights.len() - 1 {
            neighbors.push((row + 1, col));
        }

        if col > 0 {
            neighbors.push((row, col - 1));
        }

        if col < self.heights[row].len() - 1 {
            neighbors.push((row, col + 1));
        }

        neighbors
    }

    fn find_local_minima(&self) -> Vec<(usize, usize)> {
        let mut local_minima = Vec::new();

        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                if !self
                    .get_neighbors(row, col)
                    .iter()
                    .any(|&(neighbor_row, neighbor_col)| {
                        self.heights[neighbor_row][neighbor_col] <= self.heights[row][col]
                    })
                {
                    local_minima.push((row, col));
                }
            }
        }

        local_minima
    }

    pub fn combined_risk_level_at_local_minima(&self) -> u32 {
        self.find_local_minima()
            .iter()
            .map(|&(row, col)| (self.heights[row][col] + 1) as u32)
            .sum()
    }

    fn basin_members(&self, origin_row: usize, origin_col: usize) -> Vec<(usize, usize)> {
        if self.heights[origin_row][origin_col] == 9 {
            return vec![];
        }

        let mut explored = HashSet::new();
        let mut queue = VecDeque::new();
        let mut basin_members = Vec::new();

        queue.push_front((origin_row, origin_col));

        while let Some((row, col)) = queue.pop_front() {
            if explored.insert((row, col)) && self.heights[row][col] < 9 {
                basin_members.push((row, col));

                queue.extend(self.get_neighbors(row, col).iter().filter(
                    |&&(neighbor_row, neighbor_col)| {
                        !explored.contains(&(neighbor_row, neighbor_col))
                    },
                ));
            }
        }

        basin_members
    }

    pub fn largest_basin_size_product(&self, n: usize) -> u32 {
        let mut basin_sizes: Vec<u32> = self
            .find_local_minima()
            .iter()
            .map(|&(row, col)| self.basin_members(row, col).len() as u32)
            .collect();

        basin_sizes.sort_by(|a, b| b.cmp(a));

        basin_sizes.iter().take(n).product()
    }
}

impl FromStr for HeightMap {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights = s
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
            .collect();

        Ok(HeightMap { heights })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    static EXAMPLE_MAP_STRING: &str = indoc! {"
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        "};

    #[test]
    fn test_height_map_from_string() {
        let expected = HeightMap {
            heights: vec![
                vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
                vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
                vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
                vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
                vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
            ],
        };

        assert_eq!(expected, HeightMap::from_str(EXAMPLE_MAP_STRING).unwrap());
    }

    #[test]
    fn test_find_local_minima() {
        assert_eq!(
            vec![(0, 1), (0, 9), (2, 2), (4, 6)],
            HeightMap::from_str(EXAMPLE_MAP_STRING)
                .unwrap()
                .find_local_minima()
        );
    }

    #[test]
    fn test_combined_risk_level_at_local_minima() {
        assert_eq!(
            15,
            HeightMap::from_str(EXAMPLE_MAP_STRING)
                .unwrap()
                .combined_risk_level_at_local_minima()
        );
    }

    #[test]
    fn test_find_basin_members() {
        let height_map = HeightMap::from_str(EXAMPLE_MAP_STRING).unwrap();

        assert_eq!(3, height_map.basin_members(0, 1).len());
        assert_eq!(9, height_map.basin_members(0, 9).len());
        assert_eq!(14, height_map.basin_members(2, 2).len());
        assert_eq!(9, height_map.basin_members(4, 6).len());
    }

    #[test]
    fn test_largest_basin_size_product() {
        let height_map = HeightMap::from_str(EXAMPLE_MAP_STRING).unwrap();

        assert_eq!(1134, height_map.largest_basin_size_product(3));
    }
}
