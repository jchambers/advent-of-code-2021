use std::collections::HashMap;
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut polymerizer = Polymerizer::from_str(std::fs::read_to_string(path)?.as_str()).unwrap();

        for _ in 0..10 {
            polymerizer = polymerizer.advance();
        }

        println!("Element spread after 10 steps: {}", polymerizer.element_spread());

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Polymerizer {
    template: String,
    rules: HashMap<String, String>,
}

impl Polymerizer {
    pub fn advance(self) -> Self {
        let mut template = String::new();

        for i in 0..self.template.len() - 1 {
            template.push_str(&self.template[i..i + 1]);
            template.push_str(self.rules.get(&self.template[i..=i + 1]).unwrap());
        }

        template.push_str(&self.template[self.template.len() - 1..]);

        Polymerizer {
            template,
            rules: self.rules
        }
    }

    pub fn element_spread(&self) -> u32 {
        let counts_by_element = self.template.chars()
            .fold(HashMap::new(), |mut counts, c| {
                *counts.entry(c).or_insert(0) += 1;
                counts
            });

        counts_by_element.values().max().unwrap() - counts_by_element.values().min().unwrap()
    }
}

impl FromStr for Polymerizer {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut lines = string.lines();

        let template = String::from(lines.next().unwrap());

        let rules: HashMap<String, String> = lines
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut components = line.split(" -> ");

                (
                    String::from(components.next().unwrap()),
                    String::from(components.next().unwrap()),
                )
            })
            .collect();

        Ok(Polymerizer { template, rules })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_POLYMERIZER_STRING: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn test_polymerizer_from_string() {
        let expected = {
            let rules = vec![
                (String::from("CH"), String::from("B")),
                (String::from("HH"), String::from("N")),
                (String::from("CB"), String::from("H")),
                (String::from("NH"), String::from("C")),
                (String::from("HB"), String::from("C")),
                (String::from("HC"), String::from("B")),
                (String::from("HN"), String::from("C")),
                (String::from("NN"), String::from("C")),
                (String::from("BH"), String::from("H")),
                (String::from("NC"), String::from("B")),
                (String::from("NB"), String::from("B")),
                (String::from("BN"), String::from("B")),
                (String::from("BB"), String::from("N")),
                (String::from("BC"), String::from("B")),
                (String::from("CC"), String::from("N")),
                (String::from("CN"), String::from("C")),
            ]
            .into_iter()
            .collect();

            Polymerizer {
                template: String::from("NNCB"),
                rules,
            }
        };

        assert_eq!(expected, Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap());
    }

    #[test]
    fn test_advance() {
        let polymerizer = Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap();

        let polymerizer = polymerizer.advance();
        assert_eq!(String::from("NCNBCHB"), polymerizer.template);

        let polymerizer = polymerizer.advance();
        assert_eq!(String::from("NBCCNBBBCBHCB"), polymerizer.template);

        let polymerizer = polymerizer.advance();
        assert_eq!(String::from("NBBBCNCCNBBNBNBBCHBHHBCHB"), polymerizer.template);

        let polymerizer = polymerizer.advance();
        assert_eq!(String::from("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"), polymerizer.template);
    }

    #[test]
    fn test_element_spread() {
        let mut polymerizer = Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap();

        for _ in 0..10 {
            polymerizer = polymerizer.advance();
        }

        assert_eq!(1588, polymerizer.element_spread());
    }
}
