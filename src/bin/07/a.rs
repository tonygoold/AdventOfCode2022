use aoc::cli::{Cli, Node};

const MAX_SIZE: usize = 100000;

fn main() {
    let mut cli = Cli::new();
    aoc::read_lines(&aoc::input_arg()).for_each(|line| cli.handle_line(&line));
    let sizes = cli.iter().fold(0, |acc, node| {
        if let Node::Directory(dir) = node {
            let size = dir.size();
            if size <= MAX_SIZE {
                return acc + size;
            }
        };
        acc
    });
    println!("Total size of dirs <= {} is {}", MAX_SIZE, sizes);
}
