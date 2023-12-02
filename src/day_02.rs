use std::i64;

use regex::Regex;

#[derive(Debug)]
struct Game {
    red: i64,
    green: i64,
    blue: i64,
}

impl Game {
    fn from(input: &str) -> Self {
        let blue = Regex::new(r"(?<blue>[0-9]+) blue")
            .unwrap()
            .captures(input)
            .map_or(0, |name| {
                name.name("blue").unwrap().as_str().parse::<i64>().unwrap()
            });
        let red = Regex::new(r"(?<red>[0-9]+) red")
            .unwrap()
            .captures(input)
            .map_or(0, |name| {
                name.name("red").unwrap().as_str().parse::<i64>().unwrap()
            });
        let green = Regex::new(r"(?<green>[0-9]+) green")
            .unwrap()
            .captures(input)
            .map_or(0, |name| {
                name.name("green").unwrap().as_str().parse::<i64>().unwrap()
            });
        Game { red, green, blue }
    }
}

pub fn run_a(input: &str) {
    let total = Game::from("12 red cubes, 13 green cubes, and 14 blue cubes");
    let result = input
        .lines()
        .map(|line| line.split(";"))
        .map(|line| line.map(|e| Game::from(e)))
        .enumerate()
        .map(|(index, item)| {
            if item.fold(true, |acc, draw| {
                acc && draw.red <= total.red && draw.green <= total.green && draw.blue <= total.blue
            }) {
                (index + 1) as i64
            } else {
                0 as i64
            }
        })
        .sum::<i64>();
    println!("Day 02a: {}", result);
}
pub fn run_b(input: &str) {
    let result = input
        .lines()
        .map(|line| line.split(";"))
        .map(|line| line.map(|e| Game::from(e)))
        .map(|item| {
            let min_combo = item.fold(
                Game {
                    red: 0,
                    blue: 0,
                    green: 0,
                },
                |acc, draw| Game {
                    red: draw.red.max(acc.red),
                    green: draw.green.max(acc.green),
                    blue: draw.blue.max(acc.blue),
                },
            );
            min_combo.red * min_combo.green * min_combo.blue
        })
        .sum::<i64>();
    println!("Day 02b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_02/challenge.txt");
    let _test_input = include_str!("../inputs/day_02/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
