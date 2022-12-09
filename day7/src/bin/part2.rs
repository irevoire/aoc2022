use day7::*;

use aoc::*;

fn main() {
    let fs = Graph::parse();

    let disk_size = 70000000;
    let used_size = fs.size();
    let available_space = disk_size - used_size;

    let targeted_space = 30000000;

    let size = fs
        .traverse()
        .into_iter()
        .map(|dir| dir.size())
        .filter(|size| available_space + size > targeted_space)
        .sorted()
        .next()
        .unwrap();

    answer!(
        "The smallest directory that, if deleted, would free up enough space on the filesystem to run the update weight {} bytes.",
        size
    );
}
