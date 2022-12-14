use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    InvalidCharacter,
    UnbalancedList,
    InvalidNumber,
    EmptyString,
}

pub enum UintNode {
    Leaf(usize),
    Tree(Vec<UintNode>),
}

impl Ord for UintNode {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Leaf(a), Self::Leaf(b)) => a.cmp(b),
            (Self::Leaf(a), Self::Tree(_)) => Self::Tree(vec![Self::Leaf(*a)]).cmp(other),
            (Self::Tree(_), Self::Leaf(b)) => self.cmp(&Self::Tree(vec![Self::Leaf(*b)])),
            (Self::Tree(a), Self::Tree(b)) => {
                for (i, va) in a.iter().enumerate() {
                    if i >= b.len() {
                        return Ordering::Greater;
                    }
                    match va.cmp(&b[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {},
                    }
                }
                // Ran out of items. I don't understand how the case of fully equal
                // is supposed to be handled, but I think this is correct.
                if a.len() == b.len() {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            }
        }
    }
}

impl PartialOrd for UintNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for UintNode {}

impl PartialEq for UintNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl FromStr for UintNode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.char_indices();
        let c = cs.next().ok_or(Self::Err::EmptyString)?;
        if c.1 != '[' {
            let n: usize = s.parse().map_err(|_| Self::Err::InvalidNumber)?;
            return Ok(UintNode::Leaf(n));
        };
        let mut nodes: Vec<UintNode> = Vec::new();
        let mut start: Option<usize> = None;
        let mut depth: usize = 0;
        for c in cs {
            if start.is_none() {
                start = Some(c.0);
            }
            if c.1 == '[' {
                depth += 1;
            } else if c.1 == ']' && depth == 0 {
                let start_idx = start.unwrap();
                let substr = &s[start_idx..c.0];
                if !substr.is_empty() {
                    let node: UintNode = substr.parse()?;
                    nodes.push(node);
                }
                break;
            } else if c.1 == ',' && depth == 0 {
                let start_idx = start.unwrap();
                let node: UintNode = s[start_idx..c.0].parse()?;
                nodes.push(node);
                start = None;
            } else if c.1 == ']' {
                depth -= 1;
            }
        }
        Ok(UintNode::Tree(nodes))
    }
}

