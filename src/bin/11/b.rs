use aoc::monkey::Monkeys;

fn main() {
    let lines = aoc::read_lines(&aoc::input_arg()).collect::<Vec<_>>().join("\n");
    let mut monkeys: Monkeys = lines.parse().expect("Failed to parse monkeys");
    monkeys.set_reduces(false);
    (0..10000).for_each(|_| monkeys.do_round());
    let mut inspections = monkeys.inspections();
    inspections.sort_unstable();
    let n1 = inspections.pop().expect("No monkey scores");
    let n2 = inspections.pop().expect("Only one monkey score");
    println!("The level of monkey business is {}", n1 * n2);
}
