use self::ExplorationQueueEntry::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let cave_graph = CaveGraph::from_str(std::fs::read_to_string(path)?.as_str())?;

        println!(
            "Distinct paths through caves (revisiting small caves not allowed): {}",
            cave_graph.find_paths(false).len()
        );

        println!(
            "Distinct paths through caves (revisiting small caves allowed): {}",
            cave_graph.find_paths(true).len()
        );

        Ok(())
    } else {
        Err("Usage: day12 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct CaveGraph {
    connections: HashMap<String, HashSet<String>>,
}

impl CaveGraph {
    pub fn find_paths(&self, allow_small_cave_revisit: bool) -> HashSet<Vec<&str>> {
        let mut paths = HashSet::new();

        let mut current_path = Vec::new();
        let mut exploration_stack = vec![Cave("start")];

        while let Some(entry) = exploration_stack.pop() {
            match entry {
                Cave(cave) => {
                    current_path.push(cave);

                    if cave == "end" {
                        paths.insert(current_path.clone());
                    } else {
                        self.connections
                            .get(cave)
                            .unwrap()
                            .iter()
                            .filter(|&connection| {
                                Self::allow_visit(
                                    connection,
                                    &current_path,
                                    allow_small_cave_revisit,
                                )
                            })
                            .for_each(|connection| {
                                exploration_stack.push(Backtrack);
                                exploration_stack.push(Cave(connection.as_str()));
                            });
                    }
                }
                Backtrack => {
                    current_path.pop();
                }
            }
        }

        paths
    }

    fn is_big_cave(cave_id: &str) -> bool {
        cave_id.chars().all(|c| c.is_uppercase())
    }

    fn allow_visit(cave: &str, path: &[&str], allow_small_cave_revisit: bool) -> bool {
        if Self::is_big_cave(cave) {
            true
        } else if cave == "start" {
            false
        } else if !path.contains(&cave) {
            // Always allow small caves if they're not the start/end and they're not already
            // in the path
            true
        } else if allow_small_cave_revisit {
            // Allow revisiting of a single small cave; that can only happen if no other
            // small cave has been visited more than once
            let mut visited_caves = HashSet::new();

            path.iter()
                .filter(|&c| !Self::is_big_cave(c))
                .all(|c| visited_caves.insert(c))
        } else {
            false
        }
    }
}

impl FromStr for CaveGraph {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut connections = HashMap::new();

        string
            .lines()
            .map(|line| {
                let mut pieces = line.split('-');
                (pieces.next().unwrap(), pieces.next().unwrap())
            })
            .for_each(|(origin, destination)| {
                connections
                    .entry(String::from(origin))
                    .or_insert_with(HashSet::new)
                    .insert(String::from(destination));

                connections
                    .entry(String::from(destination))
                    .or_insert_with(HashSet::new)
                    .insert(String::from(origin));
            });

        if !connections.contains_key("start") {
            Err("No start node found".into())
        } else if !connections.contains_key("end") {
            Err("No end node found".into())
        } else {
            Ok(CaveGraph { connections })
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum ExplorationQueueEntry<'a> {
    Cave(&'a str),
    Backtrack,
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_CAVE_STRING: &str = indoc! {"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "};

    #[test]
    fn test_cave_graph_from_string() {
        let expected = {
            let mut connections = HashMap::new();

            connections.insert(
                String::from("start"),
                ["A", "b"].iter().map(|&id| String::from(id)).collect(),
            );

            connections.insert(
                String::from("A"),
                ["start", "b", "c", "end"]
                    .iter()
                    .map(|&id| String::from(id))
                    .collect(),
            );

            connections.insert(
                String::from("b"),
                ["start", "A", "d", "end"]
                    .iter()
                    .map(|&id| String::from(id))
                    .collect(),
            );

            connections.insert(
                String::from("c"),
                ["A"].iter().map(|&id| String::from(id)).collect(),
            );

            connections.insert(
                String::from("d"),
                ["b"].iter().map(|&id| String::from(id)).collect(),
            );

            connections.insert(
                String::from("end"),
                ["A", "b"].iter().map(|&id| String::from(id)).collect(),
            );

            CaveGraph { connections }
        };

        assert_eq!(expected, CaveGraph::from_str(TEST_CAVE_STRING).unwrap());
    }

    #[test]
    fn test_is_big_cave() {
        assert!(CaveGraph::is_big_cave("A"));
        assert!(!CaveGraph::is_big_cave("start"));
    }

    #[test]
    fn test_find_paths() {
        let expected = {
            let mut expected = HashSet::new();
            expected.insert(vec!["start", "A", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "end"]);
            expected.insert(vec!["start", "b", "end"]);

            expected
        };

        assert_eq!(
            expected,
            CaveGraph::from_str(TEST_CAVE_STRING)
                .unwrap()
                .find_paths(false)
        );

        let medium_cave_graph_string = indoc! {"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        "};

        assert_eq!(
            19,
            CaveGraph::from_str(medium_cave_graph_string)
                .unwrap()
                .find_paths(false)
                .len()
        );

        let large_cave_graph_string = indoc! {"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        "};

        assert_eq!(
            226,
            CaveGraph::from_str(large_cave_graph_string)
                .unwrap()
                .find_paths(false)
                .len()
        );
    }

    #[test]
    fn test_find_paths_with_revisit() {
        let expected = {
            let mut expected = HashSet::new();
            expected.insert(vec!["start", "A", "b", "A", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "c", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "c", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "c", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "d", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "d", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "b", "d", "b", "end"]);
            expected.insert(vec!["start", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "d", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "d", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "c", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "c", "A", "b", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "b", "end"]);
            expected.insert(vec!["start", "b", "A", "c", "A", "b", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "c", "A", "b", "end"]);
            expected.insert(vec!["start", "b", "A", "c", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "b", "A", "end"]);
            expected.insert(vec!["start", "b", "d", "b", "A", "c", "A", "end"]);
            expected.insert(vec!["start", "b", "d", "b", "A", "end"]);
            expected.insert(vec!["start", "b", "d", "b", "end"]);
            expected.insert(vec!["start", "b", "end"]);

            expected
        };

        assert_eq!(
            expected,
            CaveGraph::from_str(TEST_CAVE_STRING)
                .unwrap()
                .find_paths(true)
        );

        let medium_cave_graph_string = indoc! {"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        "};

        assert_eq!(
            103,
            CaveGraph::from_str(medium_cave_graph_string)
                .unwrap()
                .find_paths(true)
                .len()
        );

        let large_cave_graph_string = indoc! {"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        "};

        assert_eq!(3509,
            CaveGraph::from_str(large_cave_graph_string)
                .unwrap()
                .find_paths(true)
                .len()
        );
    }

    #[test]
    fn test_allow_visit() {
        assert!(!CaveGraph::allow_visit("start", &vec!["start", "a", "b"], false));
        assert!(!CaveGraph::allow_visit("start", &vec!["start", "a", "b"], true));

        assert!(CaveGraph::allow_visit("c", &vec!["start", "a", "b"], false));
        assert!(CaveGraph::allow_visit("c", &vec!["start", "a", "b"], true));

        assert!(!CaveGraph::allow_visit("a", &vec!["start", "a", "b"], false));
        assert!(CaveGraph::allow_visit("a", &vec!["start", "a", "b"], true));

        assert!(!CaveGraph::allow_visit("a", &vec!["start", "a", "b", "a"], false));
        assert!(!CaveGraph::allow_visit("a", &vec!["start", "a", "b", "a"], true));
        assert!(!CaveGraph::allow_visit("b", &vec!["start", "a", "b", "a"], false));
        assert!(!CaveGraph::allow_visit("b", &vec!["start", "a", "b", "a"], true));

        assert!(CaveGraph::allow_visit("A", &vec!["start", "A", "a", "A", "b", "a"], false));
        assert!(CaveGraph::allow_visit("A", &vec!["start", "A", "a", "A", "b", "a"], true));
    }
}
