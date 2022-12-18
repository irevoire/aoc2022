use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use aoc::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    // * the number of possible shapes
    let exhaust_both = movements.len() * 5;

    let mut shapes = (0..=4)
        .cycle()
        .map(|shape| Shape::new(shape))
        .take(1000000000000);
    // .take(exhaust_both);
    // .progress_count(1000000000000);
    let mut shape = None;

    let mut top_shapes = HashMap::new();

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
            let high = highest_point(&walls);
            let prev = top_shapes.insert((top_shape(&walls), movement, s.clone()), (step, high));
            shape = None;
            if let Some((prev_high, prev_step)) = prev {
                println!(
                    "HERE SYNC POINT REACHED in {}, previous matching step was {}",
                    step, prev_step
                );
                println!("prev high {}", prev_high);
                println!("new high {}", high);
                println!("Got bigger by {} unit in between", prev_high - high);
                let repeat = 1000000000000 / step;
                let height = high * repeat;
                println!("final heigh without remaining is {}", height);

                return;
            }
        } else {
            *s = down;
        }
    }

    let highest = highest_point(&walls);
    let repeat = 1000000000000 / exhaust_both;

    println!("highest point is {}", highest * repeat);
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

fn top_shape(walls: &HashSet<Coord<usize>>) -> Shape {
    let max = highest_point(walls);

    let mut shape = (0..7)
        .map(|x| {
            let mut coord = Coord::at(x, max);
            while !walls.contains(&coord) {
                coord += Direction::North;
            }
            coord
        })
        .collect::<Vec<_>>();

    // reduce this shape to the ground level
    let min = shape.iter().map(|coord| coord.y).min().unwrap();
    for coord in &mut shape {
        coord.y -= min;
    }

    Shape { coords: shape }
}
