use std::collections::HashMap;

use aoc::parser;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug, Clone)]
#[from_str(regex = "Monkey (?P<id>[0-9]+):
  Starting items: (?P<items>.*)
  Operation: (?P<operation>.*)
  Test: (?s)(?P<test>.*)")]
pub struct Monkey {
    id: usize,
    items: Items,
    operation: Operation,
    test: Test,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Items(Vec<usize>);

#[derive(FromStr, PartialEq, Debug, Clone)]
#[display("new = {left} {op} {right}")]
pub struct Operation {
    left: String,
    op: Op,
    right: String,
}

impl Operation {
    pub fn apply(&self, old: usize) -> usize {
        assert_eq!(&self.left, "old");
        let left = old;

        let right = match self.right.as_ref() {
            "old" => old,
            s => s.parse().unwrap(),
        };

        match self.op {
            Op::Add => left + right,
            Op::Mul => left * right,
        }
    }
}

#[derive(FromStr, PartialEq, Debug, Clone)]
pub enum Op {
    #[from_str(regex = r"\+")]
    Add,
    #[from_str(regex = r"\*")]
    Mul,
}

impl std::str::FromStr for Items {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Items(s.split(", ").map(|s| s.parse().unwrap()).collect()))
    }
}

#[derive(FromStr, PartialEq, Debug, Clone)]
#[display("divisible by {divisible_by}\n    If true: throw to monkey {yes}\n    If false: throw to monkey {no}")]
pub struct Test {
    divisible_by: usize,
    yes: usize,
    no: usize,
}

impl Test {
    pub fn execute(&self, value: usize) -> usize {
        if value % self.divisible_by == 0 {
            self.yes
        } else {
            self.no
        }
    }
}

fn main() {
    let mut monkeys = parser::input::<String>()
        .split("\n\n")
        .map(|monkey| monkey.trim().parse::<Monkey>().unwrap())
        .map(|monkey| (monkey.id, monkey))
        .collect::<HashMap<_, _>>();

    let common_factor = monkeys
        .values()
        .map(|monkey| monkey.test.divisible_by)
        .reduce(|a, b| a * b)
        .unwrap();

    let mut ids = monkeys.keys().copied().collect::<Vec<_>>();
    ids.sort();
    let ids = ids;

    let mut activity = ids
        .iter()
        .map(|&id| (id, 0))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..10000 {
        for id in &ids {
            // we remove the entry to avoid keeping a write ref on the map
            let mut monkey = monkeys.remove(&id).unwrap();
            *activity.get_mut(&monkey.id).unwrap() += monkey.items.0.len();

            while let Some(item) = monkey.items.0.pop() {
                let item = monkey.operation.apply(item);
                let item = item % common_factor;
                let throws_to = monkey.test.execute(item);

                monkeys.get_mut(&throws_to).unwrap().items.0.push(item);
            }

            monkeys.insert(monkey.id, monkey);
        }
    }

    let mut activity = activity
        .iter()
        .map(|(_id, activity)| activity)
        .collect::<Vec<_>>();
    activity.sort();
    let max = activity.pop().unwrap();
    let second = activity.pop().unwrap();

    println!("{}", max * second);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_starting_items() {
        let items = "79, 98".parse::<Items>().unwrap();
        insta::assert_debug_snapshot!(items, @r###"
        Items(
            [
                79,
                98,
            ],
        )
        "###);
    }

    #[test]
    fn test_operation() {
        let operation = "new = old * 19".parse::<Operation>().unwrap();
        insta::assert_debug_snapshot!(operation, @r###"
        Operation {
            left: "old",
            op: Mul,
            right: "19",
        }
        "###);
    }

    #[test]
    fn test_test() {
        let test = "divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            .parse::<Test>()
            .unwrap();
        insta::assert_debug_snapshot!(test, @r###"
        Test {
            divisible_by: 23,
            yes: 2,
            no: 3,
        }
        "###);
    }

    #[test]
    fn test_monkey() {
        let monkey = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            .parse::<Monkey>()
            .unwrap();
        insta::assert_debug_snapshot!(monkey, @"");
    }
}
