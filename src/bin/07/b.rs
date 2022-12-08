use aoc::cli::{Cli, Node};

const DISK_SIZE: usize = 70000000;
const FREE_SPACE_REQUIRED: usize = 30000000;

fn main() {
    let mut cli = Cli::new();
    aoc::read_lines(&aoc::input_arg()).for_each(|line| cli.handle_line(&line));
    let free_space = DISK_SIZE - cli.root_dir().size();
    let additional_space_required = FREE_SPACE_REQUIRED - free_space;
    let min_size = cli.iter().fold(0, |min, node| {
        if let Node::Directory(dir) = node {
            let size = dir.size();
            if size >= additional_space_required && (min == 0 || size < min) {
                return size;
            } else {
                return min;
            }
        };
        min
    });
    println!("The smallest size to free up {} is {}", additional_space_required, min_size);
}
