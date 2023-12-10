use std::{env, time::Instant};

use advent_of_code_2022_rust::*;

fn main() {
    let functions = [
      day_01, 
      day_02,
      day_03,
      day_04,
      day_05,
      day_06,
      day_07,
      day_08,
      day_09,
    ];
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(day) => {
            let now = Instant::now();
            functions.get(day.parse::<usize>().unwrap() - 1).unwrap()();
            println!("Day {} took {:?}ms", day, now.elapsed().as_millis());
        }
        None => {
            functions.iter().enumerate().for_each(|(i, f)| {
                let now = Instant::now();
                f();
                println!("Day {} took {:?}ms", i + 1, now.elapsed().as_millis());
            });
        }
    };
}
