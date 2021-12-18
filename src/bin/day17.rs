use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{env, error};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let target_area = TargetArea::from_str(std::fs::read_to_string(path)?.as_str()).unwrap();

        println!("Max height: {}", target_area.max_height());
        println!(
            "Distinct trajectories: {}",
            target_area.distinct_trajectories().len()
        );

        Ok(())
    } else {
        Err("Usage: day17 INPUT_FILE_PATH".into())
    }
}

// OH BOY MATH I KNOW MATH. Okay! So for a target area, we can get a minimum x velocity that will
// ever reach the target regardless of the y velocity. The farthest a projectile will ever go with
// an initial x velocity X = \sum{n = 1}{X} n, and the closed-form representation of that is
// X * (X + 1) / 2, or (X^2 + X) / 2. That means, for a given target position H, we need
// X^2 + X >= 2H. We COULD use the quadratic equation there, but it's probably actually faster to
// just iterate using integer math.
//
// From there, we can check all of possible x velocities up to the point where a single step would
// put us over the edge of the target zone (the maximum possible velocity) and, for every possible
// velocity, figure out if we actually have a step that puts us in the target x range.
//
// But! The first part of this problem (and I write this having no idea what's coming in the second
// part) is to find the maximum y-position we can reach with any valid trajectory.
//
// One really important observation: at some point on its trajectory, any projectile with an
// initially-positive y velocity will pass through y = 0, with the negative of its initial velocity,
// plus 1. In other words, if we start with an initial velocity of (X, Y), some time later, that
// projectile will pass through the point (?, 0) with a velocity of (?, -(Y + 1)).
//
// We can calculate the max height of the arc using the same sum-of-series equation we use to find
// min-x. We then have two constraints to satisfy: does the trajectory intersect with the target
// area at all, and if it does, is there an x-velocity that gets us there in the right number of
// steps?
//
// ----
//
// And now for some morning-after thoughts. What if we DID do the quadratic equation thing to get
// min_x_velocity? As a reminder, we're interested in X^2 + X >= 2H, or, rearranging a bit:
// X^2 + X - 2H >= 0. Applying the quadratic equation, we have: (-1 ± sqrt(1 + 8H)) / 2. We can rule
// out the negative velocities, since they don't make sense for our use case.

type Trajectory = (i32, i32);

#[derive(Debug, Eq, PartialEq)]
struct TargetArea {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl TargetArea {
    pub fn max_height(&self) -> i32 {
        let max_y_velocity = self
            .possible_trajectories()
            .iter()
            .filter(|trajectory| self.intersects(trajectory))
            .map(|trajectory| trajectory.1)
            .max()
            .unwrap();

        max_y_velocity * (max_y_velocity + 1) / 2
    }

    pub fn distinct_trajectories(&self) -> HashSet<Trajectory> {
        self.possible_trajectories()
            .into_iter()
            .filter(|trajectory| self.intersects(trajectory))
            .collect()
    }

    fn possible_trajectories(&self) -> Vec<Trajectory> {
        let mut possible_trajectories = Vec::new();

        for x_velocity in self.possible_x_velocities() {
            for y_velocity in self.possible_y_velocities() {
                possible_trajectories.push((x_velocity, y_velocity));
            }
        }

        possible_trajectories
    }

    fn possible_x_velocities(&self) -> Vec<i32> {
        let v_x_min =
            ((((1 + (8 * self.x_range.start())) as f64).sqrt() - 1f64) / 2f64).ceil() as i32;

        let mut possible_x_velocities = Vec::new();

        for v_x in v_x_min..=*self.x_range.end() {
            let first_t_in_target =
                Self::time_to_reach_position(v_x, *self.x_range.start()).ceil() as i32;

            // TODO We'll want this later
            let _last_t_in_target = if Self::max_x_distance(v_x) > *self.x_range.end() {
                Self::time_to_reach_position(v_x, *self.x_range.end()).floor() as i32
            } else {
                i32::MAX
            };

            if Self::x_position_after_time(v_x, first_t_in_target) <= *self.x_range.end() {
                possible_x_velocities.push(v_x);
            }
        }

        possible_x_velocities
    }

