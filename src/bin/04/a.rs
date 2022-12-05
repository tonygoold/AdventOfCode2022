use aoc::assignment::Pair;

fn main() {
    let full_overlaps: usize = aoc::read_lines(&aoc::input_arg()).map(|line| {
        line.parse::<Pair>().expect("Invalid assignment pair")
    }).map(|pair| if pair.fully_overlaps() { 1 } else { 0 }).sum();
    println!("The number of full overlaps is {}", full_overlaps);
}
