use std::{collections::HashSet, ops::Add};

use aoc::*;

#[derive(Debug, Clone)]
pub struct Shape {
    coords: Vec<Coord<usize>>,
}

impl Shape {
    pub fn new(variant: usize) -> Self {
        match variant {
            0 => Shape {
                coords: vec![
                    Coord::at(0, 0),
                    Coord::at(1, 0),
                    Coord::at(2, 0),
                    Coord::at(3, 0),
                ],
            },
            1 => Shape {
                coords: vec![
                    Coord::at(1, 0),
                    Coord::at(0, 1),
                    Coord::at(1, 1),
                    Coord::at(2, 1),
                    Coord::at(1, 2),
                ],
            },
            2 => Shape {
                coords: vec![
                    Coord::at(0, 0),
                    Coord::at(1, 0),
                    Coord::at(2, 0),
                    Coord::at(2, 1),
                    Coord::at(2, 2),
                ],
            },
            3 => Shape {
                coords: vec![
                    Coord::at(0, 0),
                    Coord::at(0, 1),
                    Coord::at(0, 2),
                    Coord::at(0, 3),
                ],
            },
            4 => Shape {
                coords: vec![
                    Coord::at(0, 0),
                    Coord::at(1, 0),
                    Coord::at(0, 1),
                    Coord::at(1, 1),
                ],
            },
            _ => unreachable!(),
        }
    }

    pub fn collide(&self, walls: &HashSet<Coord<usize>>) -> bool {
        self.coords.iter().any(|coord| walls.contains(&coord))
    }

    pub fn touch_right(&self) -> bool {
        self.coords.iter().any(|coord| coord.x == 6)
    }

    pub fn touch_left(&self) -> bool {
        self.coords.iter().any(|coord| coord.x == 0)
    }
}

impl Add<Direction> for Shape {
    type Output = Shape;

    fn add(mut self, rhs: Direction) -> Self::Output {
        for coord in &mut self.coords {
            *coord += rhs;
        }
        self
    }
}

impl Add<Coord<usize>> for Shape {
    type Output = Shape;

    fn add(mut self, rhs: Coord<usize>) -> Self::Output {
        for coord in &mut self.coords {
            *coord += rhs;
        }
        self
    }
}

fn main() {
    let movements: Vec<_> = parser::chars::<Direction>().collect();

    let mut walls = HashSet::new();
    for x in 0..7 {
        walls.insert(Coord::at(x, 0));
    }

    let mut shapes = (0..=4).cycle().map(|shape| Shape::new(shape)).take(2022);
    let mut shape = None;

    for (step, movement) in movements.into_iter().cycle().enumerate() {
        // println!("Step {step}, moving {movement:?}");
        if shape.is_none() {
            let high = highest_point(&walls);
            shape = match shapes.next() {
                Some(shape) => Some(shape + Coord::at(2, high + 4)),
                None => break,
            };
        }
        let s = shape.as_mut().unwrap();
        // display(&walls, s);
        match movement {
            Direction::West if s.touch_left() => (),
            Direction::East if s.touch_right() => (),
            direction => {
                let mv = s.clone() + direction;
                if !mv.collide(&walls) {
                    *s = mv;
                }
            }
        }
        let down = s.clone() + Direction::North;
        if down.collide(&walls) {
            // display(&walls, s);
            s.coords.iter().for_each(|coord| drop(walls.insert(*coord)));
            shape = None;
        } else {
            *s = down;
        }
    }

    println!("highest point is {}", highest_point(&walls));
}

fn highest_point(walls: &HashSet<Coord<usize>>) -> usize {
    walls.iter().map(|coord| coord.y).max().unwrap()
}

fn display(walls: &HashSet<Coord<usize>>, shape: &Shape) {
    let grid = Grid::<char>::with_dimension(7, 20);
    let mut grid = grid.map(|_| '.');
    for coord in walls {
        grid[coord] = '#';
    }
    for coord in &shape.coords {
        grid[coord] = '@';
    }

    grid.horizontal_symmetry();

    println!("{}", grid);
}
