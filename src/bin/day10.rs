use crate::NavigationSyntaxError::{Incomplete, Invalid};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let total_score = io::BufReader::new(File::open(path)?).lines()
            .map(|line| syntax_score(line.unwrap().as_str()))
            .sum::<u32>();

        println!("Total syntax score: {}", total_score);

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum NavigationSyntaxError {
    Incomplete { expected: char },
    Invalid { expected: char, found: char },
}

fn analyze_line(line: &str) -> Result<(), NavigationSyntaxError> {
    let mut stack = VecDeque::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push_front(c),
            ')' | ']' | '}' | '>' => {
                let expected_opener = expected_opener(&c);

                if stack.front() == Some(&expected_opener) {
                    stack.pop_front();
                } else {
                    return Err(Invalid {
                        expected: expected_closer(stack.front().unwrap()),
                        found: c,
                    });
                }
            }
            _ => {}
        };
    }

    if let Some(c) = stack.front() {
        Err(Incomplete {
            expected: expected_closer(c),
        })
    } else {
        Ok(())
    }
}

fn expected_closer(opener: &char) -> char {
    match opener {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn expected_opener(closer: &char) -> char {
    match closer {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn syntax_score(line: &str) -> u32 {
    match analyze_line(line) {
        Err(Invalid { expected: _, found: c }) => {
            match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!()
            }
        },
        _ => 0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::NavigationSyntaxError::Invalid;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn test_analyze_line() {
        assert_eq!(Ok(()), analyze_line("[]"));
        assert_eq!(Ok(()), analyze_line("([])"));
        assert_eq!(Ok(()), analyze_line("{()()()}"));
        assert_eq!(Ok(()), analyze_line("<([{}])>"));
        assert_eq!(Ok(()), analyze_line("[<>({}){}[([])<>]]"));
        assert_eq!(Ok(()), analyze_line("(((((((((())))))))))"));

        assert_eq!(
            Err(Invalid {
                expected: ')',
                found: ']'
            }),
            analyze_line("(]")
        );

        assert_eq!(
            Err(Invalid {
                expected: '}',
                found: '>'
            }),
            analyze_line("{()()()>")
        );

        assert_eq!(
            Err(Invalid {
                expected: ')',
                found: '}'
            }),
            analyze_line("(((()))}")
        );

        assert_eq!(
            Err(Invalid {
                expected: '>',
                found: ')'
            }),
            analyze_line("<([]){()}[{}])")
        );
    }

    #[test]
    fn test_syntax_score() {
        assert_eq!(26397, TEST_INPUT.lines()
            .map(|line| syntax_score(line))
            .sum::<u32>());
    }
}
