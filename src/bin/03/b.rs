use aoc::rucksack::Rucksack;

fn main() {
    let rucksacks: Vec<Rucksack> = aoc::read_lines(&aoc::input_arg())
        .map(|s| Rucksack::new(&s)).collect();
    let mut badges: Vec<char> = Vec::new();
    let mut i = rucksacks.iter();
    while let Some(r1) = i.next() {
        let r2 = i.next().expect("Found one rucksack by itself");
        let r3 = i.next().expect("Found two rucksacks by themselves");
        let cs = r1.common(r2);
        let c = r3.common_chars(&cs);
        assert_eq!(c.len(), 1, "A group did not have exactly one item in common");
        badges.push(c[0]);
    }
    let priorities: usize = badges.iter().map(|c| Rucksack::priority(*c)).sum();
    println!("The sum of priorities is {}", priorities);
}