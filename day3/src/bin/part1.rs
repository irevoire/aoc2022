use std::collections::HashSet;

use aoc::*;

fn main() {
    let priorities = parser::lines::<String>()
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            [left, right].map(|p| p.chars().collect::<HashSet<char>>())
        })
        .map(|[left, right]| left.intersection(&right).copied().collect::<Vec<char>>())
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
