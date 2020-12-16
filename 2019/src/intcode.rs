use std::collections::VecDeque;
use std::iter;
use thiserror::Error;

pub type Result<T, E = IntcodeError> = ::std::result::Result<T, E>;

const MEMORY_SIZE: usize = 1 << 16;
#[derive(Clone)]
pub struct IntcodeComputer {
    tape: [i64; MEMORY_SIZE],
    input: VecDeque<i64>,
    output: Vec<i64>,
    ip: usize,
    offset: usize,
    relative_base: i64,
    status: bool,
}

impl IntcodeComputer {
    pub fn new(program: &[i64]) -> Self {
        if program.len() > MEMORY_SIZE {
            panic!("program too big");
        }

        let mut tape = [0; MEMORY_SIZE];
        tape[..program.len()].copy_from_slice(program);

        Self {
            tape,
            input: VecDeque::new(),
            output: Vec::new(),
            ip: 0,
            offset: 0,
            relative_base: 0,
            status: true,
        }
    }
}

impl IntcodeComputer {
    fn fetch(&mut self) -> Result<i64> {
        let next = self.memread(self.ip + self.offset)?;
        self.offset += 1;
        Ok(next)
    }

    fn fetch_operand(&mut self, param_mode: ParameterMode) -> Result<Operand> {
        let param = self.fetch()?;
        let operand = match param_mode {
            ParameterMode::Position => Operand::Position(param as usize),
            ParameterMode::Immediate => Operand::Immediate(param),
            ParameterMode::Relative => Operand::Relative(param),
        };
        Ok(operand)
    }

    fn memread(&self, address: usize) -> Result<i64> {
        self.tape
            .get(address)
            .copied()
            .ok_or(IntcodeError::AddressOutOfBound(address))
    }

    fn memwrite(&mut self, address: usize, value: i64) -> Result<()> {
        *self
            .tape
            .get_mut(address)
            .ok_or(IntcodeError::AddressOutOfBound(address))? = value;
        Ok(())
    }

    fn read_operand(&self, operand: Operand) -> Result<i64> {
        let operand = match operand {
            Operand::Position(addr) => self.memread(addr)?,
            Operand::Immediate(operand) => operand,
            Operand::Relative(offset) => self.memread((self.relative_base + offset) as usize)?,
        };
        Ok(operand)
    }

    fn write_operand(&mut self, operand: Operand, value: i64) -> Result<()> {
        match operand {
            Operand::Position(addr) => self.memwrite(addr, value)?,
            Operand::Immediate(_) => panic!("Invalid operand"),
            Operand::Relative(offset) => {
                self.memwrite((self.relative_base + offset) as usize, value)?
            }
        };
        Ok(())
    }

    fn decode(i: i64) -> Result<(Opcode, ParameterMode, ParameterMode, ParameterMode)> {
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
            9 => Opcode::AdjustRelativeBase,
            99 => Opcode::Halt,
            _ => return Err(IntcodeError::InvalidInstruction(i)),
        };
        let mut operand_types = [ParameterMode::Position; 3];

        for (idx, op_type) in iter::successors(Some(i / 100), |n| Some(n / 10))
            .take(3)
            .map(|n| match n % 10 {
                0 => Ok(ParameterMode::Position),
                1 => Ok(ParameterMode::Immediate),
                2 => Ok(ParameterMode::Relative),
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
            (Opcode::Add, p1, p2, p3) => Instruction::Add(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch_operand(p3)?,
            ),
            (Opcode::Mul, p1, p2, p3) => Instruction::Mul(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch_operand(p3)?,
            ),
            (Opcode::Input, p1, Position, Position) => Instruction::Input(self.fetch_operand(p1)?),
            (Opcode::Output, p1, Position, Position) => {
                Instruction::Output(self.fetch_operand(p1)?)
            }
            (Opcode::JumpIfTrue, p1, p2, Position) => {
                Instruction::JumpIfTrue(self.fetch_operand(p1)?, self.fetch_operand(p2)?)
            }
            (Opcode::JumpIfFalse, p1, p2, Position) => {
                Instruction::JumpIfFalse(self.fetch_operand(p1)?, self.fetch_operand(p2)?)
            }
            (Opcode::LessThan, p1, p2, p3) => Instruction::LessThan(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch_operand(p3)?,
            ),
            (Opcode::Equals, p1, p2, p3) => Instruction::Equals(
                self.fetch_operand(p1)?,
                self.fetch_operand(p2)?,
                self.fetch_operand(p3)?,
            ),
            (Opcode::AdjustRelativeBase, p1, Position, Position) => {
                Instruction::AdjustRelativeBase(self.fetch_operand(p1)?)
            }
            (Opcode::Halt, Position, Position, Position) => Instruction::Halt,
            (_, _, _, _) => return Err(IntcodeError::InvalidInstruction(inst)),
        };

        Ok(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<()> {
        use Instruction::*;

        match instruction {
            Add(rs, rt, rd) => {
                self.write_operand(rd, self.read_operand(rs)? + self.read_operand(rt)?)?
            }
            Mul(rs, rt, rd) => {
                self.write_operand(rd, self.read_operand(rs)? * self.read_operand(rt)?)?
            }
            Input(rd) => {
                let input = self
                    .input
                    .pop_front()
                    .ok_or(IntcodeError::WaitingForInput)?;
                self.write_operand(rd, input)?;
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
            LessThan(rs, rt, rd) => {
                self.write_operand(rd, (self.read_operand(rs)? < self.read_operand(rt)?).into())?
            }
            Equals(rs, rt, rd) => self.write_operand(
                rd,
                (self.read_operand(rs)? == self.read_operand(rt)?).into(),
            )?,
            AdjustRelativeBase(rs) => self.relative_base += self.read_operand(rs)?,
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

    pub fn input(&mut self, val: i64) {
        self.input.push_back(val)
    }

    pub fn output(&self) -> &[i64] {
        &self.output
    }

    pub fn reset(&mut self, program: &[i64]) {
        let (p, zeros) = self.tape.split_at_mut(program.len());
        p.copy_from_slice(program);
        for i in zeros {
            *i = 0;
        }

        self.ip = 0;
        self.input.clear();
        self.output.clear();
        self.relative_base = 0;
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
    AdjustRelativeBase,
    Halt,
}

#[derive(Debug)]
enum Instruction {
    Add(Operand, Operand, Operand),
    Mul(Operand, Operand, Operand),
    Input(Operand),
    Output(Operand),
    JumpIfTrue(Operand, Operand),
    JumpIfFalse(Operand, Operand),
    LessThan(Operand, Operand, Operand),
    Equals(Operand, Operand, Operand),
    AdjustRelativeBase(Operand),
    Halt,
}

#[derive(Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug)]
enum Operand {
    Position(usize),
    Immediate(i64),
    Relative(i64),
}

#[derive(Debug, Error)]
pub enum IntcodeError {
    #[error("address out of bounds: {0}")]
    AddressOutOfBound(usize),
    #[error("invalid instruction '{0}'")]
    InvalidInstruction(i64),
    #[error("waiting for input")]
    WaitingForInput,
}
