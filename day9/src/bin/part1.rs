use std::collections::HashSet;

use aoc::{parser, Coord, Direction};
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Clone)]
#[display("{dir} {n}")]
struct Input {
    dir: Direction,
    n: usize,
}

fn main() {
    let positions = parser::lines::<Input>()
        .map(|Input { dir, n }| std::iter::repeat(dir).take(n))
        .flatten()
        .map(|dir| match dir {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            dir => dir,
        })
        .scan(Coord::<isize>::at(0, 0), |head, mov| {
            *head += mov;
            Some(*head)
        })
        .scan(Coord::<isize>::at(0, 0), |tail, head| {
            if tail.is_chebyshev_adjacent(&head) {
                Some(None)
            } else {
                tail.move_toward(&head);
                Some(Some(*tail))
            }
        })
        .filter_map(|c| c)
        // the default position have been missed since we ignored everything
        // before the first move
        .chain(Some(Coord::default()))
        .collect::<HashSet<_>>();

    println!("solution: {}", positions.len());
}
