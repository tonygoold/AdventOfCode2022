use std::str::FromStr;

fn main() {
    let mut lines = aoc::read_lines(&aoc::input_arg());
    let mut layout: Vec<String> = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        layout.push(line);
    }
    let number_line = layout.pop().expect("did not find stack numbering");
    let numbers = number_line.split_whitespace()
        .map(|s| s.parse::<usize>().expect("non-number found in stack numbering"));
    let mut stacks = aoc::crane::Stacks::new(numbers.count());
    for row in layout.iter().rev() {
        stacks.populate(row);
    }
    let movements = lines.map(|s| {
        aoc::crane::Movement::from_str(&s).expect("failed to parse movement")
    });
    for movement in movements {
        stacks.apply_keeping_order(&movement)
    }
    println!("The tops of the stacks spell {}", stacks.peek_all());
}
