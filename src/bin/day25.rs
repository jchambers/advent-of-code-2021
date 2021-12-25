use self::Space::*;
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let map = SeaCucumberMap::from_str(std::fs::read_to_string(path)?.as_str())?;

        println!("Time to settle: {} steps", map.time_to_settle());

        Ok(())
    } else {
        Err("Usage: day25 INPUT_FILE_PATH".into())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Space {
    Empty,
    East,
    South,
}

impl FromStr for Space {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Empty),
            ">" => Ok(East),
            "v" => Ok(South),
            _ => Err(format!("Unrecognized space string: {}", s).into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SeaCucumberMap {
    spaces: Vec<Vec<Space>>,
}

impl SeaCucumberMap {
    pub fn time_to_settle(&self) -> u32 {
        let mut previous = self.clone();
        let mut steps = 0;

        loop {
            steps += 1;
            let next = previous.next();

            if next == previous {
                return steps;
            }

            previous = next;
        }
    }

    fn next(&self) -> Self {
        let mut next_spaces = self.spaces.clone();

        let width = next_spaces[0].len();
        let height = next_spaces.len();

        {
            let mut east_movers = Vec::new();

            for row in 0..height {
                for col in 0..width {
                    if next_spaces[row][col] == East && next_spaces[row][(col + 1) % width] == Empty
                    {
                        east_movers.push((row, col));
                    }
                }
            }

            for (row, col) in east_movers {
                next_spaces[row][col] = Empty;
                next_spaces[row][(col + 1) % width] = East;
            }
        }

        {
            let mut south_movers = Vec::new();

            for row in 0..height {
                for col in 0..width {
                    if next_spaces[row][col] == South
                        && next_spaces[(row + 1) % height][col] == Empty
                    {
                        south_movers.push((row, col));
                    }
                }
            }

            for (row, col) in south_movers {
                next_spaces[row][col] = Empty;
                next_spaces[(row + 1) % height][col] = South;
            }
        }

        SeaCucumberMap {
            spaces: next_spaces,
        }
    }
}

impl FromStr for SeaCucumberMap {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| Space::from_str(c.to_string().as_str()))
                    .collect::<Result<Vec<Space>, Box<dyn error::Error>>>()
            })
            .collect::<Result<Vec<Vec<Space>>, Box<dyn error::Error>>>()
            .map(|spaces| SeaCucumberMap { spaces })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_map_from_string() {
        let map_string = indoc! {"
            ...
            .>.
            ..v
        "};

        let expected = SeaCucumberMap {
            spaces: vec![
                vec![Empty, Empty, Empty],
                vec![Empty, East, Empty],
                vec![Empty, Empty, South],
            ],
        };

        assert_eq!(expected, SeaCucumberMap::from_str(map_string).unwrap());
    }

    #[test]
    fn test_next() {
        let map_strings: [&str; 5] = [
            indoc! {"
                ...>...
                .......
                ......>
                v.....>
                ......>
                .......
                ..vvv..
            "},
            indoc! {"
                ..vv>..
                .......
                >......
                v.....>
                >......
                .......
                ....v..
            "},
            indoc! {"
                ....v>.
                ..vv...
                .>.....
                ......>
                v>.....
                .......
                .......
            "},
            indoc! {"
                ......>
                ..v.v..
                ..>v...
                >......
                ..>....
                v......
                .......
            "},
            indoc! {"
                >......
                ..v....
                ..>.v..
                .>.v...
                ...>...
                .......
                v......
            "},
        ];

        let mut map = SeaCucumberMap::from_str(map_strings[0]).unwrap();

        for expected_next_map in &map_strings[1..] {
            let expected_next_map = SeaCucumberMap::from_str(expected_next_map).unwrap();
            map = map.next();

            assert_eq!(expected_next_map, map);
        }
    }

    #[test]
    fn test_time_to_settle() {
        let map = SeaCucumberMap::from_str(indoc! {"
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        "})
        .unwrap();

        assert_eq!(58, map.time_to_settle());
    }
}
