use aoc::*;

fn main() {
    let ret = parser::input::<String>()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();

    answer!("The Elf carrying the most calories carry {} calories.", ret);
}
