use std::cmp::{max, min};
use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        {
            let mut vent_map = VentMap::default();

            io::BufReader::new(File::open(path)?)
                .lines()
                .map(|line| LineSegment::from_str(line.unwrap().as_str()).unwrap())
                .filter(|segment| segment.is_horizontal() || segment.is_vertical())
                .for_each(|segment| vent_map.add_line_segment(&segment));

            println!(
                "Cells with multiple vents (horizontal/vertical only): {}",
                vent_map.get_multi_vent_cell_count()
            );
        }

        {
            let mut vent_map = VentMap::default();

            io::BufReader::new(File::open(path)?)
                .lines()
                .map(|line| LineSegment::from_str(line.unwrap().as_str()).unwrap())
                .for_each(|segment| vent_map.add_line_segment(&segment));

            println!(
                "Cells with multiple vents (all): {}",
                vent_map.get_multi_vent_cell_count()
            );
        }

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = value.split(",").collect();

        if pieces.len() != 2 {
            return Err("Points must have two comma-separated coordinates".into());
        }

        Ok(Point {
            x: pieces[0].parse()?,
            y: pieces[1].parse()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        let min_x = min(self.start.x, self.end.x);
        let max_x = max(self.start.x, self.end.x);
        let min_y = min(self.start.y, self.end.y);
        let max_y = max(self.start.y, self.end.y);

        let cols: Box<dyn Iterator<Item = usize>> = if self.is_vertical() {
            Box::new(std::iter::repeat(self.start.x))
        } else if self.start.x > self.end.x {
            Box::new((min_x..=max_x).rev())
        } else {
            Box::new(min_x..=max_x)
        };

        let rows: Box<dyn Iterator<Item = usize>> = if self.is_horizontal() {
            Box::new(std::iter::repeat(self.start.y))
        } else if self.start.y > self.end.y {
            Box::new((min_y..=max_y).rev())
        } else {
            Box::new(min_y..=max_y)
        };

        Box::new(rows.zip(cols)
            .map(|(row, col)| Point { x: col, y: row }))
    }
}

impl FromStr for LineSegment {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = value.split(" -> ").collect();

        if pieces.len() != 2 {
            return Err("Line segments must have two vectors separated by \" -> \"".into());
        }

        Ok(LineSegment {
            start: Point::from_str(pieces[0])?,
            end: Point::from_str(pieces[1])?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct VentMap {
    cells: HashMap<Point, u32>,
}

impl VentMap {
    pub fn add_line_segment(&mut self, segment: &LineSegment) {
        for point in segment.points() {
            let count = self.cells.get(&point).unwrap_or(&0) + 1;
            self.cells.insert(point, count);
        }
    }

    pub fn get_multi_vent_cell_count(&self) -> u32 {
        self.cells
            .values()
            .filter(|&&count| count > 1)
            .count() as u32
    }
}

impl Default for VentMap {
    fn default() -> Self {
        VentMap {
            cells: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_from_string() {
        assert_eq!(Point { x: 17, y: 12 }, Point::from_str("17,12").unwrap());
    }

    #[test]
    fn test_line_segment_from_string() {
        assert_eq!(
            LineSegment {
                start: Point { x: 1, y: 2 },
                end: Point { x: 87, y: 22 },
            },
            LineSegment::from_str("1,2 -> 87,22").unwrap()
        );
    }

    #[test]
    fn test_line_segment_is_horizontal() {
        assert!(LineSegment::from_str("1,7 -> 12,7").unwrap().is_horizontal());
        assert!(!LineSegment::from_str("1,7 -> 12,9").unwrap().is_horizontal());
    }

    #[test]
    fn test_line_segment_is_vertical() {
        assert!(LineSegment::from_str("12,7 -> 12,19").unwrap().is_vertical());
        assert!(!LineSegment::from_str("7,7 -> 17,7").unwrap().is_vertical());
    }

    #[test]
    fn test_get_multi_vent_cells() {
        let segments: Vec<LineSegment> = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .iter()
        .map(|line| LineSegment::from_str(*line).unwrap())
        .collect();

        {
            let mut vent_map = VentMap::default();

            segments
                .iter()
                .filter(|segment| segment.is_horizontal() || segment.is_vertical())
                .for_each(|segment| vent_map.add_line_segment(segment));

            assert_eq!(5, vent_map.get_multi_vent_cell_count());
        }

        {
            let mut vent_map = VentMap::default();

            segments
                .iter()
                .for_each(|segment| vent_map.add_line_segment(segment));

            assert_eq!(12, vent_map.get_multi_vent_cell_count());
        }
    }
}
