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
}