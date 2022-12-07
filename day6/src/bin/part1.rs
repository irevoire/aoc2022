use aoc::*;

fn main() {
    let position = parser::input::<String>()
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|a| a.into_iter().sorted().dedup().count() == 4)
        .unwrap()
        + 4;

    answer!("The packet position is {}.", position);
}
