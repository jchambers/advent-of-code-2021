use self::Instruction::*;
use std::cmp::{max, min};
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let instructions: Vec<Instruction> = io::BufReader::new(File::open(path)?)
            .lines()
            .map(|line| Instruction::from_str(line.unwrap().as_str()).unwrap())
            .collect();

        {
            let mut small_reactor = Reactor::new(Cuboid {
                x: CoordinateRange { start: -50, end: 50 },
                y: CoordinateRange { start: -50, end: 50 },
                z: CoordinateRange { start: -50, end: 50 },
            });

            instructions.iter()
                .for_each(|instruction| small_reactor.apply_instruction(&instruction));

            println!("Active cubes in small reactor: {}", small_reactor.active_cubes());
        }

        {
            let mut large_reactor = Reactor::new(Cuboid {
                x: CoordinateRange { start: i32::MIN, end: i32::MAX },
                y: CoordinateRange { start: i32::MIN, end: i32::MAX },
                z: CoordinateRange { start: i32::MIN, end: i32::MAX },
            });

            instructions.iter()
                .for_each(|instruction| large_reactor.apply_instruction(&instruction));

            println!("Active cubes in large reactor: {}", large_reactor.active_cubes());
        }

        Ok(())
    } else {
        Err("Usage: day22 INPUT_FILE_PATH".into())
    }
}

struct Reactor {
    bounds: Cuboid,
    active_cuboids: Vec<Cuboid>,
}

impl Reactor {
    pub fn new(bounds: Cuboid) -> Self {
        Reactor {
            bounds,
            active_cuboids: Vec::new(),
        }
    }

    pub fn active_cubes(&self) -> u64 {
        self.active_cuboids.iter()
            .map(Cuboid::volume)
            .sum()
    }

    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        if !self.bounds.contains(instruction.cuboid()) {
            return;
        }

        let intersecting_cuboids = self.take_intersecting_cuboids(instruction.cuboid());

