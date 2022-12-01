use aoc::*;

fn main() {
    let ret = parser::input::<String>()
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<usize>();

    answer!(
        "The top three Elves carrying the most calories carries {} calories in total.",
        ret
    );
}
