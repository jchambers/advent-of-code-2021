mod point_cloud;
mod rotation;
mod vector;

use crate::point_cloud::PointCloud;
use crate::rotation::{RotationMatrix, ORIENTATIONS};
use crate::vector::Vector3d;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let point_clouds = point_cloud::from_lines(
            io::BufReader::new(File::open(path)?)
                .lines()
                .map(|line| line.unwrap()),
        )
        .unwrap();

        println!(
            "Distinct beacons: {}",
            distinct_beacons(&point_clouds).len()
        );

        println!(
            "Max distance between sensors: {}",
            max_sensor_distance(&point_clouds)
        );

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

fn align_point_clouds(point_clouds: &[PointCloud]) -> Vec<(RotationMatrix, Vector3d)> {
    let mut transformations = vec![None; point_clouds.len()];

    // Treat the first point cloud as our origin in terms of both position and orientation
    transformations[0] = Some((ORIENTATIONS[0], Vector3d::new(0, 0, 0)));

    while transformations.iter().any(Option::is_none) {
        // Find so-far-unaligned point clouds
        for i in 0..transformations.len() {
            if transformations[i].is_none() {
                // Can we find an already-aligned point cloud that overlaps with this one?
                if let Some(transformation) = transformations
                    .iter()
                    .enumerate()
                    .filter_map(|(j, maybe_transform)| {
                        maybe_transform.map(|(rotation, translation)| {
                            point_clouds[j].transform(&rotation, translation)
                        })
                    })
                    .find_map(|transformed_cloud| {
                        point_clouds[i].overlap_transform(&transformed_cloud, 12)
                    })
                {
                    transformations[i] = Some(transformation);
                }
            }
        }
    }

    transformations
        .iter()
        .map(|maybe_transformation| maybe_transformation.unwrap())
        .collect()
}

fn distinct_beacons(point_clouds: &[PointCloud]) -> HashSet<Vector3d> {
    let transformations = align_point_clouds(point_clouds);

    let mut distinct_beacons = HashSet::new();

    for (cloud, (rotation, translation)) in point_clouds.iter().zip(transformations.iter()) {
        distinct_beacons.extend(cloud.transform(rotation, *translation).points());
    }

    distinct_beacons
}

