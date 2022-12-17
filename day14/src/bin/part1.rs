use aoc::*;

fn main() {
    let paths = parser::lines::<String>()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| coord.parse::<Coord>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_x = paths
        .iter()
        .map(|path| path.iter().map(|coord| coord.x))
        .flatten()
        .max()
        .unwrap();
    let max_y = paths
        .iter()
        .map(|path| path.iter().map(|coord| coord.y))
        .flatten()
        .max()
        .unwrap();

    let grid = Grid::<char>::with_dimension(max_x + 1, max_y + 1);
    let mut grid = grid.map(|_| '.');

    for path in paths {
        assert!(path.len() > 1);
        for a in path.windows(2) {
            let (mut start, end) = (a[0], a[1]);
            while start != end {
                grid[start] = '#';
                start.move_toward(&end);
            }
            grid[start] = '#';
        }
    }

    for i in 0.. {
        let mut display = grid.clone();
        display.trim_matches(|el| *el == '.');
        println!("Step {i}");
        println!("{}", display);
        if let Some(coord) = drop_sand(&grid) {
            grid[coord] = 'o';
        } else {
            break;
        }
    }
}

// return none if the sand fall off the bottle
fn drop_sand(grid: &Grid<char>) -> Option<Coord<usize>> {
    let mut sand = Coord::at(500, 0);
    loop {
        if grid.get(sand + Direction::South)? == &'.' {
            sand += Direction::South;
        } else if grid.get(sand + Direction::South + Direction::West)? == &'.' {
            sand = sand + Direction::South + Direction::West;
        } else if grid.get(sand + Direction::South + Direction::East)? == &'.' {
            sand = sand + Direction::South + Direction::East;
        } else {
            return Some(sand);
        }
    }
}
