use crate::rotation::{RotationMatrix, ORIENTATIONS};
use crate::vector::Vector3d;
use std::collections::HashSet;
use std::str::FromStr;

pub fn from_lines(
    lines: impl Iterator<Item = String>,
) -> Result<Vec<PointCloud>, Box<dyn std::error::Error>> {
    let mut point_clouds = Vec::new();
    let mut collected_points = Vec::new();

    for line in lines {
        if line.starts_with("---") {
            continue;
        }

        if line.is_empty() {
            point_clouds.push(PointCloud {
                points: collected_points.clone(),
            });
            collected_points.clear();
        } else {
            collected_points.push(Vector3d::from_str(line.as_str())?);
        }
    }

    // Add any trailing lines
    if !collected_points.is_empty() {
        point_clouds.push(PointCloud {
            points: collected_points.clone(),
        });
    }

    Ok(point_clouds)
}

#[derive(Debug, Eq, PartialEq)]
pub struct PointCloud {
    points: Vec<Vector3d>,
}

impl PointCloud {
    pub fn transform(&self, rotation: &RotationMatrix, translation: Vector3d) -> Self {
        self.rotate(rotation).translate(translation)
    }

    fn translate(&self, delta: Vector3d) -> Self {
        PointCloud {
            points: self.points.iter().map(|&point| point + delta).collect(),
        }
    }

    fn rotate(&self, rotation: &RotationMatrix) -> Self {
        PointCloud {
            points: self
                .points
                .iter()
                .map(|point| point.rotate(rotation))
                .collect(),
        }
    }

    pub fn overlap_transform(
        &self,
        other: &PointCloud,
        n: usize,
    ) -> Option<(RotationMatrix, Vector3d)> {
        let other_point_set = other.points.iter().collect();

        for orientation in ORIENTATIONS {
            let rotated = self.rotate(&orientation);

            // For each point in this cloud in this orientation, hypothesize that the point is the
            // same as each point in the other cloud and see if we find alignment.
            for &local_point in &rotated.points {
                for &other_point in &other.points {
                    let offset = other_point - local_point;
                    let translated = rotated.translate(offset);

                    let translated_set: HashSet<&Vector3d> = translated.points.iter().collect();
                    let common_points = translated_set.intersection(&other_point_set);

                    if common_points.count() >= n {
                        return Some((orientation, offset));
                    }
                }
            }
        }

        None
    }

    pub fn points(&self) -> Vec<Vector3d> {
        self.points.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::point_cloud::{from_lines, PointCloud};
    use crate::rotation::ORIENTATIONS;
    use crate::vector::Vector3d;
    use std::collections::HashSet;

    const TEST_SCANNER_STRING: &str = include_str!("test_points.txt");

    #[test]
    fn test_from_lines() {
        assert_eq!(
            5,
            from_lines(TEST_SCANNER_STRING.lines().map(|line| String::from(line)))
                .unwrap()
                .len()
        );
    }

    #[test]
    fn test_translate() {
        assert_eq!(
            PointCloud {
                points: vec![
                    Vector3d::new(2, 3, 4),
                    Vector3d::new(3, 4, 5),
                    Vector3d::new(4, 5, 6),
                ],
            },
            PointCloud {
                points: vec![
                    Vector3d::new(1, 1, 1),
                    Vector3d::new(2, 2, 2),
                    Vector3d::new(3, 3, 3),
                ],
            }
            .translate(Vector3d::new(1, 2, 3))
        )
    }

    #[test]
    fn test_rotate() {
        assert_eq!(
            PointCloud {
                points: vec![
                    Vector3d::new(0, 1, 0),
                    Vector3d::new(-1, 0, 0),
                    Vector3d::new(0, 0, 1),
                ],
            },
            PointCloud {
                points: vec![
                    Vector3d::new(1, 0, 0),
                    Vector3d::new(0, 1, 0),
                    Vector3d::new(0, 0, 1),
                ],
            }
            .rotate(&ORIENTATIONS[1])
        )
    }

    #[test]
    fn test_overlap_transform_2d() {
        let scanner_0_cloud = PointCloud {
            points: vec![
                Vector3d::new(0, 2, 0),
                Vector3d::new(4, 1, 0),
                Vector3d::new(3, 3, 0),
            ],
        };

        let scanner_1_cloud = PointCloud {
            points: vec![
                Vector3d::new(-1, -1, 0),
                Vector3d::new(-5, 0, 0),
                Vector3d::new(-2, 1, 0),
            ],
        };

        let (rotation, translation) = scanner_1_cloud
            .overlap_transform(&scanner_0_cloud, 3)
            .unwrap();

        // In this example, no rotation should be required
        assert_eq!(ORIENTATIONS[0], rotation);
        assert_eq!(Vector3d::new(5, 2, 0), translation);

        assert!(scanner_1_cloud
            .overlap_transform(&scanner_0_cloud, 4)
            .is_none());
    }

    #[test]
    fn test_overlap_transform_3d() {
        let point_clouds =
            from_lines(TEST_SCANNER_STRING.lines().map(|line| String::from(line))).unwrap();

        let expected_common_points: HashSet<Vector3d> = vec![
            Vector3d::new(-618, -824, -621),
            Vector3d::new(-537, -823, -458),
            Vector3d::new(-447, -329, 318),
            Vector3d::new(404, -588, -901),
            Vector3d::new(544, -627, -890),
            Vector3d::new(528, -643, 409),
            Vector3d::new(-661, -816, -575),
            Vector3d::new(390, -675, -793),
            Vector3d::new(423, -701, 434),
            Vector3d::new(-345, -311, 381),
            Vector3d::new(459, -707, 401),
            Vector3d::new(-485, -357, 347),
        ]
        .into_iter()
        .collect();

        let (rotation, translation) = point_clouds[1]
            .overlap_transform(&point_clouds[0], 12)
            .unwrap();

        assert_eq!(Vector3d::new(68, -1246, -43), translation);

        let common_points: HashSet<Vector3d> = point_clouds[0]
            .points()
            .into_iter()
            .collect::<HashSet<Vector3d>>()
            .intersection(
                &point_clouds[1]
                    .transform(&rotation, translation)
                    .points()
                    .into_iter()
                    .collect::<HashSet<Vector3d>>(),
            )
            .cloned()
            .collect();

        assert_eq!(expected_common_points, common_points);
    }
}