fn max_sensor_distance(point_clouds: &[PointCloud]) -> u32 {
    let positions: Vec<Vector3d> = align_point_clouds(point_clouds)
        .iter()
        .map(|(_, translation)| *translation)
        .collect();

    let mut max_distance = 0;

    for i in 0..positions.len() - 1 {
        for j in i..positions.len() {
            max_distance = max(max_distance, positions[i].manhattan_distance(&positions[j]));
        }
    }

    max_distance
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_SCANNER_STRING: &str = include_str!("test_points.txt");

    #[test]
    fn test_align_point_clouds() {
        let point_clouds =
            point_cloud::from_lines(TEST_SCANNER_STRING.lines().map(String::from))
                .unwrap();

        let transformations = align_point_clouds(&point_clouds);

        assert_eq!(Vector3d::new(0, 0, 0), transformations[0].1);
        assert_eq!(Vector3d::new(68, -1246, -43), transformations[1].1);
        assert_eq!(Vector3d::new(1105, -1205, 1229), transformations[2].1);
        assert_eq!(Vector3d::new(-92, -2380, -20), transformations[3].1);
        assert_eq!(Vector3d::new(-20, -1133, 1061), transformations[4].1);
    }

    #[test]
    fn test_distinct_beacons() {
        let expected_beacons: HashSet<Vector3d> = vec![
            Vector3d::new(-892, 524, 684),
            Vector3d::new(-876, 649, 763),
            Vector3d::new(-838, 591, 734),
            Vector3d::new(-789, 900, -551),
            Vector3d::new(-739, -1745, 668),
            Vector3d::new(-706, -3180, -659),
            Vector3d::new(-697, -3072, -689),
            Vector3d::new(-689, 845, -530),
            Vector3d::new(-687, -1600, 576),
            Vector3d::new(-661, -816, -575),
            Vector3d::new(-654, -3158, -753),
            Vector3d::new(-635, -1737, 486),
            Vector3d::new(-631, -672, 1502),
            Vector3d::new(-624, -1620, 1868),
            Vector3d::new(-620, -3212, 371),
            Vector3d::new(-618, -824, -621),
            Vector3d::new(-612, -1695, 1788),
            Vector3d::new(-601, -1648, -643),
            Vector3d::new(-584, 868, -557),
            Vector3d::new(-537, -823, -458),
            Vector3d::new(-532, -1715, 1894),
            Vector3d::new(-518, -1681, -600),
            Vector3d::new(-499, -1607, -770),
            Vector3d::new(-485, -357, 347),
            Vector3d::new(-470, -3283, 303),
            Vector3d::new(-456, -621, 1527),
            Vector3d::new(-447, -329, 318),
            Vector3d::new(-430, -3130, 366),
            Vector3d::new(-413, -627, 1469),
            Vector3d::new(-345, -311, 381),
            Vector3d::new(-36, -1284, 1171),
            Vector3d::new(-27, -1108, -65),
            Vector3d::new(7, -33, -71),
            Vector3d::new(12, -2351, -103),
            Vector3d::new(26, -1119, 1091),
            Vector3d::new(346, -2985, 342),
            Vector3d::new(366, -3059, 397),
            Vector3d::new(377, -2827, 367),
            Vector3d::new(390, -675, -793),
            Vector3d::new(396, -1931, -563),
            Vector3d::new(404, -588, -901),
            Vector3d::new(408, -1815, 803),
            Vector3d::new(423, -701, 434),
            Vector3d::new(432, -2009, 850),
            Vector3d::new(443, 580, 662),
            Vector3d::new(455, 729, 728),
            Vector3d::new(456, -540, 1869),
            Vector3d::new(459, -707, 401),
            Vector3d::new(465, -695, 1988),
            Vector3d::new(474, 580, 667),
            Vector3d::new(496, -1584, 1900),
            Vector3d::new(497, -1838, -617),
            Vector3d::new(527, -524, 1933),
            Vector3d::new(528, -643, 409),
            Vector3d::new(534, -1912, 768),
            Vector3d::new(544, -627, -890),
            Vector3d::new(553, 345, -567),
            Vector3d::new(564, 392, -477),
            Vector3d::new(568, -2007, -577),
            Vector3d::new(605, -1665, 1952),
            Vector3d::new(612, -1593, 1893),
            Vector3d::new(630, 319, -379),
            Vector3d::new(686, -3108, -505),
            Vector3d::new(776, -3184, -501),
            Vector3d::new(846, -3110, -434),
            Vector3d::new(1135, -1161, 1235),
            Vector3d::new(1243, -1093, 1063),
            Vector3d::new(1660, -552, 429),
            Vector3d::new(1693, -557, 386),
            Vector3d::new(1735, -437, 1738),
            Vector3d::new(1749, -1800, 1813),
            Vector3d::new(1772, -405, 1572),
            Vector3d::new(1776, -675, 371),
            Vector3d::new(1779, -442, 1789),
            Vector3d::new(1780, -1548, 337),
            Vector3d::new(1786, -1538, 337),
            Vector3d::new(1847, -1591, 415),
            Vector3d::new(1889, -1729, 1762),
            Vector3d::new(1994, -1805, 1792),
        ]
        .into_iter()
        .collect();

        let point_clouds =
            point_cloud::from_lines(TEST_SCANNER_STRING.lines().map(String::from))
                .unwrap();

        assert_eq!(expected_beacons, distinct_beacons(&point_clouds));
    }

    #[test]
    fn test_max_sensor_distance() {
        let point_clouds =
            point_cloud::from_lines(TEST_SCANNER_STRING.lines().map(String::from))
                .unwrap();

        assert_eq!(3621, max_sensor_distance(&point_clouds));
    }
}
