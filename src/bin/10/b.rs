use aoc::cpu::{CPU, Instruction};

fn main() {
    let insts = aoc::read_lines(&aoc::input_arg()).map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>().expect("failed to parse instructions");
    let mut cpu = CPU::new(&insts);
    let mut succeeded = true;
    let mut output = String::new();
    while succeeded {
        let cycle = ((cpu.cycle() - 1) % 40) as isize;
        let x = cpu.x();
        if (cycle-1..=cycle+1).contains(&x) {
            output.push('#');
        } else {
            output.push('.');
        }
        if cpu.cycle() % 40 == 0 {
            output.push('\n');
            if cycle == 220 {
                break;
            }
        }
        succeeded = cpu.tick();
    }
    println!("The output is:\n{}", &output);
}
