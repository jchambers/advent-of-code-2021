use crate::NavigationSyntaxError::{Incomplete, Invalid};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        println!(
            "Total syntax score: {}",
            io::BufReader::new(File::open(path)?)
                .lines()
                .map(|line| syntax_score(line.unwrap().as_str()))
                .sum::<u32>()
        );

        println!(
            "Median autocomplete score: {}",
            median_autocomplete_score(
                io::BufReader::new(File::open(path)?)
                    .lines()
                    .map(|line| line.unwrap())
            )
        );

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum NavigationSyntaxError {
    Incomplete(String),
    Invalid(char),
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
                    return Err(Invalid(c));
                }
            }
            _ => {}
        };
    }

    if let Some(_) = stack.front() {
        let mut expected = String::new();

        while let Some(opener) = stack.pop_front() {
            expected.push(expected_closer(&opener));
        }

        Err(Incomplete(expected))
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
    if let Err(Invalid(c)) = analyze_line(line) {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    } else {
        0
    }
}

fn autocomplete_score(line: &str) -> u64 {
    if let Err(Incomplete(completion)) = analyze_line(line) {
        completion
            .chars()
            .map(|c| match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            })
            .fold(0, |total, char_score| (total * 5) + char_score)
    } else {
        0
    }
}

fn median_autocomplete_score(lines: impl Iterator<Item = String>) -> u64 {
    let mut scores: Vec<u64> = lines
        .map(|line| autocomplete_score(line.as_str()))
        .filter(|&score| score > 0)
        .collect();

    scores.sort();

    scores[scores.len() / 2]
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

        assert_eq!(Err(Invalid(']')), analyze_line("(]"));
        assert_eq!(Err(Invalid('>')), analyze_line("{()()()>"));
        assert_eq!(Err(Invalid('}')), analyze_line("(((()))}"));
        assert_eq!(Err(Invalid(')')), analyze_line("<([]){()}[{}])"));

        assert_eq!(
            Err(Incomplete(String::from("}}]])})]"))),
            analyze_line("[({(<(())[]>[[{[]{<()<>>")
        );

        assert_eq!(
            Err(Incomplete(String::from(")}>]})"))),
            analyze_line("[(()[<>])]({[<{<<[]>>(")
        );

        assert_eq!(
            Err(Incomplete(String::from("}}>}>))))"))),
            analyze_line("(((({<>}<{<{<>}{[]{[]{}")
        );

        assert_eq!(
            Err(Incomplete(String::from("]]}}]}]}>"))),
            analyze_line("{<[[]]>}<{[{[{[]{()[[[]")
        );

        assert_eq!(
            Err(Incomplete(String::from("])}>"))),
            analyze_line("<{([{{}}[<[[[<>{}]]]>[]]")
        );
    }

    #[test]
    fn test_syntax_score() {
        assert_eq!(
            26397,
            TEST_INPUT
                .lines()
                .map(|line| syntax_score(line))
                .sum::<u32>()
        );
    }

    #[test]
    fn test_autocomplete_score() {
        assert_eq!(288957, autocomplete_score("[({(<(())[]>[[{[]{<()<>>"));
        assert_eq!(5566, autocomplete_score("[(()[<>])]({[<{<<[]>>("));
        assert_eq!(1480781, autocomplete_score("(((({<>}<{<{<>}{[]{[]{}"));
        assert_eq!(995444, autocomplete_score("{<[[]]>}<{[{[{[]{()[[[]"));
        assert_eq!(294, autocomplete_score("<{([{{}}[<[[[<>{}]]]>[]]"));
    }

    #[test]
    fn test_median_autocomplete_score() {
        assert_eq!(
            288957,
            median_autocomplete_score(TEST_INPUT.lines().map(|line| String::from(line)))
        );
    }
}
