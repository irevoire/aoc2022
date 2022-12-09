use day7::*;

use aoc::answer;

fn main() {
    let fs = Graph::parse();

    let mut total_size = 0;

    for node in fs.traverse() {
        let size = node.size();

        if size < 100000 {
            total_size += node.size();
        }
    }

    answer!(
        "The sum of the total sizes of those directories is {}.",
        total_size
    );
}
