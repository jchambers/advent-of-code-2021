use std::{env, error, io};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use crate::Token::{ClosePair, Literal, OpenPair, Separator};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _lines = io::BufReader::new(File::open(path)?).lines();

        Ok(())
    } else {
        Err("Usage: day18 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Element {
    Pair(Box<SnailfishNumber>),
    Literal(u32),
}

#[derive(Debug, Eq, PartialEq)]
struct SnailfishNumber {
    left: Element,
    right: Element,
}

#[derive(Debug)]
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

                    Some(ExplodeState {
                        zeroed_element: true,
                        left: explode_state.left,
                        right: None,
                    })
                } else {
                    None
                }
            } else if let Element::Pair(pair) = &mut self.right {
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


                    Some(ExplodeState {
                        zeroed_element: true,
                        left: None,
                        right: explode_state.right,
                    })
                } else {
                    None
                }
            } else {
                None
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

            Some(ExplodeState {
                zeroed_element: false,
                left: Some(left),
                right: Some(right),
            })
        }
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
    }
}
