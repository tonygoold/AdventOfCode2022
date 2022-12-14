use aoc::inttree::UintNode;

fn main() {
    let mut nodes: Vec<UintNode> = Vec::new();
    let lines = aoc::read_lines(&aoc::input_arg());
    for line in lines {
        if !line.is_empty() {
            let node = line.parse::<UintNode>();
            match node {
                Ok(node) => nodes.push(node),
                Err(err) => println!("Failed to parse '{}': {:?}", &line, err),
            }
        }
    }
    let mut node_iter = nodes.iter();
    let mut pair_index: usize = 0;
    let mut good_indices: Vec<usize> = Vec::new();
    while let Some(left) = node_iter.next() {
        pair_index += 1;
        let right = node_iter.next().expect("Unbalanced pair");
        if left.cmp(right) == std::cmp::Ordering::Less {
            good_indices.push(pair_index);
        }
    }
    let score: usize = good_indices.iter().sum();
    println!("The score is {}", score);
}
