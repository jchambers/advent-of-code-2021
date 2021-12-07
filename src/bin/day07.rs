use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let positions: Vec<i32> = io::BufReader::new(File::open(path)?)
            .split(',' as u8)
            .map(|chunk| String::from_utf8(chunk.unwrap()).unwrap())
            .map(|position| i32::from_str(position.as_str()).unwrap())
            .collect();

        let flotilla = CrabFlotilla::new(positions);
        let optimal_target = flotilla.optimal_alignment_target();

        println!(
            "Optimal alignment target: {} ({} fuel)",
            optimal_target,
            flotilla.alignment_cost(optimal_target)
        );

        Ok(())
    } else {
        Err("Usage: day07 INPUT_FILE_PATH".into())
    }
}

struct CrabFlotilla {
    initial_positions: Vec<i32>,
}

impl CrabFlotilla {
    pub fn new(initial_positions: Vec<i32>) -> Self {
        CrabFlotilla { initial_positions }
    }

    pub fn optimal_alignment_target(&self) -> i32 {
        let min_position = *self.initial_positions.iter().min().unwrap() as usize;
        let max_position = *self.initial_positions.iter().max().unwrap() as usize;

        let mut best_position = (0, i32::MAX);

        for target in min_position..=max_position {
            let alignment_cost = self.alignment_cost(target as i32);

            if alignment_cost < best_position.1 {
                best_position = (target, alignment_cost);
            }
        }

        best_position.0 as i32
    }

    pub fn alignment_cost(&self, target: i32) -> i32 {
        self.initial_positions
            .iter()
            .map(|position| i32::abs(position - target))
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_alignment_cost() {
        let flotilla = CrabFlotilla::new(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(41, flotilla.alignment_cost(1));
        assert_eq!(37, flotilla.alignment_cost(2));
        assert_eq!(39, flotilla.alignment_cost(3));
        assert_eq!(71, flotilla.alignment_cost(10));
    }

    #[test]
    fn test_optimal_alignment_target() {
        let flotilla = CrabFlotilla::new(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);

        assert_eq!(2, flotilla.optimal_alignment_target());
    }
}
