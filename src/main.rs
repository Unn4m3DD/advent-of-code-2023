use std::env;

use advent_of_code_2022_rust::*;

fn main() {
    let functions = [
      day_01,
      day_02,
    ];
    let args: Vec<String> = env::args().collect();
    functions
        .get(args[1].parse::<usize>().unwrap() - 1)
        .unwrap()();
}
