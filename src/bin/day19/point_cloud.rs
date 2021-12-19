use crate::rotation::{RotationMatrix, ORIENTATIONS};
use crate::vector::Vector3d;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct PointCloud {
    points: Vec<Vector3d>,
}

impl PointCloud {
    pub fn translate(&self, delta: Vector3d) -> Self {
        PointCloud {
            points: self.points.iter().map(|&point| point + delta).collect(),
        }
    }

    pub fn rotate(&self, rotation: &RotationMatrix) -> Self {
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

    pub fn common_points(
        &self,
        other: &PointCloud,
        rotation: &RotationMatrix,
        translation: Vector3d,
    ) -> HashSet<Vector3d> {
        let transformed_set: HashSet<Vector3d> = self
            .rotate(rotation)
            .translate(translation)
            .points
            .into_iter()
            .collect();

        transformed_set
            .intersection(&other.points.iter().cloned().collect())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::point_cloud::PointCloud;
    use crate::rotation::ORIENTATIONS;
    use crate::vector::Vector3d;
    use std::collections::HashSet;

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
        let scanner_0_cloud = PointCloud {
            points: vec![
                Vector3d::new(404, -588, -901),
                Vector3d::new(528, -643, 409),
                Vector3d::new(-838, 591, 734),
                Vector3d::new(390, -675, -793),
                Vector3d::new(-537, -823, -458),
                Vector3d::new(-485, -357, 347),
                Vector3d::new(-345, -311, 381),
                Vector3d::new(-661, -816, -575),
                Vector3d::new(-876, 649, 763),
                Vector3d::new(-618, -824, -621),
                Vector3d::new(553, 345, -567),
                Vector3d::new(474, 580, 667),
                Vector3d::new(-447, -329, 318),
                Vector3d::new(-584, 868, -557),
                Vector3d::new(544, -627, -890),
                Vector3d::new(564, 392, -477),
                Vector3d::new(455, 729, 728),
                Vector3d::new(-892, 524, 684),
                Vector3d::new(-689, 845, -530),
                Vector3d::new(423, -701, 434),
                Vector3d::new(7, -33, -71),
                Vector3d::new(630, 319, -379),
                Vector3d::new(443, 580, 662),
                Vector3d::new(-789, 900, -551),
                Vector3d::new(459, -707, 401),
            ],
        };

        let scanner_1_cloud = PointCloud {
            points: vec![
                Vector3d::new(686, 422, 578),
                Vector3d::new(605, 423, 415),
                Vector3d::new(515, 917, -361),
                Vector3d::new(-336, 658, 858),
                Vector3d::new(95, 138, 22),
                Vector3d::new(-476, 619, 847),
                Vector3d::new(-340, -569, -846),
                Vector3d::new(567, -361, 727),
                Vector3d::new(-460, 603, -452),
                Vector3d::new(669, -402, 600),
                Vector3d::new(729, 430, 532),
                Vector3d::new(-500, -761, 534),
                Vector3d::new(-322, 571, 750),
                Vector3d::new(-466, -666, -811),
                Vector3d::new(-429, -592, 574),
                Vector3d::new(-355, 545, -477),
                Vector3d::new(703, -491, -529),
                Vector3d::new(-328, -685, 520),
                Vector3d::new(413, 935, -424),
                Vector3d::new(-391, 539, -444),
                Vector3d::new(586, -435, 557),
                Vector3d::new(-364, -763, -893),
                Vector3d::new(807, -499, -711),
                Vector3d::new(755, -354, -619),
                Vector3d::new(553, 889, -390),
            ],
        };

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

        let (rotation, translation) = scanner_1_cloud
            .overlap_transform(&scanner_0_cloud, 12)
            .unwrap();

        assert_eq!(Vector3d::new(68, -1246, -43), translation);
        assert_eq!(
            expected_common_points,
            scanner_1_cloud.common_points(&scanner_0_cloud, &rotation, translation)
        );
    }
}
