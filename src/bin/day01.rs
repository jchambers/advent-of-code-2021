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

        println!("Window size 1: {}", get_increase_count(&measurements, 1));
        println!("Window size 3: {}", get_increase_count(&measurements, 3));

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

fn get_increase_count(measurements: &[u32], window_size: usize) -> u32 {
    // It turns out that we don't actually need to sum the values in the given window; the change
    // from one position to the next will always be +newValue, -oldValue, and so we can get the same
    // result (is this an increase or not?) just by compairing the new value coming into the window
    // with the old value leaving the window.
    measurements[..=measurements.len() - window_size].iter()
        .zip(measurements[window_size..].iter())
        .filter(|(a, b)| b > a)
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_increase_count() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, get_increase_count(&depths, 1));
        assert_eq!(5, get_increase_count(&depths, 3));
    }
}