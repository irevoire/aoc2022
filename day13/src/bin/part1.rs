use aoc::*;
use day13::*;

fn main() {
    let count = parser::input::<String>()
        .split("\n\n")
        .map(|pairs| pairs.split_once('\n').unwrap())
        .map(|(left, right)| {
            (
                left.parse::<Value>().unwrap(),
                right.parse::<Value>().unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(idx, _)| idx + 1)
        .sum::<usize>();

    println!("count {}", count);
}
