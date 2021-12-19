use std::collections::HashSet;
use crate::rotation::{ORIENTATIONS, RotationMatrix};
use crate::vector::Vector3d;

#[derive(Debug, Eq, PartialEq)]
pub struct PointCloud {
    points: Vec<Vector3d>,
}

impl PointCloud {
    pub fn translate(&self, delta: Vector3d) -> Self {
        PointCloud {
            points: self.points.iter()
                .map(|&point| point + delta)
                .collect()
        }
    }

    pub fn rotate(&self, rotation: &RotationMatrix) -> Self {
        PointCloud {
            points: self.points.iter()
                .map(|point| point.rotate(rotation))
                .collect()
        }
    }

    pub fn overlap(&self, other: &PointCloud, n: usize) -> Option<(RotationMatrix, Vector3d)> {
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
}

#[cfg(test)]
mod test {
    use crate::point_cloud::PointCloud;
    use crate::rotation::ORIENTATIONS;
    use crate::vector::Vector3d;

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
            }.translate(Vector3d::new(1, 2, 3))
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
            }.rotate(&ORIENTATIONS[1])
        )
    }

    #[test]
    fn test_overlap_2d() {
        let scanner_0_points = PointCloud {
            points: vec![
                Vector3d::new(0, 2, 0),
                Vector3d::new(4, 1, 0),
                Vector3d::new(3, 3, 0),
            ],
        };

        let scanner_1_points = PointCloud {
            points: vec![
                Vector3d::new(-1, -1, 0),
                Vector3d::new(-5, 0, 0),
                Vector3d::new(-2, 1, 0),
            ],
        };

        let (rotation, translation) = scanner_1_points.overlap(&scanner_0_points, 3).unwrap();

        // In this example, no rotation should be required
        assert_eq!(ORIENTATIONS[0], rotation);
        assert_eq!(Vector3d::new(5, 2, 0), translation);

        assert!(scanner_1_points.overlap(&scanner_0_points, 4).is_none());
    }
}
