use self::Amphipod::*;
use self::Position::*;
use std::collections::HashMap;
use std::{env, error};
use std::fmt::{Display, Formatter};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _input_string = std::fs::read_to_string(path)?;

        Ok(())
    } else {
        Err("Usage: day23 INPUT_FILE_PATH".into())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost_to_move(&self, distance: u32) -> u32 {
        distance as u32
            * match self {
                A => 1,
                B => 10,
                C => 100,
                D => 1000,
            }
    }
}

impl ToString for Amphipod {
    fn to_string(&self) -> String {
        match self {
            A => String::from("A"),
            B => String::from("B"),
            C => String::from("C"),
            D => String::from("D"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Position {
    // Hallway spaces are numbered left to right; the entrance to the first room is at 2, the second
    // is at 4, etc.
    Hallway(u32),

    // Rooms are labeled by the kind of amphipod that wants to "live" there; within a room, spaces
    // are numbered from the top down (0 is adjacent to the hallway, 1 is at the bottom).
    Room(Amphipod, u32),
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> u32 {
        match (self, other) {
            (&Hallway(start), &Hallway(dest)) => Self::abs_diff(start, dest),
            (&Hallway(start), &Room(dest_amphipod, dest_position)) => {
                let within_hallway =
                    Self::abs_diff(start, Self::room_position_in_hallway(dest_amphipod));
                let from_hallway = dest_position + 1;

                within_hallway + from_hallway
            }
            (&Room(start_amphipod, start_position), &Room(dest_amphipod, dest_position)) => {
                if start_amphipod == dest_amphipod {
                    Self::abs_diff(start_position, dest_position)
                } else {
                    let to_hallway = start_position + 1;
                    let within_hallway = Self::abs_diff(
                        Self::room_position_in_hallway(start_amphipod),
                        Self::room_position_in_hallway(dest_amphipod),
                    );
                    let from_hallway = dest_position + 1;

                    to_hallway + within_hallway + from_hallway
                }
            }
            (&Room(start_amphipod, start_position), &Hallway(dest)) => {
                let to_hallway = start_position + 1;
                let within_hallway =
                    Self::abs_diff(Self::room_position_in_hallway(start_amphipod), dest);

                to_hallway + within_hallway
            }
        }
    }

    pub fn room_position_in_hallway(amphipod: Amphipod) -> u32 {
        match amphipod {
            A => 2,
            B => 4,
            C => 6,
            D => 8,
        }
    }

    fn abs_diff(a: u32, b: u32) -> u32 {
        if a > b {
            a - b
        } else {
            b - a
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Burrow {
    // Each amphipod has a permanent slot in this array. The first two are A1 and A2, the next are
    // B1 and B2, and so on.
    positions: [Position; 8],
}

impl Burrow {
    // Amphipods are given in order of their starting positions from left to right and top to
    // bottom. Example:
    //
    // #############
    // #...........#
    // ###B#C#B#D###  -> B, A, C, D, B, C, D, A
    //   #A#D#C#A#
    //   #########
    pub fn new(initial_positions: [Amphipod; 8]) -> Self {
        let mut amphipods_placed: HashMap<Amphipod, usize> =
            vec![(A, 0), (B, 0), (C, 0), (D, 0)].into_iter().collect();

        // Hack; pre-populate the position array with nonsense locations while we let things settle
        let mut positions = [Hallway(0); 8];
        let mut i = 0;

        for room in [A, B, C, D] {
            for space in [0, 1] {
                // We know we're traversing the room spaces in order…
                let position = Room(room, space);

                // …but now we need to figure out which position that maps to in the burrow's
                // position array (i.e. which amphipod is in that space).
                let room_index = match initial_positions[i] {
                    A => 0,
                    B => 2,
                    C => 4,
                    D => 6,
                };

                let index = room_index + amphipods_placed.get(&initial_positions[i]).unwrap();

                positions[index] = position;

                amphipods_placed
                    .entry(initial_positions[i])
                    .and_modify(|placed| *placed += 1);
                i += 1;
            }
        }

        assert!(positions
            .iter()
            .all(|position| matches!(position, Room(_, _))));

        Self { positions }
    }

    fn min_cost_to_resolve(&self) -> u32 {
        // To calculate the minimum cost to get "home" for any amphipod, ignore collisions and just
        // assume that any amphipod can move immediately/directly to its destination. As a bit of a
        // hack, assume both amphipods are heading for the deepest part of the room, then "refund"
        // one such move.
        let cost: u32 = self
            .positions
            .iter()
            .enumerate()
            .map(|(i, position)| {
                let amphipod = Self::amphipod_at_position_index(i);
                let destination = Room(amphipod, 1);

                amphipod.cost_to_move(position.distance_to(&destination))
            })
            .sum();

        // For the "refund," we know we'll move an A amphipod by one space, a B by one space, and so
        // on. That adds up to 1111.
        cost - 1111
    }

    fn amphipod_at_position_index(index: usize) -> Amphipod {
        match index {
            0 | 1 => A,
            2 | 3 => B,
            4 | 5 => C,
            _ => D,
        }
    }

    fn next_possible_states(&self) -> Vec<(Self, u32)> {
        const LEGAL_HALLWAY_STOPS: [u32; 7] = [0, 1, 3, 5, 7, 9, 10];

        let mut next_possible_states = Vec::new();

        self.positions.iter().enumerate().for_each(|(i, position)| {
            let amphipod = Self::amphipod_at_position_index(i);

            match position {
                &Hallway(h) => {
                    // If we're in the hallway, the only legal move is into our target room if
                    // it's empty or if it has one occupant of the correct type.
                    let destination_room = Position::room_position_in_hallway(amphipod);

                    if self.hallway_path_clear(h, destination_room) {
                        if let Some(space) = self.destination_space_within_room(amphipod) {
                            next_possible_states.push(self.with_move(i, Room(amphipod, space)));
                        }
                    }
                }
                &Room(room, space) => {
                    // If we're in a room, there are a few possibilities:
                    //
                    // 1. We're in the right room, but are blocking somebody who needs to get
                    // out, and we should move
                    // 2. We're in the right room and are either in the back of the room or at
                    // the front of the room with a roommate of the correct type, and we
                    // shouldn't move
                    // 3. We're in the wrong room and should move
                    let should_move = if amphipod == room {
                        // We're in the right room, but are we blocking somebody who wants to
                        // get out?
                        if space == 0 {
                            // We're at the front of the room, so somebody MUST be at the back
                            // of the room
                            self.room_occupants(room)[1].unwrap() != room
                        } else {
                            // We're at the back of the room and should never move
                            false
                        }
                    } else {
                        // We're in the wrong room and should definitely move
                        true
                    };

                    if should_move {
                        // We know we SHOULD move, but can we?
                        if self.path_to_hallway_clear(room, space) {
                            // If we can make it into our target room, do it and consider no
                            // other possible moves
                            let destination_space = self.destination_space_within_room(amphipod);
                            let start_room_position = Position::room_position_in_hallway(room);
                            let dest_room_position = Position::room_position_in_hallway(amphipod);

                            if destination_space.is_some()
                                && self.hallway_path_clear(start_room_position, dest_room_position)
                            {
                                next_possible_states.push(
                                    self.with_move(i, Room(amphipod, destination_space.unwrap())),
                                );
                            } else {
                                // Looks like we're moving to the hallway instead
                                for dest_hallway_position in LEGAL_HALLWAY_STOPS {
                                    if self.hallway_path_clear(
                                        start_room_position,
                                        dest_hallway_position,
                                    ) {
                                        next_possible_states.push(
                                            self.with_move(i, Hallway(dest_hallway_position)),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        next_possible_states
    }

    fn hallway_path_clear(&self, start: u32, destination: u32) -> bool {
        let mut hallway = [true; 11];

        for position in self.positions {
            if let Hallway(h) = position {
                hallway[h as usize] = false;
            }
        }

        let hallway_slice = if start < destination {
            &hallway[(start + 1) as usize..=destination as usize]
        } else {
            &hallway[destination as usize..start as usize]
        };

        hallway_slice.iter().all(|&space| space)
    }

    fn path_to_hallway_clear(&self, room: Amphipod, space: u32) -> bool {
        if space == 0 {
            true
        } else {
            // We're in the back of the room; is anybody in the front?
            let front_of_room = Room(room, 0);

            !self.positions.iter().any(|&p| p == front_of_room)
        }
    }

    fn room_occupants(&self, room: Amphipod) -> [Option<Amphipod>; 2] {
        let mut occupants = [None; 2];

        for i in 0..self.positions.len() {
            if let Room(r, space) = self.positions[i] {
                if r == room {
                    occupants[space as usize] = Some(Self::amphipod_at_position_index(i));
                }
            }
        }

        occupants
    }

    fn destination_space_within_room(&self, amphipod: Amphipod) -> Option<u32> {
        let occupants = self.room_occupants(amphipod);

        if occupants == [None, None] {
            Some(1)
        } else if occupants == [None, Some(amphipod)] {
            Some(0)
        } else {
            None
        }
    }

    fn with_move(&self, amphipod_index: usize, destination: Position) -> (Self, u32) {
        let mut positions = self.positions.clone();

        let amphipod = Self::amphipod_at_position_index(amphipod_index);
        let cost = amphipod.cost_to_move(positions[amphipod_index].distance_to(&destination));

        positions[amphipod_index] = destination;

        (Burrow { positions }, cost)
    }

    fn amphipod_at_position(&self, position: Position) -> Option<Amphipod> {
        self.positions.iter()
            .enumerate()
            .find(|(i, &p)| p == position)
            .map(|(i, _)| Self::amphipod_at_position_index(i))
    }

    fn is_settled(&self) -> bool {
        matches!(self.positions[0], Room(A, _))
            && matches!(self.positions[1], Room(A, _))
            && matches!(self.positions[2], Room(B, _))
            && matches!(self.positions[3], Room(B, _))
            && matches!(self.positions[4], Room(C, _))
            && matches!(self.positions[5], Room(C, _))
            && matches!(self.positions[6], Room(D, _))
            && matches!(self.positions[7], Room(D, _))
    }
}

impl Display for Burrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "█████████████")?;
        write!(f, "█")?;

        for h in 0..11 {
            write!(f, "{}", self.amphipod_at_position(Hallway(h))
                .map(|amphipod| amphipod.to_string())
                .unwrap_or(String::from(" ")))?;
        }

        writeln!(f, "█")?;
        write!(f, "██")?;

        for room in [A, B, C, D] {
            write!(f, "█")?;
            write!(f, "{}", self.amphipod_at_position(Room(room, 0))
                .map(|amphipod| amphipod.to_string())
                .unwrap_or(String::from(" ")))?;
        }

        writeln!(f, "███")?;
        write!(f, "  ")?;

        for room in [A, B, C, D] {
            write!(f, "█")?;
            write!(f, "{}", self.amphipod_at_position(Room(room, 1))
                .map(|amphipod| amphipod.to_string())
                .unwrap_or(String::from(" ")))?;
        }

        writeln!(f, "█")?;
        writeln!(f, "  █████████")?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_abs_diff() {
        assert_eq!(2, Position::abs_diff(5, 3));
        assert_eq!(2, Position::abs_diff(3, 5));
    }

    #[test]
    fn test_distance_to() {
        assert_eq!(4, Hallway(1).distance_to(&Hallway(5)));
        assert_eq!(4, Hallway(5).distance_to(&Hallway(1)));
        assert_eq!(5, Hallway(1).distance_to(&Room(B, 1)));
        assert_eq!(5, Room(B, 1).distance_to(&Hallway(1)));
        assert_eq!(1, Room(A, 0).distance_to(&Room(A, 1)));
        assert_eq!(5, Room(A, 0).distance_to(&Room(B, 1)));

        assert_eq!(0, Hallway(1).distance_to(&Hallway(1)));
        assert_eq!(0, Room(A, 0).distance_to(&Room(A, 0)));
    }

    #[test]
    fn test_min_cost_to_resolve() {
        assert_eq!(
            0,
            Burrow::new([A, A, B, B, C, C, D, D]).min_cost_to_resolve()
        );

        // #############
        // #...........#
        // ###D#C#C#A###
        //   #A#C#C#D#
        //   #########
        //
        // D moves 1 space out of the room, 6 spaces down the hall, and 1 space into the room for a
        // total of 8. A moves 1 out, 6 over, and 1 into the room (also 8).
        assert_eq!(
            8008,
            Burrow::new([D, A, B, B, C, C, A, D]).min_cost_to_resolve()
        );
    }

    #[test]
    fn test_room_occupants() {
        let mut burrow = Burrow::new([D, A, B, B, C, C, A, D]);

        assert_eq!([Some(D), Some(A)], burrow.room_occupants(A));

        burrow.positions[6] = Hallway(0);
        assert_eq!([None, Some(A)], burrow.room_occupants(A));

        burrow.positions[0] = Hallway(1);
        assert_eq!([None, None], burrow.room_occupants(A));
    }

    #[test]
    fn test_hallway_path_clear() {
        let mut burrow = Burrow::new([D, A, B, B, C, C, A, D]);

        assert!(burrow.hallway_path_clear(0, 10));

        burrow.positions[6] = Hallway(5);

        assert!(!burrow.hallway_path_clear(0, 10));
        assert!(!burrow.hallway_path_clear(10, 0));
        assert!(burrow.hallway_path_clear(0, 4));
        assert!(!burrow.hallway_path_clear(0, 5));
        assert!(burrow.hallway_path_clear(5, 0));
        assert!(burrow.hallway_path_clear(5, 10));
        assert!(!burrow.hallway_path_clear(10, 5));
    }

    #[test]
    fn test_destination_space_within_room() {
        let mut burrow = Burrow::new([D, A, B, C, C, B, A, D]);

        assert!(burrow.destination_space_within_room(A).is_none());

        // Move D from the A room into the hallway
        burrow.positions[6] = Hallway(0);

        assert_eq!(0, burrow.destination_space_within_room(A).unwrap());

        // Move A from the A room into the hallway
        burrow.positions[0] = Hallway(1);

        assert_eq!(1, burrow.destination_space_within_room(A).unwrap());

        // Move B from the B room into the hallway, leaving C in place
        burrow.positions[2] = Hallway(10);

        assert!(burrow.destination_space_within_room(B).is_none());
    }

    #[test]
    fn test_path_to_hallway_clear() {
        let mut burrow = Burrow::new([D, A, B, C, C, B, A, D]);

        assert!(burrow.path_to_hallway_clear(A, 0));
        assert!(!burrow.path_to_hallway_clear(A, 1));

        // Move D from the A room into the hallway
        burrow.positions[6] = Hallway(0);

        assert!(burrow.path_to_hallway_clear(A, 1));
    }

    #[test]
    fn test_with_move() {
        let burrow = Burrow::new([D, A, B, C, C, B, A, D]);

        let expected_burrow = Burrow {
            positions: [
                Room(A, 1),
                Room(D, 0),
                Room(B, 0),
                Room(C, 1),
                Room(B, 1),
                Room(C, 0),
                Hallway(0),
                Room(D, 1),
            ],
        };

        let expected_cost = 3000;

        assert_eq!((expected_burrow, expected_cost), burrow.with_move(6, Hallway(0)));
    }

    #[test]
    fn test_is_settled() {
        assert!(Burrow::new([A, A, B, B, C, C, D, D]).is_settled());
        assert!(!Burrow::new([D, A, B, C, C, B, A, D]).is_settled());
    }
}
