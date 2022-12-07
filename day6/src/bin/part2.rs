use aoc::*;

fn main() {
    let position = parser::input::<String>()
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|a| a.into_iter().sorted().dedup().count() == 14)
        .unwrap()
        + 14;

    answer!("The packet position is {}.", position);
}
