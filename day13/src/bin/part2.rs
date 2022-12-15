use aoc::*;
use day13::*;

fn main() {
    let mut packets = parser::lines::<String>()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Value>().unwrap())
        .collect::<Vec<_>>();

    let divider_1 = Value::List(vec![Value::List(vec![Value::Number(2)])]);
    let divider_2 = Value::List(vec![Value::List(vec![Value::Number(6)])]);

    packets.push(divider_1.clone());
    packets.push(divider_2.clone());

    packets.sort();

    println!(
        "ret {}",
        (packets.iter().position(|p| p.clone() == divider_1).unwrap() + 1)
            * (packets.iter().position(|p| p.clone() == divider_2).unwrap() + 1)
    );
}
