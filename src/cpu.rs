use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum ParseInstructionError {
    InvalidInstruction,
    InvalidParameter,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match *self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let i = parts.next().ok_or(Self::Err::InvalidInstruction)?;
        if i == "noop" {
            return Ok(Instruction::Noop);
        }
        let x = parts.next().ok_or(Self::Err::InvalidParameter)?;
        let x: isize = x.parse().map_err(|_| Self::Err::InvalidParameter)?;
        if i == "addx" {
            return Ok(Instruction::Addx(x));
        }
        Err(Self::Err::InvalidInstruction)
    }
}

pub struct ScheduledInstruction {
    inst: Instruction,
    cycles: usize,
}

impl ScheduledInstruction {
    pub fn new(inst: &Instruction) -> Self {
        let cycles = inst.cycles();
        Self { inst: *inst, cycles }
    }
}

pub struct CPU {
    cycle: usize,
    pipeline: Vec<ScheduledInstruction>,
    x: isize,
}

impl CPU {
    pub fn new(insts: &[Instruction]) -> Self {
        let pipeline = insts.iter().map(ScheduledInstruction::new).collect();
        Self { cycle: 1, pipeline, x: 1 }
    }

    pub fn tick(&mut self) -> bool {
        let inst = match self.pipeline.first_mut() {
            Some(inst) => inst,
            None => return false,
        };
        self.cycle += 1;
        inst.cycles -= 1;
        if inst.cycles > 0 {
            return true;
        }
        let inst = self.pipeline.remove(0);
        match inst.inst {
            Instruction::Noop => {},
            Instruction::Addx(x) => self.x += x,
        };
        true
    }

    pub fn cycle(&self) -> usize {
        self.cycle
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn signal_strength(&self) -> isize {
        self.x * (self.cycle as isize)
    }
}
