use std::{env, error, io};
use std::fs::File;
use std::io::BufRead;
use crate::Command::{Down, Forward, Up};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let commands: Vec<Command> = io::BufReader::new(file).lines()
            .filter_map(|line| Command::try_from(line.unwrap().as_str()).ok())
            .collect();

        let (horizontal, depth) = get_distance(&commands);

        println!("{} × {} = {}", horizontal, depth, horizontal * depth);

        Ok(())
    } else {
        Err("Usage: day02 INPUT_FILE".into())
    }
}

fn get_distance(commands: &[Command]) -> (u32, u32) {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Forward(h) => horizontal += h,
            Down(d) => depth += d,
            Up(d) => depth -= d
        };
    }

    (horizontal, depth)
}

#[derive(Debug, Eq, PartialEq)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32)
}

impl TryFrom<&str> for Command {
    type Error = &'static str;

    fn try_from(command: &str) -> Result<Self, Self::Error> {
        let pieces: Vec<&str> = command.split(" ").collect();

        if pieces.len() != 2 {
            return Err("Expected 2 pieces".into());
        }

        if let Ok(magnitude) = pieces[1].parse::<u32>() {
            match pieces[0] {
                "forward" => Ok(Forward(magnitude)),
                "down" => Ok(Down(magnitude)),
                "up" => Ok(Up(magnitude)),
                _ => Err("OH NO".into())
            }
        } else {
            Err("Could not parse magnitude".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_from_string() {
        assert_eq!(Ok(Forward(1)), Command::try_from("forward 1"));
        assert_eq!(Ok(Up(17)), Command::try_from("up 17"));
        assert_eq!(Ok(Down(22)), Command::try_from("down 22"));
        assert!(Command::try_from("OH NO").is_err());
    }

    #[test]
    fn test_get_distance() {
        let commands = [
            Forward(5),
            Down(5),
            Forward(8),
            Up(3),
            Down(8),
            Forward(2)
        ];

        assert_eq!((15, 10), get_distance(&commands));
    }
}