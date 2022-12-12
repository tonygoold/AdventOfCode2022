use std::collections::VecDeque;
use std::str::FromStr;

use crate::algo::lcm;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InvalidNumber,
    InvalidOperation,
    MonkeyMatchFailed,
    StartingMatchFailed,
    OperationMatchFailed,
    TestMatchFailed,
    BranchMatchFailed,
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Old,
    Num(usize),
}

impl Operand {
    pub fn eval(&self, old: usize) -> usize {
        match self {
            Self::Old => old,
            Self::Num(n) => *n,
        }
    }
}

impl FromStr for Operand {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Self::Old)
        } else {
            let n: usize = s.parse().map_err(|_| Self::Err::InvalidNumber)?;
            Ok(Self::Num(n))
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add(Operand, Operand),
    Mult(Operand, Operand),
}

impl Operation {
    pub fn apply(&self, old: usize) -> usize {
        match self {
            Self::Add(op1, op2) => op1.eval(old) + op2.eval(old),
            Self::Mult(op1, op2) => op1.eval(old) * op2.eval(old),
        }
    }
}

impl FromStr for Operation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Operation: new = (old|\d+) (\+|\*) (old|\d+)")
            .expect("Unable to build regex");
        let caps = re.captures(s).ok_or(Self::Err::OperationMatchFailed)?;
        let op1 = caps[1].parse()?;
        let op2 = caps[3].parse()?;
        match &caps[2] {
            "+" => Ok(Self::Add(op1, op2)),
            "*" => Ok(Self::Mult(op1, op2)),
            _ => Err(Self::Err::InvalidOperation),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Decision {
    div_by: usize,
    if_true: usize,
    if_false: usize,
}

impl Decision {
    pub fn decide(&self, item: usize) -> usize {
        if item % self.div_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl FromStr for Decision {
    type Err = ParseError;

    // This should be three lines of text
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\n');
        let test_str = parts.next().ok_or(Self::Err::TestMatchFailed)?;
        let true_str = parts.next().ok_or(Self::Err::BranchMatchFailed)?;
        let false_str = parts.next().ok_or(Self::Err::BranchMatchFailed)?;
        let test_re = Regex::new(r"Test: divisible by (\d+)")
            .expect("Unable to build regex");
        let branch_re = Regex::new(r"If (true|false): throw to monkey (\d+)")
            .expect("Unable to build regex");
        let test_caps = test_re.captures(test_str).ok_or(Self::Err::TestMatchFailed)?;
        let div_by: usize = test_caps[1].parse().map_err(|_| Self::Err::InvalidNumber)?;
        let true_caps = branch_re.captures(true_str).ok_or(Self::Err::BranchMatchFailed)?;
        if &true_caps[1] != "true" {
            return Err(Self::Err::BranchMatchFailed);
        }
        let if_true: usize = true_caps[2].parse().map_err(|_| Self::Err::InvalidNumber)?;
        let false_caps = branch_re.captures(false_str).ok_or(Self::Err::BranchMatchFailed)?;
        if &false_caps[1] != "false" {
            return Err(Self::Err::BranchMatchFailed);
        }
        let if_false: usize = false_caps[2].parse().map_err(|_| Self::Err::InvalidNumber)?;
        Ok(Decision { div_by, if_true, if_false })
    }
}

#[derive(Copy, Clone)]
pub struct Transfer {
    pub to: usize,
    pub item: usize,
}

pub struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    decision: Decision,
    inspections: usize,
    reduces: bool,
    lcm: usize,
}

impl Monkey {
    pub fn new(items: VecDeque<usize>, op: Operation, decision: Decision, reduces: bool) -> Self {
        Monkey {
            items,
            op,
            decision,
            inspections: 0,
            reduces,
            lcm: 0,
        }
    }

    pub fn add_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    pub fn inspect(&mut self) -> Option<Transfer> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;
        item = self.op.apply(item);
        if self.lcm != 0 {
            item %= self.lcm;
        }
        if self.reduces {
            item /= 3;
        }
        let to = self.decision.decide(item);
        Some(Transfer { to, item })
    }

    pub fn inspections(&self) -> usize {
        self.inspections
    }
}

impl FromStr for Monkey {
    type Err = ParseError;

    // This should be six lines of text
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('\n');
        let monkey_str = parts.next().ok_or(Self::Err::MonkeyMatchFailed)?;
        let monkey_re = Regex::new(r"Monkey (\d+):")
            .expect("Failed to build regex");
        if !monkey_re.is_match(monkey_str) {
            return Err(Self::Err::MonkeyMatchFailed);
        }

        let starting_str = parts.next().ok_or(Self::Err::StartingMatchFailed)?;
        let starting_re = Regex::new(r"Starting items: (.+)")
            .expect("Failed to build regex");
        let starting_caps = starting_re.captures(starting_str).ok_or(Self::Err::StartingMatchFailed)?;
        let values = starting_caps[1].split(", ");
        let items = values
            .map(|cap| cap.parse::<usize>().map_err(|_| Self::Err::InvalidNumber))
            .collect::<Result<VecDeque<_>, _>>()?;

        let op_str = parts.next().ok_or(Self::Err::OperationMatchFailed)?;
        let op: Operation = op_str.parse()?;

        let decision_str = parts.collect::<Vec<_>>().join("\n");
        let decision: Decision = decision_str.parse()?;

        Ok(Self::new(items, op, decision, true))
    }
}

pub struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    pub fn new(monkeys: Vec<Monkey>) -> Self {
        Self { monkeys }
    }

    pub fn do_round(&mut self) {
        (0..self.monkeys.len()).for_each(|i| {
            while let Some(transfer) = self.monkeys[i].inspect() {
                self.monkeys[transfer.to].add_item(transfer.item);
            }
        });
    }

    pub fn inspections(&self) -> Vec<usize> {
        self.monkeys.iter().map(|monkey| monkey.inspections()).collect()
    }

    pub fn set_reduces(&mut self, reduces: bool) {
        for monkey in self.monkeys.iter_mut() {
            monkey.reduces = reduces;
        }
    }
}

impl FromStr for Monkeys {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let mut monkeys: Vec<Monkey> = Vec::new();
        loop {
            let group: Vec<_> = (&mut lines).take(7).collect();
            if group.len() < 6 {
                break;
            }
            let monkey: Monkey = group.join("\n").parse()?;
            monkeys.push(monkey);
        }
        let lcm_vals = monkeys.iter().map(|m| m.decision.div_by);
        let lcm = lcm_vals.reduce(lcm).expect("Not enough monkeys");
        for monkey in monkeys.iter_mut() {
            monkey.lcm = lcm;
        }
        Ok(Self::new(monkeys))
    }
}
