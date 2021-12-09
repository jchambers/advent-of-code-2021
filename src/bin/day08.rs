use std::{env, error, io};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg

 Digit | Segment count | Lit segments
 0     | 6             | A B C _ E F G
 1     | 2             | _ _ C _ _ F _
 2     | 5             | A _ C D E _ G
 3     | 5             | A _ C D _ F G
 4     | 4             | _ B C D _ F _
 5     | 5             | A B _ D _ F G
 6     | 6             | A B _ D E F G
 7     | 3             | A _ C _ _ F _
 8     | 7             | A B C D E F G
 9     | 6             | A B C D _ F G
 */

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let entries: Vec<NotesEntry> = io::BufReader::new(File::open(path)?).lines()
            .map(|line| line.unwrap())
            .map(|line| NotesEntry::from_str(line.as_str()).unwrap())
            .collect();

        println!("Unambiguous output digit count: {}", count_unambiguous_output_digits(&entries));

        Ok(())
    } else {
        Err("Usage: day08 INPUT_FILE_PATH".into())
    }
}

fn count_unambiguous_output_digits(entries: &[NotesEntry]) -> usize {
    entries.iter()
        .map(|entry| entry.unambiguous_output_digit_count())
        .sum()
}

type SegmentGroup = Vec<char>;

#[derive(Debug, Eq, PartialEq)]
struct NotesEntry {
    scrambled_digits: [SegmentGroup; 10],
    output_digits: [SegmentGroup; 4],
}

impl NotesEntry {
    pub fn unambiguous_output_digit_count(&self) -> usize {
        self.output_digits.iter()
            .map(|segment_group| SevenDigitDisplay::get_digit_candidates(segment_group).len())
            .filter(|&candidate_count| candidate_count == 1)
            .sum()
    }
}

impl FromStr for NotesEntry {
    type Err = Box<dyn std::error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = line.split(" | ").collect();

        if pieces.len() != 2 {
            return Err("Lines must have exactly two chunks separated by \" | \"".into());
        }

        let scrambled_digits: [SegmentGroup; 10] = pieces[0]
            .split_whitespace()
            .map(|segments| segments.chars().collect())
            .collect::<Vec<SegmentGroup>>()
            .try_into()
            .unwrap_or_else(|v: Vec<SegmentGroup>| panic!("Expected 10 scrambled digits, but got {}", v.len()));

        let output_digits: [SegmentGroup; 4] = pieces[1]
            .split_whitespace()
            .map(|segments| segments.chars().collect())
            .collect::<Vec<SegmentGroup>>()
            .try_into()
            .unwrap_or_else(|v: Vec<SegmentGroup>| panic!("Expected 4 scrambled digits, but got {}", v.len()));

        Ok(NotesEntry {
            scrambled_digits,
            output_digits,
        })
    }
}

struct SevenDigitDisplay {
    positions_by_char: HashMap<char, u8>,
}

impl SevenDigitDisplay {
    // Positions by number:
    //
    //  0000
    // 1    2
    // 1    2
    //  3333
    // 4    5
    // 4    5
    //  6666

