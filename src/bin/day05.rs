use std::{env, error, io};
use std::cmp::{max, min};
use std::fs::File;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut vent_map = VentMap::default();

        io::BufReader::new(File::open(path)?)
            .lines()
            .map(|line| LineSegment::try_from(line.unwrap().as_str()).unwrap())
            .filter(|segment| segment.is_horizontal() || segment.is_vertical())
            .for_each(|segment| vent_map.add_line_segment(&segment));

        println!("Cells with multiple vents (horizontal/vertical only): {}", vent_map.get_multi_vent_cell_count());

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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
}

impl TryFrom<&str> for LineSegment {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pieces: Vec<&str> = value.split(" -> ").collect();

        if pieces.len() != 2 {
            return Err("Line segments must have two vectors separated by \" -> \"".into());
        }

        Ok(LineSegment {
            start: Point::try_from(pieces[0])?,
            end: Point::try_from(pieces[1])?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct VentMap {
    cells: Vec<Vec<u32>>,
}

impl VentMap {
    pub fn add_line_segment(&mut self, segment: &LineSegment) {
        self.resize_to_fit(segment);

        let min_x = min(segment.start.x, segment.end.x);
        let max_x = max(segment.start.x, segment.end.x);
        let min_y = min(segment.start.y, segment.end.y);
        let max_y = max(segment.start.y, segment.end.y);

        let mut row = segment.start.y;
        let mut col = segment.start.x;

        while row >= min_y && row <= max_y && col >= min_x && col <= max_x {
            self.cells[row][col] = self.cells[row][col] + 1;

            if segment.start.x < segment.end.x {
                col += 1;
            } else if segment.start.x > segment.end.x {
                col -= 1;
            }

            if segment.start.y < segment.end.y {
                row += 1;
            } else if segment.start.y > segment.end.y {
                row -= 1;
            }
        }
    }

    fn resize_to_fit(&mut self, segment: &LineSegment) {
        let max_x = max(max(segment.start.x, segment.end.x), self.cells.len());
        let max_y = max(max(segment.start.y, segment.end.y), self.cells[0].len());

        if max_y >= self.cells.len() {
            self.cells.resize_with(max_y + 1, || vec![0; max_x]);
        }

        if max_x >= self.cells[0].len() {
            self.cells.iter_mut()
                .for_each(|vec| vec.resize(max_x + 1, 0));
        }
    }

    pub fn get_multi_vent_cell_count(&self) -> u32 {
        self.cells.iter()
            .flat_map(|row| row)
            .filter(|&&cell| cell > 1)
            .count() as u32
    }
}

impl Default for VentMap {
    fn default() -> Self {
        VentMap { cells: vec![vec![0]] }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_point_from_string() {
        assert_eq!(Point { x: 17, y: 12 }, Point::try_from("17,12").unwrap());
    }

    #[test]
    fn test_line_segment_from_string() {
        assert_eq!(LineSegment {
            start: Point { x: 1, y: 2 },
            end: Point { x: 87, y: 22 },
        }, LineSegment::try_from("1,2 -> 87,22").unwrap());
    }

    #[test]
    fn test_line_segment_is_horizontal() {
        assert!(LineSegment::try_from("1,7 -> 12,7").unwrap().is_horizontal());
        assert!(!LineSegment::try_from("1,7 -> 12,9").unwrap().is_horizontal());
    }

    #[test]
    fn test_line_segment_is_vertical() {
        assert!(LineSegment::try_from("12,7 -> 12,19").unwrap().is_vertical());
        assert!(!LineSegment::try_from("7,7 -> 17,7").unwrap().is_vertical());
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
            "5,5 -> 8,2"
        ]
            .iter()
            .map(|line| LineSegment::try_from(*line).unwrap())
            .collect();

        {
            let mut vent_map = VentMap::default();

            segments.iter()
                .filter(|segment| segment.is_horizontal() || segment.is_vertical())
                .for_each(|segment| vent_map.add_line_segment(segment));

            assert_eq!(5, vent_map.get_multi_vent_cell_count());
        }
    }
}
