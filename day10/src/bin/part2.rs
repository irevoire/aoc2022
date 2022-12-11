use aoc::parser;
use day10::*;

fn main() {
    let cpu = Cpu::new(parser::lines().collect());

    print!("Cycle {:3} -> ", 1);

    for cpu in (0..)
        .scan(cpu, |cpu, _| {
            let ret = Some(cpu.clone());
            cpu.process_cycle();
            ret
        })
        .take_while(|cpu| !cpu.finished())
    {
        let l = ((cpu.cycle) % 40) as isize;
        if l == 1 {
            print!(" <- Cycle {:3}", cpu.cycle - 1);
            println!();
            print!("Cycle {:3} -> ", cpu.cycle);
        }
        if (cpu.x..cpu.x + 3).contains(&(l as isize)) {
            print!("#");
        } else {
            print!(".");
        }
    }
}
