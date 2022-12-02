use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    UnrecognizedCharacter,
    InsufficientCharacters,
}

#[derive(Debug, Copy, Clone)]
pub enum Play {
    Rock, Paper, Scissors
}

impl Play {
    pub fn outcome(&self, other: Play) -> Outcome {
        match (*self, other) {
            (Self::Rock, Self::Scissors) | (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper) => Outcome::Win,
            (Self::Rock, Self::Rock) | (Self::Scissors, Self::Scissors) | (Self::Paper, Self::Paper) => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    pub fn score(&self) -> usize {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Play {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(ParseError::UnrecognizedCharacter),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Outcome {
    Win, Draw, Lose
}

impl Outcome {
    pub fn score(&self) -> usize {
        match *self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    pub fn counter_for(&self, play: Play) -> Play {
        match *self {
            Self::Win => match play {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            },
            Self::Draw => match play {
                Play::Rock => Play::Rock,
                Play::Paper => Play::Paper,
                Play::Scissors => Play::Scissors,
            },
            Self::Lose => match play {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            }
        }
    }
}

impl FromStr for Outcome {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(ParseError::UnrecognizedCharacter),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Round {
    play: Play,
    counter: Play,
}

impl Round {
    pub fn score(&self) -> usize {
        self.counter.score() + self.counter.outcome(self.play).score()
    }
}

impl FromStr for Round {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.split_whitespace();
        let play: Play = cs.next().ok_or(Self::Err::InsufficientCharacters)?.parse()?;
        let counter: Play = cs.next().ok_or(Self::Err::InsufficientCharacters)?.parse()?;
        Ok(Round{play, counter})
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rule {
    play: Play,
    outcome: Outcome,
}

impl Rule {
    pub fn score(&self) -> usize {
        let counter = self.outcome.counter_for(self.play);
        counter.score() + self.outcome.score()
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.split_whitespace();
        let play: Play = cs.next().ok_or(Self::Err::InsufficientCharacters)?.parse()?;
        let outcome: Outcome = cs.next().ok_or(Self::Err::InsufficientCharacters)?.parse()?;
        Ok(Rule{play, outcome})
    }
}
