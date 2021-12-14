use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let respawn_times: Vec<u8> = io::BufReader::new(File::open(path)?)
            .split(b',')
            .map(|chunk| String::from_utf8(chunk.unwrap()).unwrap())
            .map(|respawn_time| u8::from_str(respawn_time.as_str()).unwrap())
            .collect();

        let school = SchoolOfFish::new(&respawn_times);

        println!("Fish after 80 days: {}", school.get_population(80));
        println!("Fish after 256 days: {}", school.get_population(256));

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}

struct SchoolOfFish {
    fish_by_initial_respawn_time: [u64; 9],
}

impl SchoolOfFish {
    pub fn new(respawn_times: &[u8]) -> Self {
        let mut fish_by_initial_respawn_time = [0; 9];

        respawn_times
            .iter()
            .for_each(|&respawn_time| fish_by_initial_respawn_time[respawn_time as usize] += 1);

        SchoolOfFish { fish_by_initial_respawn_time }
    }

    pub fn get_population(&self, day: u32) -> u64 {
        let mut fish_by_respawn_time = self.fish_by_initial_respawn_time.to_vec();

        for _ in 0..day {
            fish_by_respawn_time.rotate_left(1);
            fish_by_respawn_time[6] += fish_by_respawn_time[8];
        }

        fish_by_respawn_time.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_advance_to_day() {
        let school = SchoolOfFish::new(&[3, 4, 3, 1, 2]);

        assert_eq!(26, school.get_population(18));
        assert_eq!(5934, school.get_population(80));
        assert_eq!(26984457539, school.get_population(256));
    }
}