        match instruction {
            On(cuboid) => {
                if intersecting_cuboids.is_empty() {
                    self.active_cuboids.push(*cuboid);
                } else {
                    self.active_cuboids.extend(cuboid.union(&intersecting_cuboids))
                }
            },
            Off(cuboid) => {
                for intersecting_cuboid in intersecting_cuboids {
                    self.active_cuboids.extend(intersecting_cuboid.subtract(cuboid));
                }
            },
        }
    }

    fn take_intersecting_cuboids(&mut self, cuboid: &Cuboid) -> Vec<Cuboid> {
        let mut removed = Vec::new();

        let mut i = 0;

        while i < self.active_cuboids.len() {
            if self.active_cuboids[i].intersects(cuboid) {
                removed.push(self.active_cuboids.remove(i));
            } else {
                i += 1;
            }
        }

        removed
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
        if self.x.start <= other.x.end && self.x.end >= other.x.start
            && self.y.start <= other.y.end && self.y.end >= other.y.start
            && self.z.start <= other.z.end && self.z.end >= other.z.start
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

    pub fn contains(&self, other: &Cuboid) -> bool {
        self.x.start <= other.x.start && self.x.end >= other.x.end
            && self.y.start <= other.y.start && self.y.end >= other.y.end
            && self.z.start <= other.z.start && self.z.end >= other.z.end
    }

    pub fn subtract(&self, other: &Cuboid) -> Vec<Cuboid> {
        if !self.intersects(other) {
            return vec![*self];
        }

        // Generally speaking, we can represent the difference between two cuboids with at most six
        // new cuboids. If we imagine taking a "core" out of the middle of a cuboid, we can
        // represent the difference as a "roof," a "ceiling," and four "walls" (left, right, front,
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

    pub fn union(&self, others: &[Cuboid]) -> Vec<Cuboid> {
        let mut union = Vec::new();

        for other in others {
            if let Some(intersection) = self.intersection(other) {
                // To avoid double-counting, subtract the overlapping bit from both cuboids, then add
                // it on its own.
                union.extend(other.subtract(&intersection));
            } else {
                union.push(*other);
            }
        }

        union.push(*self);

        union
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

impl Instruction {
    pub fn cuboid(&self) -> &Cuboid {
        match self {
            On(cuboid) => cuboid,
            Off(cuboid) => cuboid,
        }
    }
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
    use indoc::indoc;
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
    fn test_intersects() {
        let query_cuboid = Cuboid {
            x: CoordinateRange { start: 10, end: 10 },
            y: CoordinateRange { start: 10, end: 10 },
            z: CoordinateRange { start: 10, end: 10 },
        };

        assert!(!query_cuboid.intersects(&Cuboid {
            x: CoordinateRange { start: 10, end: 11 },
            y: CoordinateRange { start: 10, end: 10 },
            z: CoordinateRange { start: 12, end: 12 }
        }));

        assert!(!query_cuboid.intersects(&Cuboid {
            x: CoordinateRange { start: 10, end: 10 },
            y: CoordinateRange { start: 12, end: 12 },
            z: CoordinateRange { start: 10, end: 12 }
        }));

        assert!(!query_cuboid.intersects(&Cuboid {
            x: CoordinateRange { start: 10, end: 10 },
            y: CoordinateRange { start: 11, end: 11 },
            z: CoordinateRange { start: 12, end: 12 }
        }));
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
    fn test_contains() {
        let large = Cuboid {
            x: CoordinateRange { start: -1, end: 1 },
            y: CoordinateRange { start: -1, end: 1 },
            z: CoordinateRange { start: -1, end: 1 },
        };

        let small = Cuboid {
            x: CoordinateRange { start: 0, end: 0 },
            y: CoordinateRange { start: 0, end: 0 },
            z: CoordinateRange { start: 0, end: 0 },
        };

        assert!(large.contains(&small));
        assert!(large.contains(&large));
        assert!(small.contains(&small));
        assert!(!small.contains(&large));
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

        let intersecting = Cuboid {
            x: CoordinateRange { start: 0, end: 2 },
            y: CoordinateRange { start: 0, end: 2 },
            z: CoordinateRange { start: 0, end: 2 },
        };

        assert_eq!(
            46,
            original
                .union(&[intersecting])
                .iter()
                .map(Cuboid::volume)
                .sum::<u64>()
        );

        assert_eq!(
            46,
            intersecting
                .union(&[original])
                .iter()
                .map(Cuboid::volume)
                .sum::<u64>()
        );

        let non_intersecting = Cuboid {
            x: CoordinateRange { start: 3, end: 5 },
            y: CoordinateRange { start: 3, end: 5 },
            z: CoordinateRange { start: 3, end: 5 },
        };

        assert_eq!(
            54,
            original
                .union(&[non_intersecting])
                .iter()
                .map(Cuboid::volume)
                .sum::<u64>()
        );

        assert_eq!(
            54,
            non_intersecting
                .union(&[original])
                .iter()
                .map(Cuboid::volume)
                .sum::<u64>()
        );

        assert_eq!(
            73,
            original
                .union(&[intersecting, non_intersecting])
                .iter()
                .map(Cuboid::volume)
                .sum::<u64>()
        );

        {
            let small = Cuboid {
                x: CoordinateRange { start: 0, end: 0 },
                y: CoordinateRange { start: 0, end: 0 },
                z: CoordinateRange { start: 0, end: 0 },
            };

            assert_eq!(
                27,
                original
                    .union(&[small])
                    .iter()
                    .map(Cuboid::volume)
                    .sum::<u64>()
            );

            assert_eq!(
                27,
                small
                    .union(&[original])
                    .iter()
                    .map(Cuboid::volume)
                    .sum::<u64>()
            );
        }
    }

    #[test]
    fn test_apply_reactor_instructions() {
        let mut reactor = Reactor::new(Cuboid {
            x: CoordinateRange { start: -50, end: 50 },
            y: CoordinateRange { start: -50, end: 50 },
            z: CoordinateRange { start: -50, end: 50 },
        });

        assert_eq!(0, reactor.active_cubes());

        reactor.apply_instruction(&On(Cuboid {
            x: CoordinateRange { start: 10, end: 12 },
            y: CoordinateRange { start: 10, end: 12 },
            z: CoordinateRange { start: 10, end: 12 },
        }));

        assert_eq!(27, reactor.active_cubes());

        reactor.apply_instruction(&On(Cuboid {
            x: CoordinateRange { start: 11, end: 13 },
            y: CoordinateRange { start: 11, end: 13 },
            z: CoordinateRange { start: 11, end: 13 },
        }));

        assert_eq!(46, reactor.active_cubes());

        reactor.apply_instruction(&Off(Cuboid {
            x: CoordinateRange { start: 9, end: 11 },
            y: CoordinateRange { start: 9, end: 11 },
            z: CoordinateRange { start: 9, end: 11 },
        }));

        assert_eq!(38, reactor.active_cubes());

        reactor.apply_instruction(&On(Cuboid {
            x: CoordinateRange { start: 10, end: 10 },
            y: CoordinateRange { start: 10, end: 10 },
            z: CoordinateRange { start: 10, end: 10 },
        }));

        assert_eq!(39, reactor.active_cubes());
    }

    #[test]
    fn test_apply_reactor_instructions_complex() {
        let instruction_string = indoc! {"
            on x=-20..26,y=-36..17,z=-47..7
            on x=-20..33,y=-21..23,z=-26..28
            on x=-22..28,y=-29..23,z=-38..16
            on x=-46..7,y=-6..46,z=-50..-1
            on x=-49..1,y=-3..46,z=-24..28
            on x=2..47,y=-22..22,z=-23..27
            on x=-27..23,y=-28..26,z=-21..29
            on x=-39..5,y=-6..47,z=-3..44
            on x=-30..21,y=-8..43,z=-13..34
            on x=-22..26,y=-27..20,z=-29..19
            off x=-48..-32,y=26..41,z=-47..-37
            on x=-12..35,y=6..50,z=-50..-2
            off x=-48..-32,y=-32..-16,z=-15..-5
            on x=-18..26,y=-33..15,z=-7..46
            off x=-40..-22,y=-38..-28,z=23..41
            on x=-16..35,y=-41..10,z=-47..6
            off x=-32..-23,y=11..30,z=-14..3
            on x=-49..-5,y=-3..45,z=-29..18
            off x=18..30,y=-20..-8,z=-3..13
            on x=-41..9,y=-7..43,z=-33..15
            on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
            on x=967..23432,y=45373..81175,z=27513..53682
        "};

        let mut reactor = Reactor::new(Cuboid {
            x: CoordinateRange { start: -50, end: 50 },
            y: CoordinateRange { start: -50, end: 50 },
            z: CoordinateRange { start: -50, end: 50 },
        });

        instruction_string
            .lines()
            .map(|line| Instruction::from_str(line).unwrap())
            .for_each(|instruction| reactor.apply_instruction(&instruction));

        assert_eq!(590784, reactor.active_cubes());
    }
}
