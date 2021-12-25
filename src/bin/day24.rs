use self::Register::*;
use self::Value::*;
use self::Instruction::*;
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let instructions: Vec<Instruction> = io::BufReader::new(File::open(path)?).lines()
            .map(|line| Instruction::from_str(line.unwrap().as_str()))
            .collect::<Result<Vec<Instruction>, Box<dyn error::Error>>>()?;

        println!("Largest valid model number: {}", largest_valid_model_number(&instructions).unwrap());

        Ok(())
    } else {
        Err("Usage: day24 INPUT_FILE_PATH".into())
    }
}

fn largest_valid_model_number(instructions: &[Instruction]) -> Option<u64> {
    let alu = ArithmeticLogicUnit::new(instructions);
    let model_numbers = ModelNumbers::new();

    for model_number in model_numbers {
        if alu.execute(&model_number.digits)[3] == 0 {
            return Some(model_number.into());
        }
    }

    None
}

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
    pub fn new(instructions: &[Instruction]) -> Self {
        ArithmeticLogicUnit {
            instructions: instructions.iter().cloned().collect()
        }
    }

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

#[derive(Debug, Eq, PartialEq)]
struct ModelNumber {
    digits: [i64; 14],
}

impl From<ModelNumber> for u64 {
    fn from(model_number: ModelNumber) -> Self {
        model_number.digits.iter()
            .fold(0, |m, digit| (m * 10) + *digit as u64)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct ModelNumbers {
    current: u64,
}

impl ModelNumbers {
    pub fn new() -> Self {
        ModelNumbers {
            current: 205_891_132_094_649 // 9^15
        }
    }
}

impl Iterator for ModelNumbers {
    type Item = ModelNumber;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            self.current -= 1;

            let mut remainder = self.current;
            let mut digits = [0; 14];

            for digit in (0..14).rev() {
                digits[digit] = ((remainder % 9) + 1) as i64;
                remainder /= 9;
            }

            Some(ModelNumber { digits })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Value::{Literal, Register};

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
    fn test_model_number_to_u64() {
        assert_eq!(99999999999999u64, ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]}.into());
    }

    #[test]
    fn test_model_numbers() {
        let mut model_numbers = ModelNumbers::new();

        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 7]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 6]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 5]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 4]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 3]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 2]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 1]}), model_numbers.next());
        assert_eq!(Some(ModelNumber{ digits: [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8, 9]}), model_numbers.next());
    }
}
