use regex::Regex;

fn parse_input_a(input: &str) -> Vec<(i64, i64)> {
    let input = input.lines();
    let lines = input.map(|line| {
        let re = Regex::new(r"[ ]+").unwrap();
        let line = re.replace_all(line, " ");
        line.split_once(" ")
            .unwrap()
            .1
            .split(" ")
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    });

    let result = lines.collect::<Vec<_>>();
    result[0]
        .clone()
        .into_iter()
        .zip(result[1].clone().into_iter())
        .collect()
}

pub fn run_a(input: &str) {
    let input = parse_input_a(input);
    let result: i64 = input
        .iter()
        .map(|(time, goal)| (0..*time).filter(|i| i * (time - i) > *goal).count() as i64)
        .product();
    println!("Day 06a: {}", result);
}

fn parse_input_b(input: &str) -> (i64, i64) {
    let input = input.lines();
    let mut result = input.map(|line| {
        let re = Regex::new(r"[ ]+").unwrap();
        let line = re.replace_all(line, " ");
        line.split_once(" ")
            .unwrap()
            .1
            .replace(" ", "")
            .parse::<i64>()
            .unwrap()
    });
    (result.next().unwrap(), result.next().unwrap())
}
pub fn run_b(input: &str) {
    let (time, goal) = parse_input_b(input);
    let result = (0..time).filter(|i| i * (time - i) > goal).count() as i64;
    println!("Day 06b: {}", result);
}

pub fn run() {
    let challenge_input = include_str!("../inputs/day_06/challenge.txt");
    let _test_input = include_str!("../inputs/day_06/test.txt");
    run_a(challenge_input);
    run_b(challenge_input);
}
