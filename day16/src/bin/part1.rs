use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use aoc::rand::seq::SliceRandom;
use aoc::rand::Rng;
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

static MAX: AtomicUsize = AtomicUsize::new(0);

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

    let distances = graph.generate_cache();
    let distances = &distances;

    // let mut max = 0;
    // let len = to_do.len();
    let start = graph.get_id(&String::from("AA")).unwrap();

    let mut ids = to_do.clone();
    let mut max = 0;
    let started_at = std::time::Instant::now();
    let mut last_solution = std::time::Instant::now();

    loop {
        far_away(&mut ids);
        let ret = play_scenario(start, &distances, &flow, &ids);
        if last_solution.elapsed() >= Duration::from_secs(2) {
            break;
        }
        if ret > max {
            last_solution = std::time::Instant::now();
            max = ret;
            println!("current max {}", ret);
        }
    }

    println!(
        "Finished the fully random phase in {:?}",
        started_at.elapsed()
    );
    println!("The current max is {}", max);

    let good_enough = max - (max / 20);

    println!(
        "Collecting interesting solutions. Anything higher than {}",
        good_enough
    );

    let started_at = std::time::Instant::now();
    let mut interesting = Vec::new();
    loop {
        far_away(&mut ids);
        let ret = play_scenario(start, &distances, &flow, &ids);
        if ret > good_enough {
            interesting.push(ids.clone());
            if started_at.elapsed() >= Duration::from_secs(10) || interesting.len() == 1000 {
                break;
            }
        }
    }
    println!(
        "Collected {} interesting solutions in {:?}.",
        interesting.len(),
        started_at.elapsed()
    );

    for ids in interesting {
        for _ in 0..1000 {
            let mut ids = ids.clone();
            for _ in 0..100 {
                close_away(&mut ids);
                let ret = play_scenario(start, &distances, &flow, &ids);
                if ret > max {
                    max = ret;
                    println!("current max {}", ret);
                }
            }
        }
    }

    // let (tdistances, tflow, tto_do) = (distances.clone(), flow.clone(), to_do.clone());
    // std::thread::spawn(move || test_random_solution(start, tdistances, tflow, tto_do));
    // let (tdistances, tflow, tto_do) = (distances.clone(), flow.clone(), to_do.clone());
    // std::thread::spawn(move || test_random_solution(start, tdistances, tflow, tto_do));
    // let (tdistances, tflow, tto_do) = (distances.clone(), flow.clone(), to_do.clone());
    // std::thread::spawn(move || test_random_solution(start, tdistances, tflow, tto_do));
    // let (tdistances, tflow, tto_do) = (distances.clone(), flow.clone(), to_do.clone());
    // std::thread::spawn(move || test_random_solution(start, tdistances, tflow, tto_do));

    // test_random_solution(start, distances.clone(), flow.clone(), to_do.clone())
    // for scenario in to_do.into_iter().permutations(len) {
    //     let ret = play_scenario(start, &mut distances, &flow, &scenario);
    //     if ret > max {
    //         max = ret;
    //         println!("current max {}", max);
    //     }
    // println!("len: {}", to_do.len());
    // }

    // println!("{}", max);
}

fn test_random_solution(
    start: Id,
    distances: HashMap<(Id, Id), usize>,
    flow: HashMap<Id, usize>,
    ids: Vec<Id>,
) {
    let mut ids = ids.to_vec();
    loop {
        far_away(&mut ids);
        let ret = play_scenario(start, &distances, &flow, &ids);
        if ret > MAX.load(Ordering::Relaxed) {
            MAX.store(ret, Ordering::Relaxed);
            println!("current max {}", ret);
        }
    }
}

// randomize all the elements in the array
fn far_away(elements: &mut [Id]) {
    let mut rng = aoc::rand::thread_rng();
    elements.shuffle(&mut rng);
}

// swap two elements randomly
fn close_away(elements: &mut [Id]) {
    let mut rng = aoc::rand::thread_rng();
    let pos = rng.gen_range(0..elements.len());
    let left = elements[pos];
    if pos + 1 == elements.len() {
        elements[pos] = elements[0];
        elements[0] = left;
    } else {
        elements[pos] = elements[pos + 1];
        elements[pos + 1] = left;
    }
}

fn play_scenario(
    start: Id,
    distances: &HashMap<(Id, Id), usize>,
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
                unreachable!()
            }
        };
        minutes -= distance as isize;
        // println!("{} minutes left", minutes);
        if minutes <= 0 {
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
