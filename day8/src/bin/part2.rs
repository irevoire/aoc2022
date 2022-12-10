use std::collections::HashSet;

use aoc::*;

fn main() {
    let grid = Grid::from(
        parser::lines::<String>()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect(),
    );

    let border = grid.borders_coord().collect::<HashSet<Coord<usize>>>();

    let ret = grid
        .enumerate()
        .filter(|(coord, _)| !border.contains(coord))
        .map(|(coord, base_level)| {
            let (left, right) = grid.lines().nth(coord.y).unwrap().split_at(coord.x);
            let tmp = grid.columns().nth(coord.x).unwrap();
            let (top, bottom) = tmp.split_at(coord.y);

            let left = left.iter().rev().cloned().collect::<Vec<usize>>();
            let right = right.iter().skip(1).cloned().collect::<Vec<usize>>();
            let top = top.iter().rev().cloned().cloned().collect::<Vec<usize>>();
            let bottom = bottom
                .iter()
                .skip(1)
                .cloned()
                .cloned()
                .collect::<Vec<usize>>();

            let res = [left, right, top, bottom]
                .iter()
                .map(|view| {
                    view.iter()
                        .position(|el| el >= base_level)
                        .map(|el| el + 1)
                        .unwrap_or(view.len())
                })
                .fold(1, |acc, view| acc * view);
            res
        })
        .max()
        .unwrap();

    println!("{}", ret);
}
