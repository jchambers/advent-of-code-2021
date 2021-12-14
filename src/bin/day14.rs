use std::collections::HashMap;
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let polymerizer = Polymerizer::from_str(std::fs::read_to_string(path)?.as_str()).unwrap();

        println!("Element spread after 10 generations: {}", polymerizer.element_spread(10));
        println!("Element spread after 40 generations: {}", polymerizer.element_spread(40));

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

type ElementPair = [char; 2];

#[derive(Debug, Eq, PartialEq)]
struct Polymerizer {
    template: Vec<char>,
    rules: HashMap<ElementPair, char>,
}

impl Polymerizer {
    pub fn element_spread(&self, generation: u32) -> u64 {
        let element_counts = self.element_counts(generation);

        element_counts.values().max().unwrap_or(&0u64) -
            element_counts.values().min().unwrap_or(&0u64)
    }

    fn element_counts(&self, generation: u32) -> HashMap<char, u64> {
        let mut counts_by_element = HashMap::new();

        // Start off the element count with what's already in the template
        for &c in &self.template {
            *counts_by_element.entry(c).or_insert(0) += 1;
        }

        let mut counts_by_pair = HashMap::new();

        for pair in self.template.windows(2) {
            *counts_by_pair
                .entry(pair.try_into().unwrap())
                .or_insert(0) += 1;
        }

        for _ in 0..generation {
            let mut next_generation = HashMap::new();

            for (pair, count) in counts_by_pair {
                // Every pair spawns exactly one new atom in the next generation…
                *counts_by_element.entry(*self.rules.get(&pair).unwrap()).or_insert(0) += count;

                // …and every pair "splits" into two child pairs.
                for advanced_pair in self.advance_pair(pair) {
                    *next_generation.entry(advanced_pair).or_insert(0) += count;
                }
            }

            counts_by_pair = next_generation;
        }

        counts_by_element
    }

    fn advance_pair(&self, pair: ElementPair) -> [ElementPair; 2] {
        let inserted_element = *self.rules.get(&pair).unwrap();

        [[pair[0], inserted_element], [inserted_element, pair[1]]]
    }
}

impl FromStr for Polymerizer {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut lines = string.lines();

        let template = lines.next().unwrap().chars().collect();

        let rules: HashMap<ElementPair, char> = lines
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut components = line.split(" -> ");

                let pair: ElementPair = components
                    .next()
                    .unwrap()
                    .chars()
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap();

                let element = components.next().unwrap().chars().next().unwrap();

                (pair, element)
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
                (['C', 'H'], 'B'),
                (['H', 'H'], 'N'),
                (['C', 'B'], 'H'),
                (['N', 'H'], 'C'),
                (['H', 'B'], 'C'),
                (['H', 'C'], 'B'),
                (['H', 'N'], 'C'),
                (['N', 'N'], 'C'),
                (['B', 'H'], 'H'),
                (['N', 'C'], 'B'),
                (['N', 'B'], 'B'),
                (['B', 'N'], 'B'),
                (['B', 'B'], 'N'),
                (['B', 'C'], 'B'),
                (['C', 'C'], 'N'),
                (['C', 'N'], 'C'),
            ]
            .into_iter()
            .collect();

            Polymerizer {
                template: vec!['N', 'N', 'C', 'B'],
                rules,
            }
        };

        assert_eq!(
            expected,
            Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap()
        );
    }

    #[test]
    fn test_advance_pair() {
        let polymerizer = Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap();

        assert_eq!(
            [['C', 'B'], ['B', 'H']],
            polymerizer.advance_pair(['C', 'H'])
        );
    }

    #[test]
    fn test_element_counts() {
        let expected: HashMap<char, u64> = vec![('B', 1749), ('C', 298), ('H', 161), ('N', 865)]
            .into_iter()
            .collect();

        let polymerizer = Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap();

        assert_eq!(expected, polymerizer.element_counts(10));
    }

    #[test]
    fn test_element_spread() {
        let polymerizer = Polymerizer::from_str(TEST_POLYMERIZER_STRING).unwrap();

        assert_eq!(1588, polymerizer.element_spread(10));
        assert_eq!(2188189693529, polymerizer.element_spread(40));
    }
}
