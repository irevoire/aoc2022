#![feature(hash_drain_filter)]

use std::collections::HashSet;

use aoc::{
    parser, Coord, IntoParallelRefIterator, ParallelIterator, ParallelProgressIterator,
    ProgressIterator,
};
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Copy, Clone)]
#[display("Sensor at x={s_x}, y={s_y}: closest beacon is at x={b_x}, y={b_y}")]
struct Input {
    s_x: isize,
    s_y: isize,

    b_x: isize,
    b_y: isize,
}

struct Sensor {
    pub position: Coord<isize>,
    pub distance: isize,
}

fn main() {
    let max = 4000000;
    // let max = 20;

    let (sensors, _beacons): (Vec<_>, HashSet<_>) = parser::lines::<Input>()
        .map(|input| {
            let sensor = Coord::at(input.s_x, input.s_y);
            let beacon = Coord::at(input.b_x, input.b_y);
            (
                Sensor {
                    position: sensor,
                    distance: sensor.manhattan_distance_from(&beacon),
                },
                beacon,
            )
        })
        .unzip();

    let mut coords = sensors
        .par_iter()
        .progress_count(sensors.len() as u64)
        .map(|sensor| {
            sensor
                .position
                .manhattan_coords_at_distance(sensor.distance + 1)
                .into_iter()
                .filter(|coord| (0..=max).contains(&coord.x) && (0..=max).contains(&coord.y))
                .collect::<HashSet<Coord<_>>>()
        })
        .reduce(
            || HashSet::new(),
            |left, right| left.union(&right).cloned().collect(),
        );

    // let mut coords = HashSet::new();
    // for sensor in sensors.iter() {
    //     println!("here");
    //     coords.extend(
    //         sensor
    //             .position
    //             .manhattan_coords_at_distance(sensor.distance + 1)
    //             .into_iter()
    //             .filter(|coord| (0..=max).contains(&coord.x) && (0..=max).contains(&coord.y)),
    //     )
    // }

    println!("There is {} possible coordinates", coords.len());

    for sensor in sensors {
        coords.drain_filter(|coord| {
            sensor.position.manhattan_distance_from(&coord) <= sensor.distance
        });
    }

    println!("{:?} coordinates left", coords);
    let coord = coords.into_iter().next().unwrap();
    println!("frequency: {}", coord.x * max + coord.y);
}
