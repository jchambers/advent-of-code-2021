use self::Instruction::*;
use std::cmp::{max, min};
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Cuboid {
    x: CoordinateRange,
    y: CoordinateRange,
    z: CoordinateRange,
}

impl Cuboid {
    pub fn volume(&self) -> u64 {
        self.x.len() * self.y.len() * self.z.len()
    }

    pub fn intersects(&self, other: &Cuboid) -> bool {
        self.intersection(other).is_some()
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        if self.x.start <= other.x.end
            && self.x.end >= other.x.start
            && self.x.start <= other.x.end
            && self.x.end >= other.x.start
            && self.x.start <= other.x.end
            && self.x.end >= other.x.start
        {
            Some(Cuboid {
                x: CoordinateRange {
                    start: max(self.x.start, other.x.start),
                    end: min(self.x.end, other.x.end),
                },
                y: CoordinateRange {
                    start: max(self.y.start, other.y.start),
                    end: min(self.y.end, other.y.end),
                },
                z: CoordinateRange {
                    start: max(self.z.start, other.z.start),
                    end: min(self.z.end, other.z.end),
                },
            })
        } else {
            None
        }
    }

    pub fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return vec![*self];
        }

        // Generally speaking, we can represent the difference between two cuboids with at most six
        // new cuboids. If we imagine taking a "core" out of the middle of a cuboid, we can
        // represent the difference as a "roof," a "ceiling," and four walls (left, right, front,
        // and back). Some—or all—of those may not be present.
        //
        // Let's say +x is to the right, +y is up, and +z is out of the screen toward the reader.

        let mut difference = Vec::new();

        // Roof
        if self.y.end > other.y.end {
            difference.push(Cuboid {
                x: self.x,
                y: CoordinateRange {
                    start: other.y.end + 1,
                    end: self.y.end,
                },
                z: self.z,
            });
        }

        // Floor
        if self.y.start < other.y.start {
            difference.push(Cuboid {
                x: self.x,
                y: CoordinateRange {
                    start: self.y.start,
                    end: other.y.start - 1,
                },
                z: self.z,
            });
        }

        let wall_y_range = CoordinateRange {
            start: max(self.y.start, other.y.start),
            end: min(self.y.end, other.y.end),
        };

        // Left wall
        if self.x.start < other.x.start {
            difference.push(Cuboid {
                x: CoordinateRange {
                    start: self.x.start,
                    end: other.x.start - 1,
                },
                y: wall_y_range,
                z: self.z,
            });
        }

        // Right wall
        if self.x.end > other.x.end {
            difference.push(Cuboid {
                x: CoordinateRange {
                    start: other.x.end + 1,
                    end: self.x.end,
                },
                y: wall_y_range,
                z: self.z,
            });
        }

        let wall_x_range = CoordinateRange {
            start: max(self.x.start, other.x.start),
            end: min(self.x.end, other.x.end),
        };

        // Back wall
        if self.z.start < other.z.start {
            difference.push(Cuboid {
                x: wall_x_range,
                y: wall_y_range,
                z: CoordinateRange {
                    start: self.z.start,
                    end: other.z.start - 1,
                },
            });
        }

        // Front wall
        if self.z.end > other.z.end {
            difference.push(Cuboid {
                x: wall_x_range,
                y: wall_y_range,
                z: CoordinateRange {
                    start: other.z.end + 1,
                    end: self.z.end,
                },
            });
        }

        difference
    }

    pub fn union(&self, other: &Cuboid) -> Vec<Cuboid> {
        if let Some(intersection) = self.intersection(other) {
            // To avoid double-counting, subtract the overlapping bit from both cuboids, then add
            // it on its own.
            let mut union = self.subtract(&intersection);
            union.extend(other.subtract(&intersection));
            union.push(intersection);

            union
        } else {
            vec![*self, *other]
        }
    }
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CoordinateRange {
    start: i32,
    end: i32,
}

impl CoordinateRange {
    fn len(&self) -> u64 {
        ((self.end - self.start) + 1) as u64
    }
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

