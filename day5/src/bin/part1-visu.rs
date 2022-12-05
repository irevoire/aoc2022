use day5::*;

use aoc::{answer, parser};

fn main() {
    let input = parser::input::<String>();
    let mut input = input.split("\n\n");

    let setup = input.next().unwrap();
    let mut setup: Crates = setup.parse().unwrap();

    let instructions = input.next().unwrap();
    let instructions = instructions
        .lines()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    for instr in instructions {
        println!("{}", setup);
        println!(" {} ", "-".repeat(setup.crates.len() * 4));
        println!();

        let mut crates = (0..instr.number)
            .map(|_| setup.crates[instr.from - 1].pop().unwrap())
            .collect::<Vec<char>>();
        setup.crates[instr.to - 1].append(&mut crates);
    }

    let answer = setup
        .crates
        .iter()
        .filter_map(|stack| stack.last())
        .collect::<String>();

    answer!(
        "After the rearrangement procedure completes the crates that ends up on top of each stack are {}.",
        answer
    );
}
