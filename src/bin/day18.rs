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

enum Token {
    OpenPair,
    ClosePair,
    Separator,
    Literal(u32),
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
}
