use std::collections::HashMap;

use aoc::rand::seq::SliceRandom;
use aoc::{parser, Graph, Id, Itertools};
use parse_display::FromStr;

#[derive(FromStr, Debug)]
#[from_str(
    regex = "Valve (?P<valve>[A-Z]*) has flow rate=(?P<flow_rate>[0-9]*); tunnels? leads? to valves? (?P<leads_to>.*)"
)]
struct Input {
    valve: String,
    flow_rate: usize,
    leads_to: Valves,
}

#[derive(Debug)]
struct Valves(Vec<String>);

impl std::str::FromStr for Valves {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(Valves(s.split(',').map(|s| s.trim().to_string()).collect()))
    }
}

fn main() {
    let mut graph = Graph::<String>::new_undirected();
    let mut flow = HashMap::new();

    for Input {
        valve,
        flow_rate,
        leads_to,
    } in parser::lines()
    {
        let main = graph.insert_value(valve);
        flow.insert(main, flow_rate);

        for valve in leads_to.0 {
            let other = graph.insert_value(valve);
            graph.create_edge(main, other);
        }
    }

    let to_do: Vec<_> = flow
        .iter()
        .filter(|(_, flow)| **flow > 0)
        .map(|(id, _)| *id)
        .collect();

    let mut distances = HashMap::new();
    // for nodes in to_do.iter().combinations(2) {
    //     let (left, right) = (*nodes[0], *nodes[1]);
    //     let distance = graph.distance_between(left, right).unwrap();
    //     distances.insert((left, right), distance);
    //     distances.insert((right, left), distance);
    // }
    // dbg!(&distances);

    // let best_solution =
    //     ["DD", "BB", "JJ", "HH", "EE", "CC"].map(|valve| graph.get_id(&valve.to_string()).unwrap());

    // let ret = play_scenario(&graph, &flow, &best_solution);

    let mut max = 0;
    let len = to_do.len();
    let start = graph.get_id(&String::from("AA")).unwrap();
    for scenario in to_do.into_iter().permutations(len) {
        let ret = play_scenario(start, &graph, &mut distances, &flow, &scenario);
        if ret > max {
            max = ret;
            println!("current max {}", max);
        }
        // println!("len: {}", to_do.len());
    }

    println!("{}", max);
}

// fn far_away(elements: &mut [Id]) {
//     let mut rng = aoc::rand::thread_rng();
//     elements.shuffle(&mut rng);
// }

fn play_scenario(
    start: Id,
    graph: &Graph<String>,
    distances: &mut HashMap<(Id, Id), usize>,
    flow: &HashMap<Id, usize>,
    ids: &[Id],
) -> usize {
    let mut current_position = start;
    let mut pressure_released = 0;
    let mut pressure_releasing = 0;

    let mut minutes: isize = 30;

    for id in ids {
        // println!("You move to valve {}", graph.get_value(*id).unwrap());
        let distance = match distances.get(&(current_position, *id)) {
            Some(distance) => distance + 1,
            None => {
                println!("cache miss");
                let (left, right) = (current_position, *id);
                let distance = graph.distance_between(left, right).unwrap();
                distances.insert((left, right), distance);
                distances.insert((right, left), distance);
                distance + 1
            }
        };
        minutes -= distance as isize;
        // println!("{} minutes left", minutes);
        if minutes < 0 {
            return pressure_released + (minutes.abs() as usize * pressure_releasing);
        }
        // println!("adding {} * {}", pressure_releasing, distance);
        pressure_released += pressure_releasing * distance;
        // println!("Released {} in total", pressure_released);
        pressure_releasing += flow[id];
        // println!("Now releasing {} every minutes", pressure_releasing);
        current_position = *id;
        // println!();
    }

    pressure_released + (pressure_releasing * minutes as usize)
}

// fn play_scenario(graph: &Graph<String>, flow: &HashMap<Id, usize>, ids: &[Id]) -> usize {
//     let mut current_position = graph.get_id(&String::from("AA")).unwrap();
//     let mut pressure_released = 0;
//     let mut pressure_releasing = 0;
//
//     let mut minutes: isize = 30;
//
//     for id in ids {
//         // println!("You move to valve {}", graph.get_value(*id).unwrap());
//         let distance = graph.distance_between(current_position, *id).unwrap() + 1;
//         minutes -= distance as isize;
//         // println!("{} minutes left", minutes);
//         if minutes < 0 {
//             return pressure_released + (minutes.abs() as usize * pressure_releasing);
//         }
//         // println!("adding {} * {}", pressure_releasing, distance);
//         pressure_released += pressure_releasing * distance;
//         // println!("Released {} in total", pressure_released);
//         pressure_releasing += flow[id];
//         // println!("Now releasing {} every minutes", pressure_releasing);
//         current_position = *id;
//         // println!();
//     }
//
//     pressure_released + (pressure_releasing * minutes as usize)
// }
