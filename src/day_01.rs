use std::collections::HashMap;

use regex::Regex;

pub fn run_a(input: &str) {
    let re = Regex::new(r".*?(?<c1>[0-9])(.*(?<c2>[0-9]).*?)?").unwrap();
    let result: i64 = input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|e| {
            let c1 = &e["c1"];
            let c2 = e.name("c2").map_or(c1, |c2| c2.as_str());
            format!("{}{}", c1, c2)
        })
        .map(|e| e.parse::<i64>().unwrap())
        .sum();
    println!("Day 01a: {}", result);
}

pub fn run_b(input: &str) {
    let mapper = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let re = Regex::new(r".*?(?<c1>(([0-9]|one|two|three|four|five|six|seven|eight|nine)))(.*((?<c2>[0-9]|one|two|three|four|five|six|seven|eight|nine)).*?)?").unwrap();
    let result: i64 = input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|e| {
            let c1 = &e["c1"];
            let c1 = mapper.get(c1).unwrap_or(&c1);
            let c2 = e.name("c2").map_or(*c1, |c2| c2.as_str());
            let c2 = mapper.get(c2).unwrap_or(&c2);
            format!("{}{}", c1, c2)
        })
        .map(|e| e.parse::<i64>().unwrap())
        .sum();
    println!("Day 01b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_01/challenge.txt");
    let _test_input_a = include_str!("../inputs/day_01/test_a.txt");
    let _test_input_b = include_str!("../inputs/day_01/test_b.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
