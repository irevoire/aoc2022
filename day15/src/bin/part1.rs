use std::collections::HashSet;

use aoc::{parser, Coord};
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
    let (sensors, beacons): (Vec<_>, HashSet<_>) = parser::lines::<Input>()
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

    let checking = 2000000;
    let mut total = HashSet::new();

    for sensor in sensors {
        let middle = Coord::at(sensor.position.x, checking);
        let distance = sensor.position.manhattan_distance_from(&middle);
        if distance <= sensor.distance {
            // the sensor can see some points on the targetted line
            let can_see = sensor.distance - distance;

            total.extend(
                Coord::at(middle.x - can_see, middle.y)
                    .to(Coord::at(middle.x + can_see, middle.y))
                    .unwrap(),
            );
        }
    }
    let ret = total.difference(&beacons).count();
    println!("{}", ret);
}
