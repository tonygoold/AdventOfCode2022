fn main() {
    let elves = aoc::read_uint_lists(&aoc::input_arg());
    let mut ranking: Vec<usize> = elves.iter().map(|ns| ns.iter().sum::<usize>()).collect();
    ranking.sort_unstable();
    let mut top = ranking.iter();
    let (e1, e2, e3) = (
        top.next_back().expect("no elves"),
        top.next_back().expect("only one elf"),
        top.next_back().expect("only two elves"),
    );
    println!("The elves with the most calories have {}, {}, and {}, or {} in total", e1, e2, e3, e1 + e2 + e3);
}
