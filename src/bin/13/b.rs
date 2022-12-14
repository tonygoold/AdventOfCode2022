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
    nodes.push("[[2]]".parse().expect("Failed to parse known good input"));
    nodes.push("[[6]]".parse().expect("Failed to parse known good input"));
    // nodes.push(UintNode::Tree(vec![UintNode::Tree(vec![UintNode::Leaf(2)])]));
    // nodes.push(UintNode::Tree(vec![UintNode::Tree(vec![UintNode::Leaf(6)])]));
    nodes.sort_unstable();
    let mut first_index = 0;
    let mut second_index = 0;
    for (index, node) in nodes.iter().enumerate() {
        if let UintNode::Tree(ns) = node {
            if ns.len() != 1 {
                continue;
            }
            if let UintNode::Tree(ns) = &ns[0] {
                if ns.len() != 1 {
                    continue;
                }
                if let UintNode::Leaf(n) = &ns[0] {
                    match *n {
                        2 => first_index = index + 1,
                        6 => second_index = index + 1,
                        _ => {}
                    }
                }
            }
        }
        if second_index > 0 {
            break;
        }
    }
    println!("The score is {}", first_index * second_index);
}
