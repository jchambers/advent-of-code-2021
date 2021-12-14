use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let values: Vec<String> = io::BufReader::new(file)
            .lines()
            .map(|line| line.unwrap())
            .collect();

        println!("Power consumption: {}", get_power_consumption(&values));
        println!("Life support rating: {}", get_life_support_rating(&values));

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

fn get_power_consumption(values: &[String]) -> u32 {
    let width = values[0].len();
    let gamma = get_gamma(values);

    let mut mask = 0;

    for _ in 0..width {
        mask = mask << 1 | 1;
    }

    let epsilon = !gamma & mask;

    gamma * epsilon
}

fn get_gamma(values: &[String]) -> u32 {
    let mut ones = vec![0u32; values[0].len()];

    for value in values {
        for (i, c) in value.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }

    ones.iter().fold(0, |gamma, count| {
        // One weird thing here: the problem input has an even number of values, and the problem
        // doesn't explicitly say what to do if we have a tie. "Greater than half" gives us the
        // right answer, but I think that only works because the input is deliberately crafted to
        // avoid ties.
        if *count as usize > values.len() / 2 {
            (gamma << 1) | 1
        } else {
            gamma << 1
        }
    })
}

fn get_life_support_rating(values: &[String]) -> u32 {
    get_oxygen_generator_rating(values) * get_co2_scrubber_rating(values)
}

fn get_oxygen_generator_rating(values: &[String]) -> u32 {
    get_life_support_value(values, 0, |ones_at_position, len| {
        if ones_at_position > len / 2 || (len % 2 == 0 && ones_at_position == len / 2) {
            '1'
        } else {
            '0'
        }
    })
}

fn get_co2_scrubber_rating(values: &[String]) -> u32 {
    get_life_support_value(values, 0, |ones_at_position, len| {
        if ones_at_position > len / 2 || (len % 2 == 0 && ones_at_position == len / 2) {
            '0'
        } else {
            '1'
        }
    })
}

fn get_life_support_value(
    values: &[String],
    position: usize,
    get_target_digit: impl Fn(usize, usize) -> char,
) -> u32 {
    if values.len() > 1 {
        let ones_at_position = values
            .iter()
            .filter(|value| value.chars().nth(position) == Some('1'))
            .count();

        let target_digit = get_target_digit(ones_at_position, values.len());

        let filtered_values: Vec<String> = values
            .iter()
            .filter(|value| value.chars().nth(position) == Some(target_digit))
            .cloned()
            .collect();

        get_life_support_value(&filtered_values, position + 1, get_target_digit)
    } else {
        u32::from_str_radix(values[0].as_str(), 2).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static VALUES: [&str; 12] = [
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];

    #[test]
    fn test_get_gamma() {
        let values: Vec<String> = VALUES.iter().map(|v| String::from(*v)).collect();

        assert_eq!(22, get_gamma(&values));
    }

    #[test]
    fn test_get_power_consumption() {
        let values: Vec<String> = VALUES.iter().map(|v| String::from(*v)).collect();

        assert_eq!(198, get_power_consumption(&values));
    }

    #[test]
    fn test_get_oxygen_generator_rating() {
        let values: Vec<String> = VALUES.iter().map(|v| String::from(*v)).collect();

        assert_eq!(23, get_oxygen_generator_rating(&values));
    }

    #[test]
    fn test_get_co2_scrubber_rating() {
        let values: Vec<String> = VALUES.iter().map(|v| String::from(*v)).collect();

        assert_eq!(10, get_co2_scrubber_rating(&values));
    }

    #[test]
    fn test_get_life_support_rating() {
        let values: Vec<String> = VALUES.iter().map(|v| String::from(*v)).collect();

        assert_eq!(230, get_life_support_rating(&values));
    }
}
