use crate::Fold::{Horizontal, Vertical};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let (page, folds) = load_page_and_folds(
            io::BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok()),
        );

        let page = page.apply_folds(&folds[0..1]);

        println!("Dots visible after first fold: {}", page.distinct_points());

        println!(
            "Page after applying all folds:\n{}",
            page.apply_folds(&folds[1..])
        );

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

fn load_page_and_folds(lines: impl Iterator<Item = String>) -> (TransparentPage, Vec<Fold>) {
    let (point_lines, fold_lines): (Vec<String>, Vec<String>) = lines
        .filter(|line| !line.is_empty())
        .partition(|line| !line.starts_with("fold along"));

    let points: HashSet<Point> = point_lines
        .iter()
        .map(|line| Point::from_str(line).unwrap())
        .collect();

    let folds: Vec<Fold> = fold_lines
        .iter()
        .map(|line| Fold::from_str(line).unwrap())
        .collect();

    (TransparentPage { points }, folds)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = Box<dyn error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut pieces = line.split(',');

        Ok(Point {
            x: u32::from_str(pieces.next().unwrap())?,
            y: u32::from_str(pieces.next().unwrap())?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TransparentPage {
    points: HashSet<Point>,
}

impl TransparentPage {
    pub fn apply_folds(self, folds: &[Fold]) -> Self {
        let mut page = self;

        for fold in folds {
            page = page.apply_fold(fold);
        }

        page
    }

    fn apply_fold(self, fold: &Fold) -> Self {
        let points = self.points
            .iter()
            .map(|point| {
                match fold {
                    Horizontal(y) => {
                        if point.y > *y {
                            Point { x: point.x, y: y - (point.y - y) }
                        } else {
                            *point
                        }
                    }
                    Vertical(x) => {
                        if point.x > *x {
                            Point { x: x - (point.x - x), y: point.y }
                        } else {
                            *point
                        }
                    }
                }
            })
            .collect();

        TransparentPage { points }
    }

    pub fn distinct_points(&self) -> u32 {
        self.points.len() as u32
    }
}

impl Display for TransparentPage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.points.iter().map(|point| point.x).max().unwrap_or(0);
        let max_y = self.points.iter().map(|point| point.y).max().unwrap_or(0);

        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.points.contains(&Point { x, y }) {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Fold {
    Horizontal(u32),
    Vertical(u32),
}

impl FromStr for Fold {
    type Err = Box<dyn error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        const PREFIX_LENGTH: usize = "fold along x=".len();

        if line.starts_with("fold along x=") {
            Ok(Vertical(u32::from_str(&line[PREFIX_LENGTH..])?))
        } else if line.starts_with("fold along y=") {
            Ok(Horizontal(u32::from_str(&line[PREFIX_LENGTH..])?))
        } else {
            Err(format!("Could not parse line as fold: {}", line).into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_PAGE_AND_FOLDS_STRING: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn test_load_page_and_folds() {
        let expected_page = TransparentPage {
            points: vec![
                Point { x: 6, y: 10 },
                Point { x: 0, y: 14 },
                Point { x: 9, y: 10 },
                Point { x: 0, y: 3 },
                Point { x: 10, y: 4 },
                Point { x: 4, y: 11 },
                Point { x: 6, y: 0 },
                Point { x: 6, y: 12 },
                Point { x: 4, y: 1 },
                Point { x: 0, y: 13 },
                Point { x: 10, y: 12 },
                Point { x: 3, y: 4 },
                Point { x: 3, y: 0 },
                Point { x: 8, y: 4 },
                Point { x: 1, y: 10 },
                Point { x: 2, y: 14 },
                Point { x: 8, y: 10 },
                Point { x: 9, y: 0 },
            ]
            .into_iter()
            .collect(),
        };

        let expected_folds = vec![Horizontal(7), Vertical(5)];

        assert_eq!(
            (expected_page, expected_folds),
            load_page_and_folds(
                TEST_PAGE_AND_FOLDS_STRING
                    .lines()
                    .map(|line| String::from(line))
            )
        );
    }

    #[test]
    fn test_apply_fold() {
        let (page, folds) = load_page_and_folds(
            TEST_PAGE_AND_FOLDS_STRING
                .lines()
                .map(|line| String::from(line)),
        );

        let after_first_fold = page.apply_fold(&folds[0]);
        assert_eq!(17, after_first_fold.distinct_points());

        let after_second_fold = after_first_fold.apply_fold(&folds[1]);
        assert_eq!(16, after_second_fold.distinct_points());
    }

    #[test]
    fn test_apply_folds() {
        let (page, folds) = load_page_and_folds(
            TEST_PAGE_AND_FOLDS_STRING
                .lines()
                .map(|line| String::from(line)),
        );

        let page = page.apply_folds(&folds);
        assert_eq!(16, page.distinct_points());
    }
}
