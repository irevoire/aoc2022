use std::{cmp::Ordering, str::FromStr};

use aoc::*;

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Value {
    Number(usize),
    List(Vec<Value>),
}

impl Value {
    fn parse(mut s: &str) -> (Value, &str) {
        let c = s.chars().next().unwrap();

        match c {
            '0'..='9' => {
                let end = s.find(|c| c == ',' || c == ']').unwrap();
                let (v, rem) = s.split_at(end);
                (Value::Number(v.parse().unwrap()), rem)
            }
            '[' => {
                // println!("started a list");
                s = &s[1..];
                let mut values = Vec::new();
                loop {
                    let c = s.chars().next().unwrap();

                    match c {
                        ']' => return (Value::List(values), &s[1..]),
                        ',' => {
                            let (v, rem) = Value::parse(&s[1..]);
                            s = rem;
                            // println!("pushed a value in the list");
                            values.push(v);
                        }
                        '0'..='9' | '[' => {
                            let (v, rem) = Value::parse(s);
                            s = rem;
                            // println!("pushed a value in the list");
                            values.push(v);
                        }
                        c => unreachable!("{}", c),
                    }
                }
            }
            c => unreachable!("{}", c),
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ret, rem) = Self::parse(s);
        // println!("returning {ret:?}");
        assert_eq!(rem, "");
        std::result::Result::Ok(ret)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => left.partial_cmp(right),
            (left @ Value::Number(_), right @ Value::List(_)) => {
                Value::List(vec![left.clone()]).partial_cmp(right)
            }
            (left @ Value::List(_), right @ Value::Number(_)) => {
                left.partial_cmp(&Value::List(vec![right.clone()]))
            }
            (Value::List(left), Value::List(right)) => left.partial_cmp(right),
        }
    }
}

fn main() {
    let count = parser::input::<String>()
        .split("\n\n")
        .map(|pairs| pairs.split_once('\n').unwrap())
        .map(|(left, right)| {
            (
                left.parse::<Value>().unwrap(),
                right.parse::<Value>().unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(idx, _)| idx + 1)
        // .inspect(|(id, _)| println!("remaining {}", id + 1))
        .sum::<usize>();

    println!("count {}", count);
}
