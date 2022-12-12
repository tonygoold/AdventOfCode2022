use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InvalidMovement,
}

pub struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl Movement {
    pub fn new(count: usize, from: usize, to: usize) -> Self {
        Movement { count, from, to }
    }
}

impl FromStr for Movement {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)")
            .expect("failed to build regex");
        let captures = re.captures(s).ok_or(ParseError::InvalidMovement)?;
        let count: usize = captures[1].parse().map_err(|_| ParseError::InvalidMovement)?;
        let from: usize = captures[2].parse().map_err(|_| ParseError::InvalidMovement)?;
        let to: usize = captures[3].parse().map_err(|_| ParseError::InvalidMovement)?;
        Ok(Movement { count, from, to })
    }
}

pub struct Stack {
    crates: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { crates: Vec::new() }
    }

    pub fn pop(&mut self) -> char {
        self.crates.pop().expect("cannot pop an empty stack")
    }

    pub fn push(&mut self, c: char) {
        self.crates.push(c);
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    pub fn new(size: usize) -> Self {
        let mut stacks = Vec::new();
        stacks.resize_with(size, Default::default);
        Stacks { stacks }
    }

    pub fn apply(&mut self, movement: &Movement) {
        for _ in 0..movement.count {
            let c = self.stacks[movement.from - 1].pop();
            self.stacks[movement.to - 1].push(c);
        }
    }

    pub fn apply_keeping_order(&mut self, movement: &Movement) {
        let mut cs = Vec::new();
        for _ in 0..movement.count {
            cs.push(self.stacks[movement.from - 1].pop());
        }
        while let Some(c) = cs.pop() {
            self.stacks[movement.to - 1].push(c);
        }
    }

    pub fn peek_all(&self) -> String {
        let mut s = String::new();
        for stack in self.stacks.iter() {
            if let Some(c) = stack.crates.last() {
                s.push(*c)
            }
        }
        s
    }

    pub fn populate(&mut self, s: &str) {
        let re = regex::Regex::new(r"(?:\[(.)\]|   ) ?")
            .expect("failed to build regex");
        let mut captures = re.captures_iter(s);
        for stack in self.stacks.iter_mut() {
            match captures.next() {
                Some(caps) => {
                    if let Some(cs) = caps.get(1) {
                        stack.push(cs.as_str().chars().next().expect("this should be impossible"));
                    }
                }
                None => break,
            };
        }
    }
}
