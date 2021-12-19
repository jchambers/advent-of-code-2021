use crate::rotation::RotationMatrix;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector3d {
    components: [i32; 3],
}

impl Vector3d {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Vector3d {
            components: [x, y, z],
        }
    }

    pub fn rotate(&self, rotation: &RotationMatrix) -> Self {
        Vector3d {
            components: rotation.apply(&self.components),
        }
    }
}

impl FromStr for Vector3d {
    type Err = Box<dyn std::error::Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let components: Vec<i32> = line
            .split(",")
            .map(|component| i32::from_str(component))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(Vector3d {
            components: components.as_slice().try_into()?,
        })
    }
}

impl Add for Vector3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut components = [0; 3];

        for i in 0..3 {
            components[i] = self.components[i] + rhs.components[i];
        }

        Vector3d { components }
    }
}

impl Sub for Vector3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut components = [0; 3];

        for i in 0..3 {
            components[i] = self.components[i] - rhs.components[i];
        }

        Vector3d { components }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rotation::ORIENTATIONS;

    #[test]
    fn test_from_string() {
        assert_eq!(
            Vector3d {
                components: [404, -588, -901]
            },
            Vector3d::from_str("404,-588,-901").unwrap()
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vector3d {
                components: [5, 7, 9]
            },
            Vector3d {
                components: [1, 2, 3]
            } + Vector3d {
                components: [4, 5, 6]
            }
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vector3d {
                components: [-1, -2, -3]
            },
            Vector3d {
                components: [1, 2, 3]
            } - Vector3d {
                components: [2, 4, 6]
            }
        );
    }

    #[test]
    fn test_rotate() {
        assert_eq!(
            Vector3d {
                components: [588, 404, -901]
            },
            Vector3d {
                components: [404, -588, -901]
            }
            .rotate(&ORIENTATIONS[1])
        );
    }
}
