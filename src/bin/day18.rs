use std::{env, error, io};
use std::cmp::max;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;
use crate::Token::{ClosePair, Literal, OpenPair, Separator};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let magnitude_of_sum = io::BufReader::new(File::open(path)?).lines()
            .filter_map(|line| line.ok())
            .map(|line| SnailfishNumber::from_str(line.as_str()).unwrap())
            .sum::<SnailfishNumber>()
            .magnitude();

        println!("Magnitude of sum: {}", magnitude_of_sum);

        let numbers: Vec<SnailfishNumber> = io::BufReader::new(File::open(path)?).lines()
            .filter_map(|line| line.ok())
            .map(|line| SnailfishNumber::from_str(line.as_str()).unwrap())
            .collect();

        println!("Largest possible magnitude of any sum: {}", largest_pair_magnitude(&numbers));

        Ok(())
    } else {
        Err("Usage: day18 INPUT_FILE_PATH".into())
    }
}

fn largest_pair_magnitude(numbers: &[SnailfishNumber]) -> u32 {
    let mut max_magnitude = 0;

    for i in 0..numbers.len() - 1 {
        for j in i..numbers.len() {
            max_magnitude = max(max_magnitude, (numbers[i].clone() + numbers[j].clone()).magnitude());
            max_magnitude = max(max_magnitude, (numbers[j].clone() + numbers[i].clone()).magnitude());
        }
    }

    max_magnitude
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Element {
    Pair(Box<SnailfishNumber>),
    Literal(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SnailfishNumber {
    left: Element,
    right: Element,
}

struct ExplodeState {
    zeroed_element: bool,
    left: Option<u32>,
    right: Option<u32>,
}

impl SnailfishNumber {
    fn from_tokens(tokens: &mut VecDeque<Token>) -> Result<Self, Box<dyn error::Error>> {
        if let Some(OpenPair) = tokens.pop_front() {
            let next_token = tokens.pop_front();

            let left = if let Some(Literal(value)) = next_token {
                Element::Literal(value)
            } else if let Some(OpenPair) = next_token {
                tokens.push_front(OpenPair);
                Element::Pair(Box::new(SnailfishNumber::from_tokens(tokens)?))
            } else {
                return Err("Unexpected token (expected Literal or OpenPair)".into());
            };

            if let Some(Separator) = tokens.pop_front() {
            } else {
                return Err("Unexpected token (expected Separator)".into());
            }

            let next_token = tokens.pop_front();

            let right = if let Some(Literal(value)) = next_token {
                Element::Literal(value)
            } else if let Some(OpenPair) = next_token {
                tokens.push_front(OpenPair);
                Element::Pair(Box::new(SnailfishNumber::from_tokens(tokens)?))
            } else {
                return Err("Unexpected token (expected Literal or OpenPair)".into());
            };

            if let Some(ClosePair) = tokens.pop_front() {
            } else {
                return Err("Unexpected token (expected ClosePair)".into());
            }

            Ok(SnailfishNumber { left, right })
        } else {
            Err("Unexpected token (expected OpenPair)".into())
        }
    }

    pub fn magnitude(&self) -> u32 {
        let left_magnitude = 3 * match &self.left {
            Element::Literal(value) => *value,
            Element::Pair(pair) => pair.magnitude(),
        };

        let right_magnitude = 2 * match &self.right {
            Element::Literal(value) => *value,
            Element::Pair(pair) => pair.magnitude(),
        };

        left_magnitude + right_magnitude
    }

    fn reduce(&mut self) {
        loop {
            if let Some(_) = self.try_explode(0) {
                continue;
            } else {
                if self.try_split() {
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    fn try_explode(&mut self, depth: usize) -> Option<ExplodeState> {
        if depth < 4 {
            if let Element::Pair(pair) = &mut self.left {
                if let Some(explode_state) = pair.try_explode(depth + 1) {
                    if !explode_state.zeroed_element {
                        self.left = Element::Literal(0);
                    }

                    if let Some(right_value) = explode_state.right {
                        match &mut self.right {
                            Element::Literal(old) => self.right = Element::Literal(*old + right_value),
                            Element::Pair(pair) => pair.add_to_leftmost_literal(right_value),
                        }
                    }

                    return Some(ExplodeState {
                        zeroed_element: true,
                        left: explode_state.left,
                        right: None,
                    })
                }
            }

            if let Element::Pair(pair) = &mut self.right {
                if let Some(explode_state) = pair.try_explode(depth + 1) {
                    if !explode_state.zeroed_element {
                        self.right = Element::Literal(0);
                    }

                    if let Some(left_value) = explode_state.left {
                        match &mut self.left {
                            Element::Literal(old) => self.left = Element::Literal(*old + left_value),
                            Element::Pair(pair) => pair.add_to_rightmost_literal(left_value),
                        }
                    }

                    return Some(ExplodeState {
                        zeroed_element: true,
                        left: None,
                        right: explode_state.right,
                    })
                }
            }
        } else {
            // The problem statement claims that exploding elements will always have two literals as
            // elements
            let left = match self.left {
                Element::Literal(value) => value,
                _ => unreachable!(),
            };

            let right = match self.right {
                Element::Literal(value) => value,
                _ => unreachable!(),
            };

            return Some(ExplodeState {
                zeroed_element: false,
                left: Some(left),
                right: Some(right),
            })
        }

        None
    }

    fn add_to_leftmost_literal(&mut self, value: u32) {
        match &mut self.left {
            Element::Literal(old) => self.left = Element::Literal(*old + value),
            Element::Pair(pair) => pair.add_to_leftmost_literal(value),
        };
    }

    fn add_to_rightmost_literal(&mut self, value: u32) {
        match &mut self.right {
            Element::Literal(old) => self.right = Element::Literal(*old + value),
            Element::Pair(pair) => pair.add_to_rightmost_literal(value),
        };
    }

    fn try_split(&mut self) -> bool {
        let split_left = match &mut self.left {
            Element::Literal(value) => if *value >= 10 {
                self.left = Element::Pair(Box::new(SnailfishNumber {
                    left: Element::Literal(*value / 2),
                    right: Element::Literal((*value / 2) + (*value % 2)),
                }));

                true
            } else {
                false
            },
            Element::Pair(pair) => pair.try_split()
        };

        if split_left {
            return true;
        }

        match &mut self.right {
            Element::Literal(value) => if *value >= 10 {
                self.right = Element::Pair(Box::new(SnailfishNumber {
                    left: Element::Literal(*value / 2),
                    right: Element::Literal((*value / 2) + (*value % 2)),
                }));

                true
            } else {
                false
            },
            Element::Pair(pair) => pair.try_split()
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = SnailfishNumber {
            left: Element::Pair(Box::new(self)),
            right: Element::Pair(Box::new(rhs)),
        };

        sum.reduce();

        sum
    }
}

impl Sum for SnailfishNumber {
    fn sum<I: Iterator<Item=Self>>(mut iter: I) -> Self {
        let mut sum = iter.next().unwrap();

        while let Some(next) = iter.next() {
            sum = sum + next;
        }

        sum
    }
}

impl Display for SnailfishNumber {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        match &self.left {
            Element::Literal(value) => write!(f, "{},", *value)?,
            Element::Pair(pair) => write!(f, "{},", pair)?,
        }

        match &self.right {
            Element::Literal(value) => write!(f, "{}]", *value)?,
            Element::Pair(pair) => write!(f, "{}]", pair)?,
        }

        Ok(())
    }
}

enum Token {
    OpenPair,
    ClosePair,
    Separator,
    Literal(u32),
}

impl FromStr for SnailfishNumber {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut tokens = VecDeque::new();
        let mut chars = string.chars().peekable();

        while let Some(c) = chars.next() {
            tokens.push_back(match c {
                '[' => OpenPair,
                ']' => ClosePair,
                ',' => Separator,
                '0'..='9' => {
                    let mut value = c as u32 - b'0' as u32;

                    while matches!(chars.peek(), Some('0'..='9')) {
                        value *= 10;
                        value += chars.next().unwrap() as u32 - b'0' as u32;
                    }

                    Literal(value)
                }
                _ => return Err(format!("Unexpected character: '{}'", c).into())
            });
        }

        Ok(SnailfishNumber::from_tokens(&mut tokens)?)
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_snailfish_number_from_string() {
        let expected = SnailfishNumber {
            left: Element::Pair(Box::new(SnailfishNumber {
                left: Element::Pair(Box::new(SnailfishNumber {
                    left: Element::Pair(Box::new(SnailfishNumber {
                        left: Element::Pair(Box::new(SnailfishNumber {
                            left: Element::Literal(9),
                            right: Element::Literal(8),
                        })),
                        right: Element::Literal(1),
                    })),
                    right: Element::Literal(2),
                })),
                right: Element::Literal(3),
            })),
            right: Element::Literal(4),
        };

        assert_eq!(expected, SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]").unwrap());
    }

    #[test]
    fn test_explode() {
        {
            let expected = SnailfishNumber::from_str("[[[[0,9],2],3],4]").unwrap();

            let mut exploded = SnailfishNumber::from_str("[[[[[9,8],1],2],3],4]").unwrap();
            assert!(exploded.try_explode(0).is_some());

            assert_eq!(expected, exploded);
        }

        {
            let expected = SnailfishNumber::from_str("[7,[6,[5,[7,0]]]]").unwrap();

            let mut exploded = SnailfishNumber::from_str("[7,[6,[5,[4,[3,2]]]]]").unwrap();
            assert!(exploded.try_explode(0).is_some());

            assert_eq!(expected, exploded);
        }

        {
            let expected = SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]").unwrap();

            let mut exploded = SnailfishNumber::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
            assert!(exploded.try_explode(0).is_some());

            assert_eq!(expected, exploded);
        }

        {
            let expected = SnailfishNumber::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();

            let mut exploded = SnailfishNumber::from_str("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]").unwrap();
            assert!(exploded.try_explode(0).is_some());

            assert_eq!(expected, exploded);
        }
    }

    #[test]
    fn test_split() {
        let expected = SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]").unwrap();

        let mut split = SnailfishNumber::from_str("[[[[0,7],4],[15,[0,13]]],[1,1]]").unwrap();
        assert!(split.try_split());

        assert_eq!(expected, split);
    }

    #[test]
    fn test_reduce() {
        let expected = SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();

        let mut reduced = SnailfishNumber::from_str("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").unwrap();
        reduced.reduce();

        assert_eq!(expected, reduced);
    }

    #[test]
    fn test_add() {
        {
            let lhs = SnailfishNumber::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
            let rhs = SnailfishNumber::from_str("[1,1]").unwrap();

            let expected_sum = SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();

            assert_eq!(expected_sum, lhs + rhs);
        }

        {
            let expected = SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();

            let numbers: &str = indoc! {"
                [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]
            "};

            let sum: SnailfishNumber = numbers.lines()
                .map(|line| SnailfishNumber::from_str(line).unwrap())
                .sum();

            assert_eq!(expected, sum);
        }
    }

    #[test]
    fn test_magnitude() {
        let test_cases = [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ];

        for (string, expected) in test_cases {
            assert_eq!(expected, SnailfishNumber::from_str(string).unwrap().magnitude());
        }
    }

    #[test]
    fn test_largest_pair_magnitude() {
        let numbers_string: &str = indoc! {"
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "};

        let numbers: Vec<SnailfishNumber> = numbers_string.lines()
            .map(|line| SnailfishNumber::from_str(line).unwrap())
            .collect();

        assert_eq!(3993, largest_pair_magnitude(&numbers));
    }
}
