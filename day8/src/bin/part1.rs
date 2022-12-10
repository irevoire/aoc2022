use std::collections::HashSet;

use aoc::*;

fn main() {
    let mut grid = Grid::from(
        parser::lines::<String>()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect(),
    );

    let width = grid.width();
    let height = grid.height();
    let mut nb_trees_visible: HashSet<Coord<usize>> = grid.borders_coord().collect();

    // seing from the left
    grid.lines()
        .enumerate()
        .skip(1)
        .map(|(l, line)| {
            let mut iter = line.iter().copied().enumerate();
            let mut current_max = iter.next().unwrap().1;

            iter.tuple_windows::<((usize, usize), (usize, usize))>()
                .filter(|(a, _)| {
                    if a.1 > current_max {
                        current_max = a.1;
                        true
                    } else {
                        false
                    }
                })
                .map(|(a, _)| Coord::at(l, a.0))
                .collect::<Vec<Coord<usize>>>()
        })
        .flatten()
        .for_each(|coord| drop(nb_trees_visible.insert(coord)));

    // seing from the top
    grid.columns()
        .enumerate()
        .map(|(c, column)| {
            let mut iter = column.iter().copied().copied().enumerate();
            let mut current_max = iter.next().unwrap().1;

            iter.tuple_windows::<((usize, usize), (usize, usize))>()
                .filter(|(a, _)| {
                    if a.1 > current_max {
                        current_max = a.1;
                        true
                    } else {
                        false
                    }
                })
                .map(|(a, _)| Coord::at(a.0, c))
                .collect::<Vec<Coord<usize>>>()
        })
        .flatten()
        .for_each(|coord| drop(nb_trees_visible.insert(coord)));

    grid.rotate_left();
    grid.rotate_left();

    // seing from the right
    grid.lines()
        .enumerate()
        .skip(1)
        .map(|(l, line)| {
            let mut iter = line.iter().copied().enumerate();
            let mut current_max = iter.next().unwrap().1;

            iter.tuple_windows::<((usize, usize), (usize, usize))>()
                .filter(|(a, _)| {
                    if a.1 > current_max {
                        current_max = a.1;
                        true
                    } else {
                        false
                    }
                })
                .map(|(a, _)| Coord::at(width - l - 1, height - a.0 - 1))
                .collect::<Vec<Coord<usize>>>()
        })
        .flatten()
        .for_each(|coord| drop(nb_trees_visible.insert(coord)));

    // seing from the bottom
    grid.columns()
        .enumerate()
        .map(|(c, column)| {
            let mut iter = column.iter().copied().copied().enumerate();
            let mut current_max = iter.next().unwrap().1;

            iter.tuple_windows::<((usize, usize), (usize, usize))>()
                .filter(|(a, _)| {
                    if a.1 > current_max {
                        current_max = a.1;
                        true
                    } else {
                        false
                    }
                })
                .map(|(a, _)| Coord::at(width - a.0 - 1, height - c - 1))
                .collect::<Vec<Coord<usize>>>()
        })
        .flatten()
        .for_each(|coord| drop(nb_trees_visible.insert(coord)));

    grid.rotate_left();
    grid.rotate_left();

    println!("{:?}", nb_trees_visible.len());
}
