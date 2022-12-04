use aoc::{answer, parser};
use day4::*;

fn main() {
    let contained = parser::lines::<Pair>().filter(Pair::contained).count();
    answer!(
        "In {} assignment pairs one range fully contain the other.",
        contained
    );
}
