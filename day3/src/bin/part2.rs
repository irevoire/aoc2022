use std::collections::HashSet;

use aoc::*;

fn main() {
    let priorities = parser::lines::<String>()
        .map(|bag| bag.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|mut bags| {
            bags.next()
                .unwrap()
                .intersection(&bags.next().unwrap())
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&bags.next().unwrap())
                .copied()
                .collect::<Vec<char>>()
        })
        .flatten()
        .map(|c| match c {
            'a'..='z' => c as u8 - b'a' + 1,
            'A'..='Z' => c as u8 - b'A' + 27,
            _ => unreachable!(),
        })
        .map(|b| b as usize)
        .sum::<usize>();

    answer!(
        "The sum of the priorities of those item types as {}.",
        priorities
    );
}
