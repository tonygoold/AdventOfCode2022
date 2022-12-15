use aoc::cave::{Cave, Movement, Path};

fn main() {
    let paths: Vec<Path> = aoc::read_lines(&aoc::input_arg())
        .map(|line| line.parse::<Path>())
        .collect::<Result<Vec<_>, _>>().expect("Failed to parse paths");
    let mut cave = Cave::new(paths, true);
    let mut count: usize = 0;
    loop {
        match cave.tick() {
            Movement::Escape => break,
            Movement::Stop => count += 1,
            _ => {},
        }
    }
    println!("There are {} units of sand at rest", count);
}
