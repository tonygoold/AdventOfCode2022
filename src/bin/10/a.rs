use aoc::cpu::{CPU, Instruction};

fn main() {
    let insts = aoc::read_lines(&aoc::input_arg()).map(|line| line.parse())
        .collect::<Result<Vec<Instruction>, _>>().expect("failed to parse instructions");
    let mut cpu = CPU::new(&insts);
    let mut signals: Vec<isize> = Vec::new();
    let mut succeeded = true;
    while succeeded {
        let cycle = cpu.cycle();
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            signals.push(cpu.signal_strength());
            if cycle == 220 {
                break;
            }
        }
        succeeded = cpu.tick();
    }
    let sum: isize = signals.iter().take(6).sum();
    println!("The sum of signal strengths is {}", sum);
}
