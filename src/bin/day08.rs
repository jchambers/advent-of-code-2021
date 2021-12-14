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

        let decoded_sum: u32 = entries.iter()
            .map(|entry| SevenDigitDisplay::new(&entry.scrambled_digits).decode(&entry.output_digits))
            .sum();

        println!("Sum of decoded numbers: {}", decoded_sum);

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
    positions_by_character: HashMap<char, u8>,
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

    pub fn new(scrambled_digits: &[SegmentGroup; 10]) -> Self {
        let mut positions_by_character: HashMap<char, u8> = HashMap::new();

        // We can use the frequency with which certain characters appear to unambiguously map some,
        // but not all, of them to positions.
        {
            let mut frequencies = HashMap::new();

            scrambled_digits.iter()
                .for_each(|segment_group| {
                    segment_group.iter().for_each(|character| {
                        let count = frequencies.entry(character).or_insert(0);
                        *count += 1;
                    })
                });

            for (&character, frequency) in frequencies {
                match frequency {
                    4 => positions_by_character.insert(character, 4),
                    6 => positions_by_character.insert(character, 1),
                    9 => positions_by_character.insert(character, 5),
                    _ => None
                };
            }
        }

        // Three down, four to go! We can get position 0 by figuring out what's lit in '7', but not
        // '1'. We can also figure out which of the segments in the '1' isn't already in the map.
        {
            let one_chars: HashSet<char> = scrambled_digits.iter()
                .find(|segment_group| segment_group.len() == 2)
                .unwrap()
                .clone()
                .into_iter()
                .collect();

            let seven_chars: HashSet<char> = scrambled_digits.iter()
                .find(|segment_group| segment_group.len() == 3)
                .unwrap()
                .clone()
                .into_iter()
                .collect();

            let top_segment_char = seven_chars.difference(&one_chars).next().unwrap();

            positions_by_character.insert(*top_segment_char, 0);

            // Only one of the characters in the '1' will be undefined, and that maps to position 2
            one_chars.iter().for_each(|character| {
                positions_by_character.entry(*character).or_insert(2);
            });
        }

        // Five down, two to go! We can find position three because it will only appear twice among
        // the unsolved characters in the 6-segment (0/6/9) cohort.
        {
            let mut frequencies = HashMap::new();

            scrambled_digits.iter()
                .filter(|segment_group| segment_group.len() == 6)
                .for_each(|segment_group| {
                    segment_group.iter()
                        .filter(|character| !positions_by_character.contains_key(character))
                        .for_each(|character| {
                            let count = frequencies.entry(character).or_insert(0);
                            *count += 1;
                        });
                });

            let middle_character = frequencies.iter()
                .filter_map(|(character, &count)| {
                    if count == 2 {
                        Some(character)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap();

            positions_by_character.insert(**middle_character, 3);
        }

        // Six down! Whatever is left is in position 6.
        for character in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            positions_by_character.entry(character).or_insert(6);
        }

        SevenDigitDisplay { positions_by_character }
    }

    pub fn decode(&self, segment_groups: &[SegmentGroup]) -> u32 {
        let mut decoded = 0;

        for segment_group in segment_groups {
            decoded *= 10;
            decoded += self.decode_digit(segment_group) as u32;
        }

        decoded
    }

    fn decode_digit(&self, segment: &SegmentGroup) -> u8 {
        let mut lit_positions: Vec<u8> = segment.iter()
            .map(|character| *self.positions_by_character.get(character).unwrap())
            .collect();

        lit_positions.sort_unstable();

        for digit in 0..=9 {
            if lit_positions == Self::get_lit_segments(digit) {
                return digit;
            }
        }

        unreachable!()
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
    fn test_decode_digit() {
        let entry =
            NotesEntry::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
                .unwrap();

        let display = SevenDigitDisplay::new(&entry.scrambled_digits);

        assert_eq!(8, display.decode_digit(&("acedgfb".chars().collect())));
        assert_eq!(5, display.decode_digit(&("cdfbe".chars().collect())));
        assert_eq!(2, display.decode_digit(&("gcdfa".chars().collect())));
        assert_eq!(3, display.decode_digit(&("fbcad".chars().collect())));
        assert_eq!(7, display.decode_digit(&("dab".chars().collect())));
        assert_eq!(9, display.decode_digit(&("cefabd".chars().collect())));
        assert_eq!(6, display.decode_digit(&("cdfgeb".chars().collect())));
        assert_eq!(4, display.decode_digit(&("eafb".chars().collect())));
        assert_eq!(0, display.decode_digit(&("cagedb".chars().collect())));
        assert_eq!(1, display.decode_digit(&("ab".chars().collect())));
    }

    #[test]
    fn test_decode() {
        let entry =
            NotesEntry::from_str("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
                .unwrap();

        let display = SevenDigitDisplay::new(&entry.scrambled_digits);

        assert_eq!(5353, display.decode(&entry.output_digits));
    }
}
