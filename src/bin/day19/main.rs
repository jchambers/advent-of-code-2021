mod point_cloud;
mod rotation;
mod vector;

use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};
use std::collections::HashSet;
use crate::point_cloud::PointCloud;
use crate::rotation::{ORIENTATIONS, RotationMatrix};
use crate::vector::Vector3d;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _point_clouds = point_cloud::from_lines(
            io::BufReader::new(File::open(path)?)
                .lines()
                .map(|line| line.unwrap()),
        );

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

fn align_point_clouds(point_clouds: &Vec<PointCloud>) -> Vec<(RotationMatrix, Vector3d)> {
    let mut transformations = vec![None; point_clouds.len()];

    // Treat the first point cloud as our origin in terms of both position and orientation
    transformations[0] = Some((ORIENTATIONS[0], Vector3d::new(0, 0, 0)));

    while transformations.iter().any(Option::is_none) {
        // Find so-far-unaligned point clouds
        for i in 0..transformations.len() {
            if transformations[i].is_none() {
                // Can we find an already-aligned point cloud that overlaps with this one?
                if let Some(transformation) = transformations.iter()
                    .enumerate()
                    .filter_map(|(j, maybe_transform)| {
                        maybe_transform.map(|(rotation, translation)| {
                            point_clouds[j].rotate(&rotation).translate(translation)
                        })
                    })
                    .find_map(|transformed_cloud| point_clouds[i].overlap_transform(&transformed_cloud, 12)) {

                    transformations[i] = Some(transformation);
                }
            }
        }
    }

    transformations.iter()
        .map(|maybe_transformation| maybe_transformation.unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_SCANNER_STRING: &str = include_str!("test_points.txt");

    #[test]
    fn test_align_point_clouds() {
        let point_clouds = point_cloud::from_lines(TEST_SCANNER_STRING.lines().map(|line| String::from(line)))
            .unwrap();

        let transformations = align_point_clouds(&point_clouds);


        assert_eq!(Vector3d::new(0, 0, 0), transformations[0].1);
        assert_eq!(Vector3d::new(68, -1246, -43), transformations[1].1);
        assert_eq!(Vector3d::new(1105, -1205, 1229), transformations[2].1);
        assert_eq!(Vector3d::new(-92,-2380,-20), transformations[3].1);
        assert_eq!(Vector3d::new(-20, -1133, 1061), transformations[4].1);
    }
}
