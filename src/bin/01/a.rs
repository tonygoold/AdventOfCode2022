fn main() {
    let elves = aoc::read_uint_lists(&aoc::input_arg());
    let solution = elves.iter().map(|ns| ns.iter().sum::<usize>()).max();
    println!("The elf with the most calories has {}", solution.expect("no input"));
}
