use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let positions: Vec<u32> = io::BufReader::new(File::open(path)?)
            .split(',' as u8)
            .map(|chunk| String::from_utf8(chunk.unwrap()).unwrap())
            .map(|position| u32::from_str(position.as_str()).unwrap())
            .collect();

        let flotilla = CrabFlotilla::new(positions);

        {
            let optimal_target = flotilla.optimal_alignment_target(&CrabFlotilla::linear_cost);

            println!(
                "Optimal alignment target with linear movement cost: {} ({} fuel)",
                optimal_target,
                flotilla.alignment_cost(optimal_target, &CrabFlotilla::linear_cost)
            );
        }

        {
            let optimal_target = flotilla.optimal_alignment_target(&CrabFlotilla::geometric_cost);

            println!(
                "Optimal alignment target with geometric movement cost: {} ({} fuel)",
                optimal_target,
                flotilla.alignment_cost(optimal_target, &CrabFlotilla::geometric_cost)
            );
        }

        Ok(())
    } else {
        Err("Usage: day07 INPUT_FILE_PATH".into())
    }
}

type CostFunction = dyn Fn(u32) -> u32;

struct CrabFlotilla {
    initial_positions: Vec<u32>,
}

impl CrabFlotilla {
    pub fn new(initial_positions: Vec<u32>) -> Self {
        CrabFlotilla { initial_positions }
    }

    pub fn optimal_alignment_target(&self, cost_function: &CostFunction) -> u32 {
        let min_position = *self.initial_positions.iter().min().unwrap() as usize;
        let max_position = *self.initial_positions.iter().max().unwrap() as usize;

        let mut best_position = (0, u32::MAX);

        for target in min_position..=max_position {
            let alignment_cost = self.alignment_cost(target as u32, cost_function);

            if alignment_cost < best_position.1 {
                best_position = (target, alignment_cost);
            }
        }

        best_position.0 as u32
    }

    pub fn alignment_cost(&self, target: u32, cost_function: &CostFunction) -> u32 {
        self.initial_positions
            .iter()
            .map(|position| {
                // Nightly has u32::abs_diff, which would make this much, much cleaner
                let abs_diff = i32::abs(*position as i32 - target as i32) as u32;
                cost_function(abs_diff)
            })
            .sum()
    }

    pub fn linear_cost(distance: u32) -> u32 {
        distance
    }

    pub fn geometric_cost(distance: u32) -> u32 {
        distance * (distance + 1) / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alignment_cost() {
        let flotilla = CrabFlotilla::new(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(41, flotilla.alignment_cost(1, &CrabFlotilla::linear_cost as &CostFunction));
        assert_eq!(37, flotilla.alignment_cost(2, &CrabFlotilla::linear_cost));
        assert_eq!(39, flotilla.alignment_cost(3, &CrabFlotilla::linear_cost));
        assert_eq!(71, flotilla.alignment_cost(10, &CrabFlotilla::linear_cost));
    }

    #[test]
    fn test_optimal_alignment_target() {
        let flotilla = CrabFlotilla::new(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(2, flotilla.optimal_alignment_target(&CrabFlotilla::linear_cost));
    }

    #[test]
    fn test_geometric_cost() {
        assert_eq!(1, CrabFlotilla::geometric_cost(1));
        assert_eq!(3, CrabFlotilla::geometric_cost(2));
        assert_eq!(6, CrabFlotilla::geometric_cost(3));
        assert_eq!(10, CrabFlotilla::geometric_cost(4));
        assert_eq!(15, CrabFlotilla::geometric_cost(5));
        assert_eq!(45, CrabFlotilla::geometric_cost(9));
        assert_eq!(66, CrabFlotilla::geometric_cost(11));
    }
}
