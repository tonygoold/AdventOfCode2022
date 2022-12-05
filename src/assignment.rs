use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    MissingComma, MissingDash, NotANumber
}

pub struct Pair(RangeInclusive<usize>, RangeInclusive<usize>);

impl Pair {
    pub fn fully_overlaps(&self) -> bool {
        (self.0.contains(self.1.start()) && self.0.contains(self.1.end())) ||
        (self.1.contains(self.0.start()) && self.1.contains(self.0.end()))
    }

    pub fn overlaps(&self) -> bool {
        self.0.contains(self.1.start()) || self.0.contains(self.1.end()) ||
        self.1.contains(self.0.start()) || self.1.contains(self.0.end())
    }
}

impl FromStr for Pair {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let s1 = parts.next().ok_or(ParseError::MissingComma)?;
        let s2 = parts.next().ok_or(ParseError::MissingComma)?;
        let mut parts1 = s1.split('-');
        let r11: usize = parts1.next().ok_or(ParseError::MissingDash)?
            .parse().map_err(|_| ParseError::NotANumber)?;
        let r12: usize = parts1.next().ok_or(ParseError::MissingDash)?
            .parse().map_err(|_| ParseError::NotANumber)?;
        let mut parts2 = s2.split('-');
        let r21: usize = parts2.next().ok_or(ParseError::MissingDash)?
            .parse().map_err(|_| ParseError::NotANumber)?;
        let r22: usize = parts2.next().ok_or(ParseError::MissingDash)?
            .parse().map_err(|_| ParseError::NotANumber)?;
        Ok(Pair(r11..=r12, r21..=r22))
    }
}
