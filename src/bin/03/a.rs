use aoc::rucksack::Rucksack;

fn main() {
    let rucksacks = aoc::read_lines(&aoc::input_arg())
        .map(|s| Rucksack::new(&s));
    let priorities: usize = rucksacks.map(|r| r.duplicate_priority()).sum();
    println!("The sum of the duplicate priorities is {}", priorities);
}
