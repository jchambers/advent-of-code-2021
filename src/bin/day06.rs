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

        for _ in 0..80 {
            school.advance();
        }

        println!("Fish after 80 days: {}", school.get_population());

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct SchoolOfFish {
    fish_by_age: [u32; 9],
}

impl SchoolOfFish {
    pub fn new(ages: &[u8]) -> Self {
        let mut fish_by_age = [0; 9];

        ages.iter().for_each(|&age| fish_by_age[age as usize] += 1);

        SchoolOfFish { fish_by_age }
    }

    pub fn advance(&mut self) {
        let resetting = self.fish_by_age[0];

        for i in 0..self.fish_by_age.len() - 1 {
            self.fish_by_age[i] = self.fish_by_age[i + 1];
        }

        self.fish_by_age[6] += resetting;
        self.fish_by_age[8] = resetting;
    }

    pub fn get_population(&self) -> u32 {
        self.fish_by_age.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_advance() {
        let mut school = SchoolOfFish::new(&[3, 4, 3, 1, 2]);

        for _ in 0..18 {
            school.advance();
        }

        assert_eq!(26, school.get_population());

        for _ in 18..80 {
            school.advance();
        }

        assert_eq!(5934, school.get_population());
    }
}
