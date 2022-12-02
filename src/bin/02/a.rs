use aoc::rps::{ParseError, Round};

fn main() {
    let rules: Vec<Round> = aoc::read_lines(&aoc::input_arg())
        .map(|s| s.parse()).collect::<Result<_, ParseError>>().expect("Failed to parse rules");
    let score: usize = rules.iter().map(Round::score).sum();
    println!("The score is {}", score);
}
