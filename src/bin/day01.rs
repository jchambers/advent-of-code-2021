use std::{env, error, io};
use std::fs::File;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let measurements: Vec<u32> = io::BufReader::new(file).lines()
            .filter_map(|line| line.unwrap().parse().ok())
            .collect();

        println!("{}", get_increase_count(&measurements));

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

fn get_increase_count(measurements: &[u32]) -> u32 {
    let mut increases = 0;

    for i in 1..measurements.len() {
        if measurements[i] > measurements[i - 1] {
            increases += 1;
        }
    }

    increases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_increase_count() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, get_increase_count(&depths));
    }
}