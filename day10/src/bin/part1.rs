use aoc::parser;
use day10::*;

fn main() {
    let cpu = Cpu::new(parser::lines().collect());

    let ret = (0..)
        .scan(cpu, |cpu, _| {
            let ret = Some(cpu.clone());
            cpu.process_cycle();
            ret
        })
        .take_while(|cpu| !cpu.finished())
        .skip(19)
        .step_by(40)
        .map(|cpu| cpu.signal())
        .sum::<isize>();

    println!("solution {}", ret);
}
