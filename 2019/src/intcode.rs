use std::collections::VecDeque;
use std::iter;
use thiserror::Error;

pub type Result<T, E = IntcodeError> = ::std::result::Result<T, E>;

#[derive(Clone)]
pub struct IntcodeComputer {
    tape: Vec<i32>,
    input: VecDeque<i32>,
    output: Vec<i32>,
    ip: usize,
    offset: usize,
    status: bool,
}

impl IntcodeComputer {
    pub fn new(tape: &[i32]) -> Self {
        Self {
            tape: tape.to_owned(),
            input: VecDeque::new(),
            output: Vec::new(),
            ip: 0,
            offset: 0,
            status: true,
        }
    }
}

impl IntcodeComputer {
    fn fetch(&mut self) -> Result<i32> {
        let next = self.memread(self.ip + self.offset)?;
        self.offset += 1;
        Ok(next)
    }

    fn fetch_operand(&mut self, param_mode: ParameterMode) -> Result<Operand> {
        let param = self.fetch()?;
        let operand = match param_mode {
            ParameterMode::Position => Operand::Position(param as usize),
            ParameterMode::Immediate => Operand::Immediate(param),
        };
        Ok(operand)
    }

    fn memread(&self, address: usize) -> Result<i32> {
        self.tape
            .get(address)
            .copied()
            .ok_or(IntcodeError::AddressOutOfBound(address))
    }

    fn memwrite(&mut self, address: usize, value: i32) -> Result<()> {
        *self
            .tape
            .get_mut(address)
            .ok_or(IntcodeError::AddressOutOfBound(address))? = value;
        Ok(())
    }

    fn read_operand(&self, operand: Operand) -> Result<i32> {
        let operand = match operand {
            Operand::Position(addr) => self.memread(addr)?,
            Operand::Immediate(operand) => operand,
        };
        Ok(operand)
    }

    fn decode(i: i32) -> Result<(Opcode, ParameterMode, ParameterMode, ParameterMode)> {
        if i >= 100_000 {
            return Err(IntcodeError::InvalidInstruction(i));
        }

        let opcode = match i % 100 {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => return Err(IntcodeError::InvalidInstruction(i)),
        };
        let mut operand_types = [ParameterMode::Position; 3];

        for (idx, op_type) in iter::successors(Some(i / 100), |n| Some(n / 10))
            .take(3)
            .map(|n| match n % 10 {
                0 => Ok(ParameterMode::Position),
                1 => Ok(ParameterMode::Immediate),
                _ => Err(IntcodeError::InvalidInstruction(i)),
            })
            .enumerate()
        {
            operand_types[idx] = op_type?;
        }

        Ok((opcode, operand_types[0], operand_types[1], operand_types[2]))
    }

    fn fetch_and_decode(&mut self) -> Result<Instruction> {
        use ParameterMode::*;

        let inst = self.fetch()?;
        let instruction = match Self::decode(inst)? {
            (Opcode::Add, p1, p2, Position) => Instruction::Add(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch()? as usize,
            ),
            (Opcode::Mul, p1, p2, Position) => Instruction::Mul(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch()? as usize,
            ),
            (Opcode::Input, Position, Position, Position) => {
                Instruction::Input(self.fetch()? as usize)
            }
            (Opcode::Output, p1, Position, Position) => {
                Instruction::Output(self.fetch_operand(p1)?)
            }
            (Opcode::JumpIfTrue, p1, p2, Position) => {
                Instruction::JumpIfTrue(self.fetch_operand(p1)?, self.fetch_operand(p2)?)
            }
            (Opcode::JumpIfFalse, p1, p2, Position) => {
                Instruction::JumpIfFalse(self.fetch_operand(p1)?, self.fetch_operand(p2)?)
            }
            (Opcode::LessThan, p1, p2, Position) => Instruction::LessThan(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch()? as usize,
            ),
            (Opcode::Equals, p1, p2, Position) => Instruction::Equals(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch()? as usize,
            ),
            (Opcode::Halt, Position, Position, Position) => Instruction::Halt,
            (_, _, _, _) => return Err(IntcodeError::InvalidInstruction(inst)),
        };

        Ok(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<()> {
        use Instruction::*;

        match instruction {
            Add(rs, rt, addr) => {
                self.memwrite(addr, self.read_operand(rs)? + self.read_operand(rt)?)?
            }
            Mul(rs, rt, addr) => {
                self.memwrite(addr, self.read_operand(rs)? * self.read_operand(rt)?)?
            }
            Input(addr) => {
                let input = self
                    .input
                    .pop_front()
                    .ok_or(IntcodeError::WaitingForInput)?;
                self.memwrite(addr, input)?;
            }
            Output(rs) => self.output.push(self.read_operand(rs)?),
            JumpIfTrue(rs, rt) => {
                if self.read_operand(rs)? != 0 {
                    self.ip = self.read_operand(rt)? as usize;
                    self.offset = 0;
                }
            }
            JumpIfFalse(rs, rt) => {
                if self.read_operand(rs)? == 0 {
                    self.ip = self.read_operand(rt)? as usize;
                    self.offset = 0;
                }
            }
            LessThan(rs, rt, addr) => self.memwrite(
                addr,
                (self.read_operand(rs)? < self.read_operand(rt)?).into(),
            )?,
            Equals(rs, rt, addr) => self.memwrite(
                addr,
                (self.read_operand(rs)? == self.read_operand(rt)?).into(),
            )?,
            Halt => self.status = false,
        };

        Ok(())
    }

    fn commit(&mut self) {
        self.ip += self.offset;
        self.offset = 0;
    }

    pub fn run(&mut self) -> Result<()> {
        self.offset = 0;

        while self.status {
            let instruction = self.fetch_and_decode()?;
            self.execute(instruction)?;
            self.commit();
        }
        Ok(())
    }

    pub fn input(&mut self, val: i32) {
        self.input.push_back(val)
    }

    pub fn output(&self) -> &[i32] {
        &self.output
    }

    pub fn reset(&mut self, program: &[i32]) {
        self.tape.clear();
        self.tape.extend_from_slice(program);
        self.ip = 0;
        self.input.clear();
        self.output.clear();
        self.status = true;
    }

    pub fn status(&self) -> bool {
        self.status
    }
}

#[derive(Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

enum Instruction {
    Add(Operand, Operand, usize),
    Mul(Operand, Operand, usize),
    Input(usize),
    Output(Operand),
    JumpIfTrue(Operand, Operand),
    JumpIfFalse(Operand, Operand),
    LessThan(Operand, Operand, usize),
    Equals(Operand, Operand, usize),
    Halt,
}

#[derive(Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
}

enum Operand {
    Position(usize),
    Immediate(i32),
}

#[derive(Debug, Error)]
pub enum IntcodeError {
    #[error("address out of bounds: {0}")]
    AddressOutOfBound(usize),
    #[error("invalid instruction '{0}'")]
    InvalidInstruction(i32),
    #[error("waiting for input")]
    WaitingForInput,
}
