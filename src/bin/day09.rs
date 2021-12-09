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
    fn find_local_minima(&self) -> Vec<u8> {
        let mut local_minima = Vec::new();

        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                let height = self.heights[row][col];

                if row > 0 && self.heights[row - 1][col] <= height {
                    continue;
                }

                if row < self.heights.len() - 1 && self.heights[row + 1][col] <= height {
                    continue;
                }

                if col > 0 && self.heights[row][col - 1] <= height {
                    continue;
                }

                if col < self.heights[row].len() - 1 && self.heights[row][col + 1] <= height {
                    continue;
                }

                // If we made it this far, the height at (row, col) is a local minimum
                local_minima.push(height);
            }
        }

        local_minima
    }

    pub fn combined_risk_level_at_local_minima(&self) -> u32 {
        self.find_local_minima()
            .iter()
            .map(|height| (height + 1) as u32)
            .sum()
    }
}

impl FromStr for HeightMap {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights = s
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect())
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
            vec![1, 0, 5, 5],
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
}
