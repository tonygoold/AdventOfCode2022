use aoc::assignment::Pair;

fn main() {
    let overlaps: usize = aoc::read_lines(&aoc::input_arg()).map(|line| {
        line.parse::<Pair>().expect("Invalid assignment pair")
    }).map(|pair| if pair.overlaps() { 1 } else { 0 }).sum();
    println!("The number of full overlaps is {}", overlaps);
}
