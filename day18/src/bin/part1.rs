use std::collections::HashSet;

use aoc::parser;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[display("{x},{y},{z}")]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

fn main() {
    let coords: HashSet<_> = parser::lines::<Coord>().collect();

    let mut exposed = 0;

    for Coord { x, y, z } in coords.iter().copied() {
        if !coords.contains(&Coord { x: x + 1, y, z }) {
            exposed += 1;
        }
        if !coords.contains(&Coord { x: x - 1, y, z }) {
            exposed += 1;
        }
        if !coords.contains(&Coord { x, y: y + 1, z }) {
            exposed += 1;
        }
        if !coords.contains(&Coord { x, y: y - 1, z }) {
            exposed += 1;
        }
        if !coords.contains(&Coord { x, y, z: z + 1 }) {
            exposed += 1;
        }
        if !coords.contains(&Coord { x, y, z: z - 1 }) {
            exposed += 1;
        }
    }

    // dbg!(&coords);
    println!("{}", exposed);
}
