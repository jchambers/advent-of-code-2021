use self::Instruction::*;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _instructions: Vec<Instruction> = io::BufReader::new(File::open(path)?)
            .lines()
            .map(|line| Instruction::from_str(line.unwrap().as_str()).unwrap())
            .collect();

        Ok(())
    } else {
        Err("Usage: day22 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Cuboid {
    x: CoordinateRange,
    y: CoordinateRange,
    z: CoordinateRange,
}

impl Cuboid {}

impl FromStr for Cuboid {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let x = {
            let range_start = string.find("x=").unwrap() + 2;
            let range_end = string[range_start..].find(',').unwrap() + range_start;

            CoordinateRange::from_str(&string[range_start..range_end])?
        };

        let y = {
            let range_start = string.find("y=").unwrap() + 2;
            let range_end = string[range_start..].find(',').unwrap() + range_start;

            CoordinateRange::from_str(&string[range_start..range_end])?
        };

        let z = {
            let range_start = string.find("z=").unwrap() + 2;

            CoordinateRange::from_str(&string[range_start..])?
        };

        Ok(Cuboid { x, y, z })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct CoordinateRange {
    start: i32,
    end: i32,
}

impl FromStr for CoordinateRange {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut pieces = string.split("..");

        let start = i32::from_str(pieces.next().unwrap())?;
        let end = i32::from_str(pieces.next().unwrap())?;

        Ok(CoordinateRange { start, end })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    On(Cuboid),
    Off(Cuboid),
}

impl FromStr for Instruction {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.starts_with("on ") {
            Ok(On(Cuboid::from_str(&string[3..])?))
        } else if string.starts_with("off ") {
            Ok(Off(Cuboid::from_str(&string[4..])?))
        } else {
            Err(format!("Could not parse instruction: {}", string).into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_from_string() {
        assert_eq!(
            CoordinateRange {
                start: -44,
                end: 17
            },
            CoordinateRange::from_str("-44..17").unwrap()
        );
    }

    #[test]
    fn test_cuboid_from_string() {
        let expected = Cuboid {
            x: CoordinateRange {
                start: -54112,
                end: -39298,
            },
            y: CoordinateRange {
                start: -85059,
                end: -49293,
            },
            z: CoordinateRange {
                start: -27449,
                end: 7877,
            },
        };

        assert_eq!(
            expected,
            Cuboid::from_str("x=-54112..-39298,y=-85059..-49293,z=-27449..7877").unwrap()
        );
    }

    #[test]
    fn test_instruction_from_string() {
        assert_eq!(
            On(Cuboid {
                x: CoordinateRange { start: 11, end: 13 },
                y: CoordinateRange { start: 11, end: 13 },
                z: CoordinateRange { start: 11, end: 13 },
            }),
            Instruction::from_str("on x=11..13,y=11..13,z=11..13").unwrap()
        );

        assert_eq!(
            Off(Cuboid {
                x: CoordinateRange { start: 9, end: 11 },
                y: CoordinateRange { start: 9, end: 11 },
                z: CoordinateRange { start: 9, end: 11 },
            }),
            Instruction::from_str("off x=9..11,y=9..11,z=9..11").unwrap()
        );
    }
}
