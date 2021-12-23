use self::Amphipod::*;
use self::Position::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let burrow_folded = Burrow::<2>::from_str(std::fs::read_to_string(args[1].as_str())?.as_str())?;

        println!(
            "Min cost to settle positions (folded map): {}",
            burrow_folded.min_cost_to_resolve().unwrap()
        );

        let burrow_unfolded = Burrow::<4>::from_str(std::fs::read_to_string(args[2].as_str())?.as_str())?;

        println!(
            "Min cost to settle positions (unfolded map): {}",
            burrow_unfolded.min_cost_to_resolve().unwrap()
        );

        Ok(())
    } else {
        Err("Usage: day23 INPUT_FILE_1_PATH INPUT_FILE_2_PATH".into())
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

impl FromStr for Amphipod {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "A" => Ok(A),
            "B" => Ok(B),
            "C" => Ok(C),
            "D" => Ok(D),
            _ => Err(format!("Bad amphipod identifier: {}", string).into()),
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Burrow<const N: usize> {
    // Each amphipod has a permanent slot in this array. The first two are A1 and A2, the next are
    // B1 and B2, and so on.
    positions: [[Position; N]; 4],
}

impl<const N: usize> Burrow<N> {
    // Amphipods are given in order of their starting positions from left to right and top to
    // bottom. Example:
    //
    // #############
    // #...........#
    // ###B#C#B#D###  -> [B, A], [C, D], [B, C], [D, A]
    //   #A#D#C#A#
    //   #########
    pub fn new(initial_positions: [[Amphipod; N]; 4]) -> Self {
        let mut positions_by_amphipod_type = HashMap::new();

        for room in 0..initial_positions.len() {
            let room_type = match room {
                0 => A,
                1 => B,
                2 => C,
                3 => D,
                _ => unreachable!(),
            };

            for space in 0..initial_positions[room].len() {
                positions_by_amphipod_type
                    .entry(initial_positions[room][space])
                    .or_insert(Vec::new())
                    .push(Room(room_type, space as u32))
            }
        }

        // Hack; pre-populate the position array with nonsense locations while we let things settle
        let mut positions = [[Hallway(0); N]; 4];

        for amphipod_type in [A, B, C, D] {
            let room_index = Self::room_index_for_amphipod(amphipod_type);

            positions[room_index] = positions_by_amphipod_type
                .get(&amphipod_type)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap();
        }

        for room in 0..positions.len() {
            assert!(positions[room]
                .iter()
                .all(|position| matches!(position, Room(_, _))));
        }

        Self { positions }
    }

    pub fn min_cost_to_resolve(&self) -> Option<u32> {
        let mut visited_states = HashSet::new();
        let mut tentative_costs = BinaryHeap::new();

        tentative_costs.push(StateAndCost {
            cost: 0,
            state: *self,
        });

        while let Some(state_and_cost) = tentative_costs.pop() {
            if visited_states.contains(&(state_and_cost.state)) {
                continue;
            }

            if state_and_cost.state.is_settled() {
                return Some(state_and_cost.cost);
            }

            state_and_cost
                .state
                .next_possible_states()
                .iter()
                .filter(|(next_state, _)| !visited_states.contains(next_state))
                .for_each(|&(next_state, cost)| {
                    tentative_costs.push(StateAndCost {
                        state: next_state,
                        cost: cost + state_and_cost.cost,
                    })
                });

            visited_states.insert(state_and_cost.state);
        }

        None
    }

    fn room_index_for_amphipod(amphipod: Amphipod) -> usize {
        match amphipod {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }

    fn next_possible_states(&self) -> Vec<(Self, u32)> {
        const LEGAL_HALLWAY_STOPS: [u32; 7] = [0, 1, 3, 5, 7, 9, 10];

        let mut next_possible_states = Vec::new();

        for (amphipod, position) in self.positions() {
            match position {
                Hallway(h) => {
                    // If we're in the hallway, the only legal move is into our target room if
                    // it's empty or if it has one occupant of the correct type.
                    let destination_room = Position::room_position_in_hallway(amphipod);

                    if self.hallway_path_clear(h, destination_room) {
                        if let Some(space) = self.destination_space_within_room(amphipod) {
                            next_possible_states
                                .push(self.with_move(position, Room(amphipod, space)));
                        }
                    }
                }
                Room(room, space) => {
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
                        let occupants = self.room_occupants(room);

                        occupants[space as usize..].iter()
                            .any(|&maybe_occupant| if let Some(occupant) = maybe_occupant {
                                occupant != room
                            } else {
                                false
                            })
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
                                next_possible_states.push(self.with_move(
                                    position,
                                    Room(amphipod, destination_space.unwrap()),
                                ));
                            } else {
                                // Looks like we're moving to the hallway instead
                                for dest_hallway_position in LEGAL_HALLWAY_STOPS {
                                    if self.hallway_path_clear(
                                        start_room_position,
                                        dest_hallway_position,
                                    ) {
                                        next_possible_states.push(
                                            self.with_move(
                                                position,
                                                Hallway(dest_hallway_position),
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        next_possible_states
    }

    fn hallway_path_clear(&self, start: u32, destination: u32) -> bool {
        let mut hallway = [true; 11];

        for (_, position) in self.positions() {
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
        self.room_occupants(room)[0..space as usize]
            .iter()
            .all(Option::is_none)
    }

    fn room_occupants(&self, room: Amphipod) -> [Option<Amphipod>; N] {
        let mut occupants = [None; N];

        for (amphipod, position) in self.positions() {
            if let Room(r, space) = position {
                if r == room {
                    occupants[space as usize] = Some(amphipod);
                }
            }
        }

        occupants
    }

    fn destination_space_within_room(&self, amphipod: Amphipod) -> Option<u32> {
        let occupants = self.room_occupants(amphipod);

        for space in (0..N).rev() {
            match occupants[space] {
                None => return Some(space as u32),
                Some(occupant) => {
                    if occupant != amphipod {
                        return None;
                    }
                }
            }
        }

        None
    }

    fn with_move(&self, start: Position, destination: Position) -> (Self, u32) {
        for i in 0..self.positions.len() {
            for j in 0..N {
                if self.positions[i][j] == start {
                    let amphipod = match i {
                        0 => A,
                        1 => B,
                        2 => C,
                        3 => D,
                        _ => unreachable!(),
                    };

                    let mut with_move = self.clone();
                    with_move.positions[i][j] = destination;

                    let cost = amphipod.cost_to_move(start.distance_to(&destination));

                    return (with_move, cost);
                }
            }
        }

        unreachable!();
    }

    fn positions(&self) -> Vec<(Amphipod, Position)> {
        let mut positions = Vec::with_capacity(self.positions.len() * N);

        for amphipod_type in [A, B, C, D] {
            let room = Self::room_index_for_amphipod(amphipod_type);

            for subscript in 0..N {
                positions.push((amphipod_type, self.positions[room][subscript]))
            }
        }

        positions
    }

    fn amphipod_at_position(&self, position: Position) -> Option<Amphipod> {
        /* self.positions
        .iter()
        .enumerate()
        .find(|(_, &p)| p == position)
        .map(|(i, _)| Self::amphipod_at_position_index(i)) */

        todo!()
    }

    fn is_settled(&self) -> bool {
        self.positions().iter().all(|(amphipod, position)| {
            if let Room(room, _) = position {
                amphipod == room
            } else {
                false
            }
        })
    }
}

impl<const N: usize> FromStr for Burrow<N> {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lines = string.lines().skip(2).take(N);
        let mut initial_positions = [[A; N]; 4];

        lines.enumerate().for_each(|(space, line)| {
            line.chars()
                .filter_map(|c| Amphipod::from_str(c.to_string().as_str()).ok())
                .enumerate()
                .for_each(|(room, amphipod)| initial_positions[room][space] = amphipod);
        });

        Ok(Burrow::new(initial_positions))
    }
}

impl<const N: usize> Display for Burrow<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "█████████████")?;
        write!(f, "█")?;

        for h in 0..11 {
            write!(
                f,
                "{}",
                self.amphipod_at_position(Hallway(h))
                    .map(|amphipod| amphipod.to_string())
                    .unwrap_or(String::from(" "))
            )?;
        }

        writeln!(f, "█")?;
        write!(f, "██")?;

        for room in [A, B, C, D] {
            write!(f, "█")?;
            write!(
                f,
                "{}",
                self.amphipod_at_position(Room(room, 0))
                    .map(|amphipod| amphipod.to_string())
                    .unwrap_or(String::from(" "))
            )?;
        }

        writeln!(f, "███")?;
        write!(f, "  ")?;

        for room in [A, B, C, D] {
            write!(f, "█")?;
            write!(
                f,
                "{}",
                self.amphipod_at_position(Room(room, 1))
                    .map(|amphipod| amphipod.to_string())
                    .unwrap_or(String::from(" "))
            )?;
        }

        writeln!(f, "█")?;
        writeln!(f, "  █████████")?;

        Ok(())
    }
}

#[derive(Eq, PartialEq)]
struct StateAndCost<const N: usize> {
    state: Burrow<N>,
    cost: u32,
}

impl<const N: usize> Ord for StateAndCost<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Swap the "normal" order so we have a min-first heap
        other.cost.cmp(&self.cost)
    }
}

impl<const N: usize> PartialOrd<Self> for StateAndCost<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

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
    fn test_room_occupants() {
        let mut burrow = Burrow::new([[D, A], [B, B], [C, C], [A, D]]);

        assert_eq!([Some(D), Some(A)], burrow.room_occupants(A));

        // Move D into the hallway
        burrow.positions[3][0] = Hallway(0);
        assert_eq!([None, Some(A)], burrow.room_occupants(A));

        // Move A into the hallway
        burrow.positions[0][0] = Hallway(1);
        assert_eq!([None, None], burrow.room_occupants(A));
    }

    #[test]
    fn test_hallway_path_clear() {
        let mut burrow = Burrow::new([[D, A], [B, B], [C, C], [A, D]]);

        assert!(burrow.hallway_path_clear(0, 10));

        burrow.positions[3][0] = Hallway(5);

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
        let mut burrow = Burrow::new([[D, A], [B, C], [C, B], [A, D]]);

        assert!(burrow.destination_space_within_room(A).is_none());

        // Move D from the A room into the hallway
        burrow.positions[3][0] = Hallway(0);

        assert_eq!(0, burrow.destination_space_within_room(A).unwrap());

        // Move A from the A room into the hallway
        burrow.positions[0][0] = Hallway(1);

        assert_eq!(1, burrow.destination_space_within_room(A).unwrap());

        // Move B from the B room into the hallway, leaving C in place
        burrow.positions[1][0] = Hallway(10);

        assert!(burrow.destination_space_within_room(B).is_none());
    }

    #[test]
    fn test_path_to_hallway_clear() {
        let mut burrow = Burrow::new([[D, A], [B, C], [C, B], [A, D]]);

        assert!(burrow.path_to_hallway_clear(A, 0));
        assert!(!burrow.path_to_hallway_clear(A, 1));

        // Move D from the A room into the hallway
        burrow.positions[3][0] = Hallway(0);

        assert!(burrow.path_to_hallway_clear(A, 1));
    }

    #[test]
    fn test_with_move() {
        let burrow = Burrow::new([[D, A], [B, C], [C, B], [A, D]]);

        let expected_burrow = Burrow {
            positions: [
                [Room(A, 1), Room(D, 0)],
                [Room(B, 0), Room(C, 1)],
                [Room(B, 1), Room(C, 0)],
                [Hallway(0), Room(D, 1)],
            ],
        };

        let expected_cost = 3000;

        assert_eq!(
            (expected_burrow, expected_cost),
            burrow.with_move(Room(A, 0), Hallway(0))
        );
    }

    #[test]
    fn test_next_possible_states() {
        assert!(Burrow::new([[A, A], [B, B], [C, C], [D, D]])
            .next_possible_states()
            .is_empty());

        assert!(!Burrow::new([[D, A], [B, C], [C, B], [A, D]])
            .next_possible_states()
            .is_empty());
    }

    #[test]
    fn test_is_settled() {
        assert!(Burrow::new([[A, A], [B, B], [C, C], [D, D]]).is_settled());
        assert!(!Burrow::new([[D, A], [B, C], [C, B], [A, D]]).is_settled());
    }

    #[test]
    fn test_min_cost_to_resolve() {
        assert_eq!(
            Some(12521),
            Burrow::new([[B, A], [C, D], [B, C], [D, A]]).min_cost_to_resolve()
        );

        assert_eq!(
            Some(44169),
            Burrow::new([[B, D, D, A], [C, C, B, D], [B, B, A, C], [D, A, C, A]]).min_cost_to_resolve()
        );
    }

    #[test]
    fn test_burrow_from_string() {
        assert_eq!(
            Burrow::new([[B, A], [C, D], [B, C], [D, A]]),
            Burrow::from_str(indoc! {"
                #############
                #...........#
                ###B#C#B#D###
                  #A#D#C#A#
                  #########
            "})
            .unwrap()
        );

        assert_eq!(
            Burrow::new([[B, D, D, A], [C, C, B, D], [B, B, A, C], [D, A, C, A]]),
            Burrow::from_str(indoc! {"
                #############
                #...........#
                ###B#C#B#D###
                  #D#C#B#A#
                  #D#B#A#C#
                  #A#D#C#A#
                  #########
            "})
            .unwrap()
        );
    }
}