    fn max_x_distance(initial_velocity: i32) -> i32 {
        initial_velocity * (initial_velocity + 1) / 2
    }

    fn x_position_after_time(initial_velocity: i32, time: i32) -> i32 {
        Self::max_x_distance(initial_velocity) - Self::max_x_distance(initial_velocity - time)
    }

    fn time_to_reach_position(initial_velocity: i32, position: i32) -> f64 {
        (((2 * initial_velocity - 1) as f64
            - (((2 * initial_velocity - 1) * (2 * initial_velocity - 1)
                + 8 * (initial_velocity - position)) as f64)
                .sqrt())
            / 2f64)
            + 1f64
    }

    fn possible_y_velocities(&self) -> Vec<i32> {
        let mut possible_y_velocities = Vec::new();
        let mut initial_velocity = *self.y_range.start() - 1;

        while -initial_velocity >= *self.y_range.start() {
            initial_velocity += 1;

            let mut velocity = initial_velocity;
            let mut position = 0;

            while position >= *self.y_range.start() {
                position += velocity;

                if self.y_range.contains(&position) {
                    possible_y_velocities.push(initial_velocity);
                    break;
                }

                velocity -= 1;
            }
        }

        possible_y_velocities
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        self.x_range.contains(&x) && self.y_range.contains(&y)
    }

    fn intersects(&self, trajectory: &Trajectory) -> bool {
        let (mut x_velocity, mut y_velocity) = trajectory;

        let mut x = 0;
        let mut y = 0;

        while x <= *self.x_range.end() && y >= *self.y_range.start() {
            if self.contains(x, y) {
                return true;
            }

            x += x_velocity;
            y += y_velocity;

            if x_velocity < 0 {
                x_velocity += 1;
            } else if x_velocity > 0 {
                x_velocity -= 1;
            }

            y_velocity -= 1;
        }

        false
    }
}

impl FromStr for TargetArea {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let coordinates = string.strip_prefix("target area: ").unwrap();
        let mut ranges = coordinates.split(", ");

        let mut x_range = ranges
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .split("..");
        let x_min = i32::from_str(x_range.next().unwrap()).unwrap();
        let x_max = i32::from_str(x_range.next().unwrap()).unwrap();

        let mut y_range = ranges
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .split("..");
        let y_min = i32::from_str(y_range.next().unwrap()).unwrap();
        let y_max = i32::from_str(y_range.next().unwrap()).unwrap();

        Ok(TargetArea {
            x_range: x_min..=x_max,
            y_range: y_min..=y_max,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_target_area_from_string() {
        let expected = TargetArea {
            x_range: 20..=30,
            y_range: -10..=-5,
        };

        assert_eq!(
            expected,
            TargetArea::from_str("target area: x=20..30, y=-10..-5").unwrap()
        );
    }

    #[test]
    fn test_max_height() {
        let target_area = TargetArea {
            x_range: 20..=30,
            y_range: -10..=-5,
        };

        assert_eq!(45, target_area.max_height());
    }

    #[test]
    fn test_distinct_trajectories() {
        let target_area = TargetArea {
            x_range: 20..=30,
            y_range: -10..=-5,
        };

        assert_eq!(112, target_area.distinct_trajectories().len());
    }

    #[test]
    fn test_time_to_reach_position() {
        assert_eq!(1, TargetArea::time_to_reach_position(6, 6).ceil() as u32);
        assert_eq!(2, TargetArea::time_to_reach_position(6, 10).ceil() as u32);
        assert_eq!(5, TargetArea::time_to_reach_position(6, 20).ceil() as u32);
    }

    #[test]
    fn test_x_position_after_time() {
        assert_eq!(0, TargetArea::x_position_after_time(6, 0));
        assert_eq!(6, TargetArea::x_position_after_time(6, 1));
        assert_eq!(11, TargetArea::x_position_after_time(6, 2));
    }
}
