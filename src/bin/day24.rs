fn main() {
    if let Some(largest_valid_model_number) = largest_valid_model_number() {
        println!("Largest valid model number: {:?}", largest_valid_model_number);
    }

    if let Some(smallest_valid_model_number) = smallest_valid_model_number() {
        println!("Smallest valid model number: {:?}", smallest_valid_model_number);
    }
}

fn largest_valid_model_number() -> Option<u64> {
    explore_model_number(0, &[], Direction::Descending)
        .map(|digits| {
            digits.iter()
                .fold(0, |model_number, digit| (model_number * 10) + *digit as u64)
        })
}

fn smallest_valid_model_number() -> Option<u64> {
    explore_model_number(0, &[], Direction::Ascending)
        .map(|digits| {
            digits.iter()
                .fold(0, |model_number, digit| (model_number * 10) + *digit as u64)
        })
}

fn explore_model_number(chunk: usize, preceding_digits: &[i64], direction: Direction) -> Option<[i64; 14]> {
    const CHUNK_WIDTHS: [usize; 7] = [4, 3, 1, 1, 3, 1, 1];

    let digits = Digits::new(CHUNK_WIDTHS[chunk], direction);

    for next_digits in digits {
        let mut candidate_digits = Vec::from(preceding_digits);
        candidate_digits.extend(next_digits);

        let (checksum, shifted_left_on_last_digit) = checksum(&candidate_digits);

        if candidate_digits.len() == 14 {
            if checksum == 0 {
                return Some(candidate_digits.try_into().unwrap())
            }
        } else {
            if !shifted_left_on_last_digit {
                if let Some(valid_model_number) = explore_model_number(chunk + 1, &candidate_digits, direction) {
                    return Some(valid_model_number);
                }
            }
        }
    }

    None
}

fn checksum(digits: &[i64]) -> (i64, bool) {
    const A: [i64; 14] = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
    const B: [i64; 14] = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];

    let mut checksum = 0;
    let mut shift_left = true;

    for i in 0..digits.len() {
        shift_left = (checksum % 26) + A[i] != digits[i];
        let shift_right = A[i] < 0;

        if shift_right {
            checksum /= 26;
        }

        if shift_left {
            checksum *= 26;
            checksum += digits[i] + B[i]
        }
    }

    (checksum, shift_left)
}

#[derive(Copy, Clone)]
enum Direction {
    Ascending,
    Descending,
}

struct Digits {
    len: usize,
    direction: Direction,

    current: u64,
    max: u64,
}

impl Digits {
    pub fn new(len: usize, direction: Direction) -> Self {
        let mut max = 1;

        for _ in 0..len {
            max *= 9;
        }

        let current = match direction {
            Direction::Ascending => 0,
            Direction::Descending => max,
        };

        Digits { len, direction, current, max }
    }
}

impl Iterator for Digits {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::Ascending => {
                if self.current >= self.max {
                    return None;
                }
            }
            Direction::Descending => {
                if self.current > 0 {
                    self.current -= 1;
                } else {
                    return None;
                }
            }
        }

        let mut remainder = self.current;
        let mut digits = vec![0; self.len];

        for digit in (0..self.len).rev() {
            digits[digit] = ((remainder % 9) + 1) as i64;
            remainder /= 9;
        }

        if matches!(self.direction, Direction::Ascending) {
            self.current += 1;
        }

        Some(digits)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digits() {
        {
            let mut digits = Digits::new(1, Direction::Descending);

            assert_eq!(Some(vec![9]), digits.next());
            assert_eq!(Some(vec![8]), digits.next());
            assert_eq!(Some(vec![7]), digits.next());
            assert_eq!(Some(vec![6]), digits.next());
            assert_eq!(Some(vec![5]), digits.next());
            assert_eq!(Some(vec![4]), digits.next());
            assert_eq!(Some(vec![3]), digits.next());
            assert_eq!(Some(vec![2]), digits.next());
            assert_eq!(Some(vec![1]), digits.next());
            assert_eq!(None, digits.next());
        }

        {
            let mut digits = Digits::new(3, Direction::Descending);

            assert_eq!(Some(vec![9, 9, 9]), digits.next());
            assert_eq!(Some(vec![9, 9, 8]), digits.next());
            assert_eq!(Some(vec![9, 9, 7]), digits.next());
            assert_eq!(Some(vec![9, 9, 6]), digits.next());
            assert_eq!(Some(vec![9, 9, 5]), digits.next());
            assert_eq!(Some(vec![9, 9, 4]), digits.next());
            assert_eq!(Some(vec![9, 9, 3]), digits.next());
            assert_eq!(Some(vec![9, 9, 2]), digits.next());
            assert_eq!(Some(vec![9, 9, 1]), digits.next());
            assert_eq!(Some(vec![9, 8, 9]), digits.next());
        }

        {
            let mut digits = Digits::new(1, Direction::Ascending);

            assert_eq!(Some(vec![1]), digits.next());
            assert_eq!(Some(vec![2]), digits.next());
            assert_eq!(Some(vec![3]), digits.next());
            assert_eq!(Some(vec![4]), digits.next());
            assert_eq!(Some(vec![5]), digits.next());
            assert_eq!(Some(vec![6]), digits.next());
            assert_eq!(Some(vec![7]), digits.next());
            assert_eq!(Some(vec![8]), digits.next());
            assert_eq!(Some(vec![9]), digits.next());
            assert_eq!(None, digits.next());
        }

        {
            let mut digits = Digits::new(3, Direction::Ascending);

            assert_eq!(Some(vec![1, 1, 1]), digits.next());
            assert_eq!(Some(vec![1, 1, 2]), digits.next());
            assert_eq!(Some(vec![1, 1, 3]), digits.next());
            assert_eq!(Some(vec![1, 1, 4]), digits.next());
            assert_eq!(Some(vec![1, 1, 5]), digits.next());
            assert_eq!(Some(vec![1, 1, 6]), digits.next());
            assert_eq!(Some(vec![1, 1, 7]), digits.next());
            assert_eq!(Some(vec![1, 1, 8]), digits.next());
            assert_eq!(Some(vec![1, 1, 9]), digits.next());
            assert_eq!(Some(vec![1, 2, 1]), digits.next());
        }
    }
}
