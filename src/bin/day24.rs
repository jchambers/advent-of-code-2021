use self::Register::*;
use self::Value::*;
use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _lines = io::BufReader::new(File::open(path)?).lines();

        Ok(())
    } else {
        Err("Usage: day24 INPUT_FILE_PATH".into())
    }
}

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

enum Instruction {
    Input(Register),
    Add(Register, Value),
    Multiply(Register, Value),
    Divide(Register, Value),
    Modulo(Register, Value),
    Compare(Register, Value),
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

#[cfg(test)]
mod test {
    use super::Instruction::*;
    use super::Register::*;
    use super::*;
    use crate::Value::{Literal, Register};

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
}
