use std::{cmp::Reverse, collections::HashSet};

use aoc::*;

fn main() {
    let grid: Grid<char> = Grid::from(
        parser::lines::<String>()
            .map(|line| line.chars().collect())
            .collect(),
    );

    let ending_position = grid.position(|&c| c == 'E').unwrap();

    let grid = grid.map(|height| match height {
        'S' => b'a' - b'a',
        'E' => b'z' - b'a',
        b => b as u8 - b'a',
    });

    println!("ends at {:?}", ending_position);

    let mut available = grid
        .enumerate()
        .filter(|(_, h)| **h == 0)
        .map(|(coord, _)| (coord, 0))
        .collect::<Vec<_>>();

    let mut explored = HashSet::new();

    loop {
        available.sort_unstable_by_key(|s| Reverse(s.1));
        let (current_coord, current_distance) = available.pop().unwrap();
        if explored.contains(&current_coord) {
            continue;
        }

        if current_coord == ending_position {
            println!("found in {}", current_distance);
            break;
        }

        let current_height = grid[current_coord];
        explored.insert(current_coord);

        current_coord
            .manhattan_adjacent()
            .filter(|coord| !explored.contains(&coord))
            .filter(|coord| grid.get(*coord).is_some())
            .filter(|coord| grid[coord] <= current_height + 1)
            .for_each(|coord| available.push((coord, current_distance + 1)));
    }
}
