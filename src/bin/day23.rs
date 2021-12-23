use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _lines = io::BufReader::new(File::open(path)?).lines();
        let _input_string = std::fs::read_to_string(path)?;

        Ok(())
    } else {
        Err("Usage: day23 INPUT_FILE_PATH".into())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn destination_room_index(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    #[inline]
    fn destination_room_position(&self) -> usize {
        match self {
            Amphipod::A => 2,
            Amphipod::B => 4,
            Amphipod::C => 6,
            Amphipod::D => 8,
        }
    }

    #[inline]
    fn cost_to_move(&self, distance: usize) -> u32 {
        distance as u32
            * match self {
                Amphipod::A => 1,
                Amphipod::B => 10,
                Amphipod::C => 100,
                Amphipod::D => 1000,
            }
    }
}

// #############
// #...........#
// ###A#B#C#D###
//   #A#B#C#D#
//   #########
struct Burrow {
    // Hallway spaces are numbered left to right; the entrance to the first room is at 2, the second
    // is at 4, etc.
    hallway: [Option<Amphipod>; 11],

    // Rooms are numbered from left to right; A wants to be in 0, B in 1, etc.. Within a room,
    // spaces are numbered from the bottom up (0 is at the bottom, 1 is adjacent to the hallway).
    rooms: [[Option<Amphipod>; 2]; 4],
}

impl Burrow {
    pub fn new(rooms: [[Amphipod; 2]; 4]) -> Self {
        Burrow {
            hallway: [None; 11],
            rooms: rooms
                .iter()
                .map(|room| [Some(room[0]), Some(room[1])])
                .collect::<Vec<[Option<Amphipod>; 2]>>()
                .try_into()
                .unwrap(),
        }
    }

    fn min_cost_to_resolve(&self) -> u32 {
        // To calculate the minimum cost to get "home" for any amphipod, ignore collisions and just
        // assume that any amphipod can move immediately/directly to its destination. As a bit of a
        // hack, assume both amphipods are heading for the deepest part of the room, then "refund"
        // one such move.
        let mut cost = 0;

        // For anybody in the hallway, move laterally to the destination room, then two spaces in
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(position, maybe_amphipod)| maybe_amphipod.map(|a| (position, a)))
            .for_each(|(position, amphipod)| {
                // Plus two for the two spaces into the back of the room
                cost += amphipod.cost_to_move(Self::abs_diff(
                    position,
                    amphipod.destination_room_index() + 2,
                ))
            });

        // For everybody in a room, move out of the room if it's not the right one, then down the
        // hallway, then into the right room. If already in the right room, move toward the back.
        self.rooms.iter().enumerate().for_each(|(room, spaces)| {
            spaces
                .iter()
                .enumerate()
                .filter_map(|(position, maybe_amphipod)| maybe_amphipod.map(|a| (position, a)))
                .for_each(|(position, amphipod)| {
                    if room == amphipod.destination_room_index() {
                        // We're already in the right room and just need to move to the back
                        cost += amphipod.cost_to_move(position);
                    } else {
                        let spaces_to_leave_room = 2 - position;
                        let hallway_distance =
                            2 * Self::abs_diff(room, amphipod.destination_room_index());

                        cost += amphipod.cost_to_move(spaces_to_leave_room + hallway_distance + 2);
                    }
                });
        });

        // For the refund, we know we'll move an A amphipod by one space, a B by one space, and so
        // on. That adds up to 1111.
        cost - 1111
    }

    fn abs_diff(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
}

#[cfg(test)]
mod test {
    use super::Amphipod::*;
    use super::*;

    #[test]
    fn test_abs_diff() {
        assert_eq!(2, Burrow::abs_diff(5, 3));
        assert_eq!(2, Burrow::abs_diff(3, 5));
    }

    #[test]
    fn test_min_cost_to_resolve() {
        assert_eq!(0, Burrow::new([[A, A], [B, B], [C, C], [D, D]]).min_cost_to_resolve());

        // D moves 1 space out of the room, 6 spaces down the hall, and 1 space into the room for a
        // total of 8. A moves 1 out, 6 over, and 1 into the room (also 8).
        assert_eq!(8008, Burrow::new([[A, D], [B, B], [C, C], [D, A]]).min_cost_to_resolve());
    }
}
