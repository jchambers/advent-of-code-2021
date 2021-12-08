use std::{env, error, io};
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
}

impl SevenDigitDisplay {
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
}
