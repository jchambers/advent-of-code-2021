use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let ages: Vec<u8> = io::BufReader::new(File::open(path)?)
            .split(',' as u8)
            .map(|chunk| String::from_utf8(chunk.unwrap()).unwrap())
            .map(|age| u8::from_str(age.as_str()).unwrap())
            .collect();

        let mut school = SchoolOfFish::new(&ages);

        school.advance_to_time(80);
        println!("Fish after 80 days: {}", school.get_population());

        school.advance_to_time(256);
        println!("Fish after 256 days: {}", school.get_population());

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct SchoolOfFish {
    time: u32,
    fish_by_age: [u64; 9],
}

impl SchoolOfFish {
    pub fn new(ages: &[u8]) -> Self {
        let mut fish_by_age = [0; 9];

        ages.iter().for_each(|&age| fish_by_age[age as usize] += 1);

        SchoolOfFish { time: 0, fish_by_age }
    }

    pub fn advance_to_time(&mut self, time: u32) {
        while self.time < time {
            let resetting = self.fish_by_age[0];

            for i in 0..self.fish_by_age.len() - 1 {
                self.fish_by_age[i] = self.fish_by_age[i + 1];
            }

            self.fish_by_age[6] += resetting;
            self.fish_by_age[8] = resetting;

            self.time += 1;
        }
    }

    pub fn get_population(&self) -> u64 {
        self.fish_by_age.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_advance_to_time() {
        let mut school = SchoolOfFish::new(&[3, 4, 3, 1, 2]);

        school.advance_to_time(18);
        assert_eq!(26, school.get_population());

        school.advance_to_time(80);
        assert_eq!(5934, school.get_population());

        school.advance_to_time(256);
        assert_eq!(26984457539, school.get_population());
    }
}
