fn main() {
    if let Some(largest_valid_model_number) = largest_valid_model_number() {
        println!(
            "Largest valid model number: {:?}",
            largest_valid_model_number
        );
    }

    if let Some(smallest_valid_model_number) = smallest_valid_model_number() {
        println!(
            "Smallest valid model number: {:?}",
            smallest_valid_model_number
        );
    }
}

fn largest_valid_model_number() -> Option<u64> {
    explore_model_number(0, &[], Direction::Descending).map(|digits| {
        digits
            .iter()
            .fold(0, |model_number, digit| (model_number * 10) + *digit as u64)
    })
}

fn smallest_valid_model_number() -> Option<u64> {
    explore_model_number(0, &[], Direction::Ascending).map(|digits| {
        digits
            .iter()
            .fold(0, |model_number, digit| (model_number * 10) + *digit as u64)
    })
}

fn explore_model_number(
    chunk: usize,
    preceding_digits: &[i64],
    direction: Direction,
) -> Option<[i64; 14]> {
    const CHUNK_WIDTHS: [usize; 7] = [4, 3, 1, 1, 3, 1, 1];

    let digits = Digits::new(CHUNK_WIDTHS[chunk], direction);

    for next_digits in digits {
        let mut candidate_digits = Vec::from(preceding_digits);
        candidate_digits.extend(next_digits);

        let (checksum, shifted_left_on_last_digit) = checksum(&candidate_digits);

        if candidate_digits.len() == 14 {
            if checksum == 0 {
                return Some(candidate_digits.try_into().unwrap());
            }
        } else {
            if !shifted_left_on_last_digit {
                if let Some(valid_model_number) =
                    explore_model_number(chunk + 1, &candidate_digits, direction)
                {
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

        Digits {
            len,
            direction,
            current,
            max,
        }
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
    use self::Instruction::*;
    use self::Register::*;
    use self::Value::*;
    use super::*;
    use std::error;
    use std::str::FromStr;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Register {
        W,
        X,
        Y,
        Z,
    }

    impl Register {
        #[inline]
        pub fn index(&self) -> usize {
            match self {
                W => 0,
                X => 1,
                Y => 2,
                Z => 3,
            }
        }
    }

    impl FromStr for Register {
        type Err = Box<dyn error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "w" => Ok(Register::W),
                "x" => Ok(Register::X),
                "y" => Ok(Register::Y),
                "z" => Ok(Register::Z),
                _ => Err(format!("Not a valid register: {}", s).into()),
            }
        }
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Value {
        Literal(i64),
        Register(Register),
    }

    impl Value {
        #[inline]
        pub fn eval(&self, registers: &[i64; 4]) -> i64 {
            match self {
                Literal(literal) => *literal,
                Register(register) => registers[register.index()],
            }
        }
    }

    impl FromStr for Value {
        type Err = Box<dyn error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Ok(register) = Register::from_str(s) {
                Ok(Register(register))
            } else if let Ok(literal) = i64::from_str(s) {
                Ok(Literal(literal))
            } else {
                Err(format!("Not a valid value: {}", s).into())
            }
        }
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Instruction {
        Input(Register),
        Add(Register, Value),
        Multiply(Register, Value),
        Divide(Register, Value),
        Modulo(Register, Value),
        Compare(Register, Value),
    }

    impl FromStr for Instruction {
        type Err = Box<dyn error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut components = s.split_whitespace();

            let instruction: Result<&str, &str> = components.next().ok_or("No instruction".into());

            let register: Result<Result<Register, Box<dyn std::error::Error>>, &str> = components
                .next()
                .ok_or("No register".into())
                .map(|r| Register::from_str(r));

            let value: Result<Result<Value, Box<dyn std::error::Error>>, &str> = components
                .next()
                .ok_or("No value".into())
                .map(|v| Value::from_str(v));

            match instruction? {
                "inp" => Ok(Input(register??)),
                "add" => Ok(Add(register??, value??)),
                "mul" => Ok(Multiply(register??, value??)),
                "div" => Ok(Divide(register??, value??)),
                "mod" => Ok(Modulo(register??, value??)),
                "eql" => Ok(Compare(register??, value??)),
                _ => Err(format!("Unrecognized instruction: {}", instruction?).into()),
            }
        }
    }

    struct ArithmeticLogicUnit {
        instructions: Vec<Instruction>,
    }

    impl ArithmeticLogicUnit {
        pub fn execute(&self, inputs: &[i64]) -> [i64; 4] {
            let mut registers = [0; 4];
            let mut read_index = 0;

            for instruction in &self.instructions {
                match instruction {
                    Instruction::Input(register) => {
                        registers[register.index()] = inputs[read_index];
                        read_index += 1;
                    }
                    Instruction::Add(register, value) => {
                        registers[register.index()] =
                            registers[register.index()] + value.eval(&registers);
                    }
                    Instruction::Multiply(register, value) => {
                        registers[register.index()] =
                            registers[register.index()] * value.eval(&registers);
                    }
                    Instruction::Divide(register, value) => {
                        registers[register.index()] =
                            registers[register.index()] / value.eval(&registers);
                    }
                    Instruction::Modulo(register, value) => {
                        registers[register.index()] =
                            registers[register.index()] % value.eval(&registers);
                    }
                    Instruction::Compare(register, value) => {
                        registers[register.index()] =
                            if registers[register.index()] == value.eval(&registers) {
                                1
                            } else {
                                0
                            }
                    }
                }
            }

            registers
        }
    }

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

    #[test]
    fn test_instruction_from_string() {
        assert_eq!(Input(W), Instruction::from_str("inp w").unwrap());
        assert_eq!(Add(X, Literal(4)), Instruction::from_str("add x 4").unwrap());
        assert_eq!(Multiply(Y, Register(Z)), Instruction::from_str("mul y z").unwrap());
        assert_eq!(Divide(Z, Literal(-7)), Instruction::from_str("div z -7").unwrap());
        assert_eq!(Modulo(W, Register(X)), Instruction::from_str("mod w x").unwrap());
        assert_eq!(Compare(X, Register(Y)), Instruction::from_str("eql x y").unwrap());
    }

    #[test]
    fn test_execute() {
        {
            let alu = ArithmeticLogicUnit {
                instructions: vec![Input(X), Multiply(X, Literal(-1))],
            };

            assert_eq!([0, -4, 0, 0], alu.execute(&[4]));
        }

        {
            let alu = ArithmeticLogicUnit {
                instructions: vec![
                    Input(Z),
                    Input(X),
                    Multiply(Z, Literal(3)),
                    Compare(Z, Register(X)),
                ],
            };

            assert_eq!([0, 12, 0, 1], alu.execute(&[4, 12]));
            assert_eq!([0, 13, 0, 0], alu.execute(&[4, 13]));
        }

        {
            let alu = ArithmeticLogicUnit {
                instructions: vec![
                    Input(W),
                    Add(Z, Register(W)),
                    Modulo(Z, Literal(2)),
                    Divide(W, Literal(2)),
                    Add(Y, Register(W)),
                    Modulo(Y, Literal(2)),
                    Divide(W, Literal(2)),
                    Add(X, Register(W)),
                    Modulo(X, Literal(2)),
                    Divide(W, Literal(2)),
                    Modulo(W, Literal(2)),
                ],
            };

            assert_eq!([1, 0, 1, 0], alu.execute(&[0b1010]));
            assert_eq!([0, 1, 0, 1], alu.execute(&[0b0101]));
        }
    }

    #[test]
    fn test_checksum() {
        const TEST_INSTRUCTIONS: &str = include_str!("instructions.txt");

        let alu = {
            let instructions = TEST_INSTRUCTIONS
                .lines()
                .map(Instruction::from_str)
                .collect::<Result<Vec<Instruction>, Box<dyn error::Error>>>()
                .unwrap();

            ArithmeticLogicUnit { instructions }
        };

        for inputs in [
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5],
            [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9],
            [9, 8, 7, 6, 5, 4, 3, 2, 1, 9, 8, 7, 6, 5],
        ] {
            let (checksum, _) = checksum(&inputs);
            assert_eq!(alu.execute(&inputs)[3], checksum);
        }
    }
}
