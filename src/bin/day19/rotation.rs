pub const ORIENTATIONS: [RotationMatrix; 24] = [
    RotationMatrix::new(0, 0, 0),
    RotationMatrix::new(0, 0, 1),
    RotationMatrix::new(0, 0, 2),
    RotationMatrix::new(0, 0, 3),
    RotationMatrix::new(0, 1, 0),
    RotationMatrix::new(0, 1, 1),
    RotationMatrix::new(0, 1, 2),
    RotationMatrix::new(0, 1, 3),
    RotationMatrix::new(0, 2, 0),
    RotationMatrix::new(0, 2, 1),
    RotationMatrix::new(0, 2, 2),
    RotationMatrix::new(0, 2, 3),
    RotationMatrix::new(0, 3, 0),
    RotationMatrix::new(0, 3, 1),
    RotationMatrix::new(0, 3, 2),
    RotationMatrix::new(0, 3, 3),
    RotationMatrix::new(1, 0, 0),
    RotationMatrix::new(1, 0, 1),
    RotationMatrix::new(1, 0, 2),
    RotationMatrix::new(1, 0, 3),
    RotationMatrix::new(3, 0, 0),
    RotationMatrix::new(3, 0, 1),
    RotationMatrix::new(3, 0, 2),
    RotationMatrix::new(3, 0, 3),
];

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RotationMatrix {
    matrix: [[i32; 3]; 3],
}

impl RotationMatrix {
    const fn new(x: u32, y: u32, z: u32) -> Self {
        RotationMatrix {
            matrix: [
                [
                    cos(z) * cos(y),
                    cos(z) * sin(y) * sin(x) - sin(z) * cos(x),
                    cos(z) * sin(y) * cos(x) + sin(z) * sin(x),
                ],
                [
                    sin(z) * cos(y),
                    sin(z) * sin(y) * sin(x) + cos(z) * cos(x),
                    sin(z) * sin(y) * cos(x) - cos(z) * sin(x),
                ],
                [-sin(y), cos(y) * sin(x), cos(y) * cos(x)],
            ],
        }
    }

    pub fn apply(&self, vector: &[i32; 3]) -> [i32; 3] {
        let mut rotated = [0; 3];

        for i in 0..3 {
            for j in 0..3 {
                rotated[i] += self.matrix[i][j] * vector[j];
            }
        }

        rotated
    }
}

const fn sin(half_pi: u32) -> i32 {
    match half_pi % 4 {
        1 => 1,
        3 => -1,
        _ => 0,
    }
}

const fn cos(half_pi: u32) -> i32 {
    match half_pi % 4 {
        0 => 1,
        2 => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::rotation::{RotationMatrix, ORIENTATIONS};
    use std::collections::HashSet;

    #[test]
    fn test_distinct_orientations() {
        let distinct_orientations: HashSet<RotationMatrix> = ORIENTATIONS.into_iter().collect();
        assert_eq!(24, distinct_orientations.len());
    }

    #[test]
    fn test_apply() {
        assert_eq!([1, 2, 3], RotationMatrix::new(0, 0, 0).apply(&[1, 2, 3]));

        assert_eq!([0, 0, 1], RotationMatrix::new(0, 0, 1).apply(&[0, 0, 1]));
        assert_eq!([0, 1, 0], RotationMatrix::new(0, 0, 1).apply(&[1, 0, 0]));
        assert_eq!([-1, 0, 0], RotationMatrix::new(0, 0, 2).apply(&[1, 0, 0]));
        assert_eq!([0, -1, 0], RotationMatrix::new(0, 0, 3).apply(&[1, 0, 0]));

        assert_eq!([0, 1, 0], RotationMatrix::new(0, 1, 0).apply(&[0, 1, 0]));
        assert_eq!([0, 0, -1], RotationMatrix::new(0, 1, 0).apply(&[1, 0, 0]));
        assert_eq!([-1, 0, 0], RotationMatrix::new(0, 2, 0).apply(&[1, 0, 0]));
        assert_eq!([0, 0, 1], RotationMatrix::new(0, 3, 0).apply(&[1, 0, 0]));

        assert_eq!([1, 0, 0], RotationMatrix::new(1, 0, 0).apply(&[1, 0, 0]));
        assert_eq!([0, 0, 1], RotationMatrix::new(1, 0, 0).apply(&[0, 1, 0]));
        assert_eq!([0, -1, 0], RotationMatrix::new(2, 0, 0).apply(&[0, 1, 0]));
        assert_eq!([0, 0, -1], RotationMatrix::new(3, 0, 0).apply(&[0, 1, 0]));

        assert_eq!([0, 1, 0], RotationMatrix::new(1, 0, 1).apply(&[1, 0, 0]));
    }
}