    const INITIAL_CANDIDATES: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    pub fn new(scrambled_digits: &[SegmentGroup; 10]) {
        // Initially, any character could map to any position on the seven-segment display
        let mut candidates_by_position: [HashSet<char>; 7] = std::iter::repeat_with(|| SevenDigitDisplay::INITIAL_CANDIDATES.into_iter().collect())
            .take(7)
            .collect::<Vec<HashSet<char>>>()
            .try_into()
            .unwrap();

        for segment_group in scrambled_digits {
            // For a given segment group (a chunk of characters), we can infer which digits it might
            // possibly represent by the number of lit segments (e.g. if there are two segments lit,
            // the segment group can only represent the digit '1'; if there are 5 segments lit, it
            // could be any of '2', '3', or '5').
            let possible_digits = Self::get_digit_candidates(&segment_group);

            // From the possible digits, we can figure out which segments might be lit by those
            // digits.
            let possible_segments = possible_digits.iter()
                .fold(HashSet::new(), |mut segments, &digit| {
                    segments.extend(Self::get_lit_segments(digit));
                    segments
                });

            // Knowing which segments may be lit by a segment group tells us two important things:
            //
            // 1. The lit segments MAY correspond to one of the characters in the group
            // 2. The unlit segments MUST NOT correspond to any of the characters in the group
            //
            // An example: we get the segment group "cg". Because it's two characters long, we know
            // it may only correspond to the digit '1'. We know that either 'c' or 'g' must be in
            // positions 2 or 5 (rule 1). We also know that 'c' and 'g' cannot be in positions 0, 1,
            // 3, 4, or 6.
            println!("Processing segment group {:?}", segment_group);
            println!("Possible segments: {:?}", possible_segments);

            for position in 0..candidates_by_position.len() {
                if possible_segments.contains(&(position as u8)) {
                    // The segment may be lit, and any of the characters in this group might
                    // correspond to that segment.
                    // println!("\tPosition {} lit; retaining characters in segment group", position);
                    // candidates_by_position[position].retain(|candidate| segment_group.contains(candidate));
                } else {
                    // The segment is not lit, and cannot contain any of the characters in this
                    // group
                    println!("\tPosition {} not lit; removing characters in segment group", position);
                    candidates_by_position[position].retain(|candidate| !segment_group.contains(candidate));
                }

                println!("\tCandidates for position {}: {:?}", position, candidates_by_position[position]);
            }

            println!("----");
        }
    }

    pub fn get_digit_candidates(segments: &SegmentGroup) -> Vec<u8> {
        match segments.len() {
            2 => vec![1],
            3 => vec![7],
            4 => vec![4],
            5 => vec![2, 3, 5],
            6 => vec![0, 6, 9],
            7 => vec![8],
            _ => unreachable!()
        }
    }

    pub fn get_lit_segments(digit: u8) -> Vec<u8> {
        // See diagram above
        match digit {
            0 => vec![0, 1, 2, 4, 5, 6],
            1 => vec![2, 5],
            2 => vec![0, 2, 3, 4, 6],
            3 => vec![0, 2, 3, 5, 6],
            4 => vec![1, 2, 3, 5],
            5 => vec![0, 1, 3, 5, 6],
            6 => vec![0, 1, 3, 4, 5, 6],
            7 => vec![0, 2, 5],
            8 => vec![0, 1, 2, 3, 4, 5, 6],
            9 => vec![0, 1, 2, 3, 5, 6],
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_notes_entry_from_string() {
        let line = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

        let expected = NotesEntry {
            scrambled_digits: [
                vec!['b', 'e'],
                vec!['c', 'f', 'b', 'e', 'g', 'a', 'd'],
                vec!['c', 'b', 'd', 'g', 'e', 'f'],
                vec!['f', 'g', 'a', 'e', 'c', 'd'],
                vec!['c', 'g', 'e', 'b'],
                vec!['f', 'd', 'c', 'g', 'e'],
                vec!['a', 'g', 'e', 'b', 'f', 'd'],
                vec!['f', 'e', 'c', 'd', 'b'],
                vec!['f', 'a', 'b', 'c', 'd'],
                vec!['e', 'd', 'b'],
            ],

            output_digits: [
                vec!['f', 'd', 'g', 'a', 'c', 'b', 'e'],
                vec!['c', 'e', 'f', 'd', 'b'],
                vec!['c', 'e', 'f', 'b', 'g', 'd'],
                vec!['g', 'c', 'b', 'e'],
            ]
        };

        assert_eq!(expected, NotesEntry::from_str(line).unwrap());
    }

    #[test]
    fn test_unambiguous_output_digit_count() {
        let entry =
            NotesEntry::from_str("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")
                .unwrap();

        assert_eq!(2, entry.unambiguous_output_digit_count());
    }

    #[test]
    fn test_count_unambiguous_output_digits() {
        let entries: Vec<NotesEntry> = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ]
            .iter()
            .map(|&line| NotesEntry::from_str(line).unwrap())
            .collect();

        assert_eq!(26, count_unambiguous_output_digits(&entries));
    }

    #[test]
    fn test_i_have_no_idea_what_im_doing() {
        let entry =
            NotesEntry::from_str("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")
                .unwrap();

        SevenDigitDisplay::new(&entry.scrambled_digits);
    }
}