    const CORNER_CUTS: [Cuboid; 8] = [
        Cuboid {
            x: CoordinateRange { start: 0, end: 1 },
            y: CoordinateRange { start: 0, end: 1 },
            z: CoordinateRange { start: 0, end: 1 },
        },
        Cuboid {
            x: CoordinateRange { start: 0, end: 1 },
            y: CoordinateRange { start: 0, end: 1 },
            z: CoordinateRange { start: -1, end: 0 },
        },
        Cuboid {
            x: CoordinateRange { start: 0, end: 1 },
            y: CoordinateRange { start: -1, end: 0 },
            z: CoordinateRange { start: 0, end: 1 },
        },
        Cuboid {
            x: CoordinateRange { start: -1, end: 0 },
            y: CoordinateRange { start: 0, end: 1 },
            z: CoordinateRange { start: 0, end: 1 },
        },
        Cuboid {
            x: CoordinateRange { start: 0, end: 1 },
            y: CoordinateRange { start: -1, end: 0 },
            z: CoordinateRange { start: -1, end: 0 },
        },
        Cuboid {
            x: CoordinateRange { start: -1, end: 0 },
            y: CoordinateRange { start: 0, end: 1 },
            z: CoordinateRange { start: -1, end: 0 },
        },
        Cuboid {
            x: CoordinateRange { start: -1, end: 0 },
            y: CoordinateRange { start: -1, end: 0 },
            z: CoordinateRange { start: 0, end: 1 },
        },
        Cuboid {
            x: CoordinateRange { start: -1, end: 0 },
            y: CoordinateRange { start: -1, end: 0 },
            z: CoordinateRange { start: -1, end: 0 },
        },
    ];

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

    #[test]
    fn test_volume() {
        assert_eq!(
            27,
            Cuboid {
                x: CoordinateRange { start: 10, end: 12 },
                y: CoordinateRange { start: 10, end: 12 },
                z: CoordinateRange { start: 10, end: 12 },
            }
            .volume()
        );
    }

    #[test]
    fn test_intersection() {
        let original = Cuboid {
            x: CoordinateRange { start: -1, end: 1 },
            y: CoordinateRange { start: -1, end: 1 },
            z: CoordinateRange { start: -1, end: 1 },
        };

        for corner_cut in CORNER_CUTS {
            assert_eq!(Some(corner_cut), original.intersection(&corner_cut));
        }

        assert!(original
            .intersection(&Cuboid {
                x: CoordinateRange { start: 2, end: 2 },
                y: CoordinateRange { start: 2, end: 2 },
                z: CoordinateRange { start: 2, end: 2 },
            })
            .is_none());
    }

    #[test]
    fn test_subtract() {
        let original = Cuboid {
            x: CoordinateRange { start: -1, end: 1 },
            y: CoordinateRange { start: -1, end: 1 },
            z: CoordinateRange { start: -1, end: 1 },
        };

        assert_eq!(27, original.volume());

        assert_eq!(
            vec![original],
            original.subtract(&Cuboid {
                x: CoordinateRange { start: 2, end: 2 },
                y: CoordinateRange { start: 2, end: 2 },
                z: CoordinateRange { start: 2, end: 2 },
            })
        );

        {
            let center_cut = Cuboid {
                x: CoordinateRange { start: 0, end: 0 },
                y: CoordinateRange { start: 0, end: 0 },
                z: CoordinateRange { start: 0, end: 0 },
            };

            assert_eq!(1, center_cut.volume());

            assert_eq!(6, original.subtract(&center_cut).len());
            assert_eq!(
                original.volume() - center_cut.volume(),
                original
                    .subtract(&center_cut)
                    .iter()
                    .map(Cuboid::volume)
                    .sum::<u64>()
            );
        }

        for corner_cut in CORNER_CUTS {
            assert_eq!(8, corner_cut.volume());

            assert_eq!(3, original.subtract(&corner_cut).len());
            assert_eq!(
                original.volume() - corner_cut.volume(),
                original
                    .subtract(&corner_cut)
                    .iter()
                    .map(Cuboid::volume)
                    .sum::<u64>()
            );
        }

        assert!(original.subtract(&original).is_empty());
    }

    #[test]
    fn test_union() {
        let original = Cuboid {
            x: CoordinateRange { start: -1, end: 1 },
            y: CoordinateRange { start: -1, end: 1 },
            z: CoordinateRange { start: -1, end: 1 },
        };

        {
            let intersecting = Cuboid {
                x: CoordinateRange { start: 0, end: 2 },
                y: CoordinateRange { start: 0, end: 2 },
                z: CoordinateRange { start: 0, end: 2 },
            };

            assert_eq!(
                46,
                original
                    .union(&intersecting)
                    .iter()
                    .map(Cuboid::volume)
                    .sum::<u64>()
            );
        }

        {
            let non_intersecting = Cuboid {
                x: CoordinateRange { start: 0, end: 2 },
                y: CoordinateRange { start: 0, end: 2 },
                z: CoordinateRange { start: 0, end: 2 },
            };

            assert_eq!(
                54,
                original
                    .union(&non_intersecting)
                    .iter()
                    .map(Cuboid::volume)
                    .sum::<u64>()
            );
        }
    }
}
