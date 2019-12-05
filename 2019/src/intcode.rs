use std::iter;
use thiserror::Error;

pub type Result<T, E = IntcodeError> = ::std::result::Result<T, E>;

pub struct IntcodeComputer<'a> {
    tape: &'a mut [i32],
    ip: usize,
    status: bool,
}

impl<'a> IntcodeComputer<'a> {
    pub fn new(tape: &'a mut [i32]) -> Self {
        Self {
            tape,
            ip: 0,
            status: true,
        }
    }
}

impl IntcodeComputer<'_> {
    fn fetch(&mut self) -> Result<i32> {
        let next = self.memread(self.ip)?;
        self.ip += 1;
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
            (Opcode::Halt, Position, Position, Position) => Instruction::Halt,
            (_, _, _, _) => return Err(IntcodeError::InvalidInstruction(inst)),
        };

        Ok(instruction)
    }

    fn execute(&mut self, instruction: Instruction) -> Result<()> {
        use Instruction::*;

        match instruction {
            Add(rs, rt, rd) => {
                self.memwrite(rd, self.read_operand(rs)? + self.read_operand(rt)?)?
            }
            Mul(rs, rt, rd) => {
                self.memwrite(rd, self.read_operand(rs)? * self.read_operand(rt)?)?
            }
            Halt => self.status = false,
        };

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while self.status {
            let instruction = self.fetch_and_decode()?;
            self.execute(instruction)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    Halt,
}

enum Instruction {
    Add(Operand, Operand, usize),
    Mul(Operand, Operand, usize),
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
}
